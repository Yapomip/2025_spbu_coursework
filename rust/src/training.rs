use std::sync::Arc;

use crate::{
    data::{MnistBatch, MnistBatcher},
    dataset::{TestBatch, TestDataset, TestBatcher},
    model::{Model, ModelConfig},
};
use burn::{
    data::{dataloader::{DataLoaderBuilder, DataLoader}, dataset::vision::MnistDataset, dataset::Dataset},
    nn::loss::{MseLoss, Reduction},
    optim::{AdamConfig, GradientsParams, Optimizer},
    prelude::*,
    record::CompactRecorder,
    tensor::backend::AutodiffBackend,
    train::{
        RegressionOutput, LearnerBuilder, LearningStrategy, TrainOutput, TrainStep, ValidStep,
        metric::{LossMetric, CpuMemory, CpuUse, CudaMetric},
    },
    train::renderer::{MetricsRenderer, MetricsRendererEvaluation, MetricsRendererTraining},
};
use serde::Deserialize;
use std::time::{Instant, Duration};

impl<B: Backend> Model<B> {
    pub fn forward_step(
        &self,
        item: TestBatch<B>,
    ) -> RegressionOutput<B> {
        let output = self.forward(item.input);
        let targets = item.targets;
        let loss = MseLoss::new().forward(output.clone(), targets.clone(), Reduction::Sum);
        RegressionOutput::new(loss, output, targets)
    }
    /// output tagrets loss
    pub fn forward_no_reduction_step(
        &self,
        item: TestBatch<B>,
    ) -> (Tensor<B, 2>, Tensor<B, 2>, Tensor<B, 2>) {
        let output = self.forward(item.input);
        let targets = item.targets;
        let loss = MseLoss::new().forward_no_reduction(output.clone(), targets.clone());
        (output, targets, loss)
    }
}

impl<B: AutodiffBackend> TrainStep<TestBatch<B>, RegressionOutput<B>> for Model<B> {
    fn step(&self, batch: TestBatch<B>) -> TrainOutput<RegressionOutput<B>> {
        let item: RegressionOutput<B> = self.forward_step(batch);

        TrainOutput::new(self, item.loss.backward(), item)
    }
}

impl<B: Backend> ValidStep<TestBatch<B>, RegressionOutput<B>> for Model<B> {
    fn step(&self, batch: TestBatch<B>) -> RegressionOutput<B> {
        self.forward_step(batch)
    }
}

#[derive(Debug, Clone, Default)]
struct MyRenderer;

impl MetricsRendererEvaluation for MyRenderer {
    fn render_test(&mut self, item: burn::train::renderer::EvaluationProgress) {
        print!("{}: ", item.progress.items_processed as f64 / item.progress.items_total as f64);
    }
    fn update_test(&mut self, name: burn::train::renderer::EvaluationName, state: burn::train::renderer::MetricState) {}
}
impl MetricsRendererTraining for MyRenderer {
    fn render_train(&mut self, item: burn::train::renderer::TrainingProgress) {
        println!("train {item:?}");
    }
    fn render_valid(&mut self, item: burn::train::renderer::TrainingProgress) {
        println!("valid {item:?}");
    }
    fn update_train(&mut self, state: burn::train::renderer::MetricState) {
        // match state => {
        //     burn::train::renderer::MetricState::
        // }
        // println!("{}", );
    }
    fn update_valid(&mut self, state: burn::train::renderer::MetricState) {}
}
impl MetricsRenderer for MyRenderer {
    fn manual_close(&mut self) {}
}

#[derive(Config, Debug)]
pub struct TrainingConfig {
    pub model: ModelConfig,
    pub optimizer: AdamConfig,
    #[config(default = 600)]
    pub num_epochs: usize,
    #[config(default = 512)]
    pub batch_size: usize,
    #[config(default = 8)]
    pub num_workers: usize,
    #[config(default = 1232)]
    pub seed: u64,
    #[config(default = 5.0e-5)]
    pub learning_rate: f64,
    #[config(default = 0.85)]
    pub train_procent: f64,
}

fn create_artifact_dir(artifact_dir: &str) {
    // Remove existing artifacts before to get an accurate learner summary
    std::fs::remove_dir_all(artifact_dir).ok();
    std::fs::create_dir_all(artifact_dir).ok();
}

pub fn train<B: AutodiffBackend>(artifact_dir: &str, config: TrainingConfig, device: B::Device) {
    create_artifact_dir(artifact_dir);
    config
        .save(format!("{artifact_dir}/config.json"))
        .expect("Config should be saved successfully");

    B::seed(&device, config.seed);


    let mut all_data_set = TestDataset::new();
    all_data_set.shufle();
    all_data_set.shufle();
    all_data_set.shufle();
    
    all_data_set.iter().take(5).for_each(|item| { println!("{item:?}"); });

    let batcher = TestBatcher {mean: all_data_set.mean.clone(), std: all_data_set.std.clone()};
    let (train, test) = all_data_set.split_by_procent(config.train_procent);

    let dataloader_train: Arc<dyn DataLoader<B, TestBatch<B>>> = DataLoaderBuilder::new(batcher.clone())
        .batch_size(config.batch_size)
        // .shuffle(config.seed)
        .num_workers(config.num_workers)
        .build(train);

    let dataloader_test: Arc<dyn DataLoader<B, TestBatch<B>>> = DataLoaderBuilder::new(batcher)
        .batch_size(config.batch_size)
        .shuffle(config.seed)
        .num_workers(config.num_workers)
        .build(test);

    let mut model = config.model.init::<B>(&device);
    let mut optimizer = config.optimizer.init::<B, Model<B>>();
    let start = Instant::now();
    let mut iter_time_all = 0.0;
    let mut calc_time_all = 0.0;
    

    for epoch in 0..config.num_epochs {
        let iter_time = Instant::now();
        for (iteration, batch) in dataloader_train.iter().enumerate() {
            
            iter_time_all += iter_time.elapsed().as_secs_f64();
            
            let calc_time = Instant::now();

            let output = model.forward(batch.input.clone());
            let mse_loss = MseLoss::new().forward(output.clone(), batch.targets.clone(), Reduction::Auto);
            let mae_loss = output - batch.targets;
            
            // Gradients for the current backward pass
            let grads = mse_loss.backward();
            // Gradients linked to each parameter of the model.
            let grads = GradientsParams::from_grads(grads, &model);
            // Update the model using the optimizer.
            model = optimizer.step(config.learning_rate, model, grads);
            
            calc_time_all += calc_time.elapsed().as_secs_f64();

            println!(
                "[Epoch {} - Iteration {}] MSE {} | MAE {}",
                epoch,
                iteration,
                mse_loss.clone().into_scalar(),
                mae_loss.max().into_scalar(),
            );
        }
    }

    let duration = start.elapsed();

    println!("TIME(s): {} {} {}", duration.as_secs_f64(), iter_time_all, calc_time_all);
    
    let _ = std::fs::write("./duration.txt", format!("{:?}\n{}\n{}", duration, duration.as_secs_f64(), duration.as_millis())).inspect_err(|e| println!("error write duration {e}"));
    
    model
        .save_file(format!("{artifact_dir}/model"), &CompactRecorder::new())
        .expect("Trained model should be saved successfully");
}
