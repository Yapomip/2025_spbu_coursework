use std::io::BufRead;
use std::path::Path;

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
}

#[derive(Debug, Clone, Default)]
pub struct TestDataset {
    data: Vec<TestDataItem>,
    pub mean: TestDataItem,
    pub std: TestDataItem,
}

impl TestDataset {
    pub fn new() -> Self {
        Self::load_from("./out2/all.csv").unwrap()
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
    pub fn load_from(path: &str) -> Result<Self, std::io::Error> {
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
        std.t = f64::sqrt(1.0 / (data.len() - 1) as f64 * std.t);
        std.atom_n = f64::sqrt(1.0 / (data.len() - 1) as f64 * std.atom_n);
        std.pressure = f64::sqrt(1.0 / (data.len() - 1) as f64 * std.pressure);
        std.shear_viscosity = f64::sqrt(1.0 / (data.len() - 1) as f64 * std.shear_viscosity);
        std.thermal_conductivity = f64::sqrt(1.0 / (data.len() - 1) as f64 * std.thermal_conductivity);
        std.bulk_viscosity = f64::sqrt(1.0 / (data.len() - 1) as f64 * std.bulk_viscosity);
        std.n.iter_mut().for_each(|item| *item = (1.0 / (data.len() - 1) as f64 * (*item)).sqrt());
        // for j in 0..std.n.len() {
        //     std.n[j] = f64::sqrt(1.0 / (data.len() - 1) as f64 * std.n[j]);
        // }
        println!("create test dataset, size {}\n\n\n\n{:?}\n\n\n\n{:?}", data.len(), mean, std);

        for i in 0..data.len() {
            data[i].normilize(&mean, &std);
            println!("\n{:?}", data[i]);
        }

        let dataset = Self { data, mean, std };
        
        Ok(dataset)
    }
    
}

fn main() {
    let _ = TestDataset::new();
    // println!("{:?}", TestDataset::new());
}