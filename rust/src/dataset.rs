use std::{clone, default, io::BufRead, iter, num::ParseFloatError, path::Path, vec, };
use std::process::Command;
use rand::Rng;
use serde::{Deserialize, Serialize};
use burn::{
    backend, data::{dataloader::{batcher::Batcher, split}, dataset::{Dataset, InMemDataset, SqliteDataset, HuggingfaceDatasetLoader}}, prelude::*
};

#[derive(Debug, Clone, Default)]
pub struct TestDataItem {
    pub t: f64,
    pub pressure: f64,
    pub atom_n: f64,

    pub n: Vec<f64>,

    pub thermal_conductivity: f64,
    pub shear_viscosity: f64,
    pub bulk_viscosity: f64,
}

impl TestDataItem {
    fn to_pair<B: Backend>(mut self, device: &B::Device) -> (Tensor<B, 2>, Tensor<B, 2>) {
        let a_len = 3 + self.n.len();
        let mut a = Vec::with_capacity(a_len);
        
        a.push(self.t);
        a.push(self.atom_n);
        a.push(self.pressure);
        a.append(&mut self.n);

        let input_tensor_data = TensorData::new(a, [1, a_len]);
        let input_tensor = Tensor::<B, 2>::from_floats(input_tensor_data, &device);
        let target_tensor = Tensor::<B, 2>::from_floats([[self.thermal_conductivity, self.shear_viscosity, self.bulk_viscosity]], &device);
        (input_tensor, target_tensor)
    }
    fn to_tensor<B: Backend>(mut self, device: &B::Device) -> Tensor<B, 2> {
        let a_len = 3 + self.n.len();
        let mut a = Vec::with_capacity(a_len);
        
        a.push(self.t);
        a.push(self.atom_n);
        a.push(self.pressure);
        a.append(&mut self.n);
        a.push(self.thermal_conductivity);
        a.push(self.shear_viscosity);
        a.push(self.bulk_viscosity);
        
        let res_tensor_data = TensorData::new(a, [1, a_len]);
        let res_tensor = Tensor::<B, 2>::from_floats(res_tensor_data, &device);
        res_tensor
    }
    fn normilize(&mut self, mean_element: &Self, std_element: &Self) {
        self.t = (self.t - mean_element.t) / std_element.t;
        self.pressure = (self.pressure - mean_element.pressure) / std_element.pressure;
        self.atom_n = (self.atom_n - mean_element.atom_n) / std_element.atom_n;
        self.thermal_conductivity = (self.thermal_conductivity - mean_element.thermal_conductivity) / std_element.thermal_conductivity;
        self.shear_viscosity = (self.shear_viscosity - mean_element.shear_viscosity) / std_element.shear_viscosity;
        self.bulk_viscosity = (self.bulk_viscosity - mean_element.bulk_viscosity) / std_element.bulk_viscosity;
        self.n.iter_mut()
            .zip(mean_element.n.iter())
            .zip(std_element.n.iter())
            .for_each(|((item, mean), std)| *item = ((*item) - mean) / std);
    }
    fn to_normilize_pair<B: Backend>(mut self, mean_element: &Self, std_element: &Self, device: &B::Device) -> (Tensor<B, 2>, Tensor<B, 2>) {
        self.normilize(mean_element, std_element);
        self.to_pair(device)
    }
}

#[derive(Clone)]
pub struct TestDataset<B: Backend> {
    data: Vec<TestDataItem>,
    data_in_tensor: Vec<(Tensor<B, 2>, Tensor<B, 2>)>,
    pub mean: TestDataItem,
    pub std: TestDataItem,
}

impl<B: Backend> TestDataset<B> {
    pub fn new(device: &B::Device) -> Self {
        Self::load_from("./../out2/all.csv", device).unwrap()
    }
    fn read_from_csv<P: AsRef<Path>>(path: P) -> Result<Vec<TestDataItem>, std::io::Error> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);

        let mut res = Vec::new();
        
        let mut a = reader.lines();
        let b = a.next();
        res = a.map(|line| -> Result<TestDataItem, std::io::Error> {
            let line = line?;
            let f = line.split(';').map(|item| item.parse::<f64>().unwrap()).collect::<Vec<_>>();
            
            Ok(TestDataItem {
                t: f[0],
                pressure: f[1],
                atom_n: f[2],
                thermal_conductivity: f[f.len() - 3],
                shear_viscosity: f[f.len() - 2],
                bulk_viscosity: f[f.len() - 1],
                n: f[3..f.len() - 3].to_vec(),
            })
        }).collect::<Result<Vec<_>, _>>()?;

        Ok(res)
    }
    pub fn load_from(path: &str, device: &B::Device) -> Result<Self, std::io::Error> {
        // Build dataset from csv with tab ('\t') delimiter
        let mut data = Self::read_from_csv(path).unwrap();

        let mut mean = TestDataItem::default();
        mean.n = vec![0.0; data[0].n.len()];
        for i in 0..data.len() {
            mean.t += data[i].t;
            mean.atom_n += data[i].atom_n;
            mean.pressure += data[i].pressure;
            mean.shear_viscosity += data[i].shear_viscosity;
            mean.thermal_conductivity += data[i].thermal_conductivity;
            mean.bulk_viscosity += data[i].bulk_viscosity;
            for j in 0..data[i].n.len() {
                mean.n[j] += data[i].n[j];
            }
        }
        mean.t /= data.len() as f64;
        mean.atom_n /= data.len() as f64;
        mean.pressure /= data.len() as f64;
        mean.shear_viscosity /= data.len() as f64;
        mean.thermal_conductivity /= data.len() as f64;
        mean.bulk_viscosity /= data.len() as f64;
        mean.n.iter_mut().for_each(|item| *item /= data.len() as f64);

        let mut std = TestDataItem::default();
        std.n = vec![0.0; data[0].n.len()];
        for i in 0..data.len() {
            std.t += (data[i].t - mean.t).powi(2);
            std.atom_n += (data[i].atom_n - mean.atom_n).powi(2);
            std.pressure += (data[i].pressure - mean.pressure).powi(2);
            std.shear_viscosity += (data[i].shear_viscosity - mean.shear_viscosity).powi(2);
            std.thermal_conductivity += (data[i].thermal_conductivity - mean.thermal_conductivity).powi(2);
            std.bulk_viscosity += (data[i].bulk_viscosity - mean.bulk_viscosity).powi(2);
            for j in 0..data[i].n.len() {
                std.n[j] += (data[i].n[j] - mean.n[j]).powi(2);
            }
        }
        std.t = f64::sqrt(std.t / (data.len() - 1) as f64);
        std.atom_n = f64::sqrt(std.atom_n / (data.len() - 1) as f64);
        std.pressure = f64::sqrt(std.pressure / (data.len() - 1) as f64);
        std.shear_viscosity = f64::sqrt(std.shear_viscosity / (data.len() - 1) as f64);
        std.thermal_conductivity = f64::sqrt(std.thermal_conductivity / (data.len() - 1) as f64);
        std.bulk_viscosity = f64::sqrt(std.bulk_viscosity / (data.len() - 1) as f64);
        std.n.iter_mut().for_each(|item| *item = ((*item) / (data.len() - 1) as f64).sqrt());
        
        // println!("create test dataset, size {}\n{:?}\n{:?}", data.len(), mean, std);
        
        for i in 0..data.len() {
            data[i].normilize(&mean, &std);
        }
        
        let data_in_tensor = data.iter()
            .map(|item| item.clone().to_pair(device))
            .collect::<Vec<_>>();
        // let data_in_tensor = Tensor::cat(data_in_tensor, 0);

        let dataset = Self { data, data_in_tensor, mean, std };

        Ok(dataset)
    }
    
    pub fn shufle_n(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            let i1 = rng.r#gen::<usize>() % self.len();
            let i2 = rng.r#gen::<usize>() % self.len();
            self.data.swap(i1, i2);
        }
    }
    pub fn shufle(&mut self) {
        self.shufle_n(self.len())
    }
    pub fn split_by_index(mut self, index: usize) -> (Self, Self) {
        let other = Self { 
            data: self.data.split_off(index), 
            data_in_tensor: self.data_in_tensor.split_off(index), 
            mean: self.mean.clone(), 
            std: self.std.clone() 
        };
        (self, other)
    }
    pub fn split_by_procent(self, p: f64) -> (Self, Self) {
        if p < 0.0 || p > 1.0 {
            panic!()
        }
        let index = (self.len() as f64 * p) as usize;
        self.split_by_index(index)
    }
}

// Implement the `Dataset` trait which requires `get` and `len`
impl<B: Backend> Dataset<(Tensor<B, 2>, Tensor<B, 2>)> for TestDataset<B> {
    fn get(&self, index: usize) -> Option<(Tensor<B, 2>, Tensor<B, 2>)> {
        self.data_in_tensor.get(index).map(|x| x.clone())
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

#[derive(Default, Clone, Debug)]
pub struct TestBatcher {
    pub mean: TestDataItem,
    pub std: TestDataItem,
}

#[derive(Clone, Debug)]
pub struct TestBatch<B: Backend> {
    pub input: Tensor<B, 2>,
    pub targets: Tensor<B, 2>,
}

impl<B: Backend> Batcher<B, (Tensor<B, 2>, Tensor<B, 2>), TestBatch<B>> for TestBatcher {
    fn batch(&self, items: Vec<(Tensor<B, 2>, Tensor<B, 2>)>, device: &B::Device) -> TestBatch<B> {
        // let items = items.into_iter()
        //     .map(|item| item.to_pair(device))
        //     .collect::<(Vec<_>, Vec<_>)>();
        // let input = Tensor::cat(items.0, 0);
        // let targets = Tensor::cat(items.1, 0);
        // // println!("batcher call: {}", input.dims()[0]);
        let items = items.into_iter().collect::<(Vec<_>, Vec<_>)>();
        let input = Tensor::cat(items.0, 0);
        let targets = Tensor::cat(items.1, 0);
        TestBatch { input, targets }
    }
}
