use crate::{dataset::{TestBatcher, TestDataItem, TestDataset}, training::TrainingConfig};
use burn::{
    data::{dataloader::batcher::Batcher, dataset::Dataset}, nn::loss::MseLoss, prelude::*, record::{CompactRecorder, Recorder}
};

pub fn infer<B: Backend>(artifact_dir: &str, device: B::Device, items: Vec<TestDataItem>) {
    let config = TrainingConfig::load(format!("{artifact_dir}/config.json"))
        .expect("Config should exist for the model; run train first");
    let record = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist; run train first");

    let model = config.model.init::<B>(&device).load_record(record);

    // let dataset = TestDataset::new();
    // let target = dataset.get(200).unwrap();
    let batcher = TestBatcher::default();
    let batch = batcher.batch(items, &device);
    // let output = model.forward(batch.input.clone());
    // let loss = MseLoss::new().forward(output.clone(), batch.targets.clone(), nn::loss::Reduction::Auto);

    let reg_out = model.forward_no_reduction_step(batch);
    let aloss = reg_out.0.clone() - reg_out.1.clone();
    println!("Predicted:\n{}\n\nExpected:\n{}\n\nLoss:\n{}\n\nALoss:\n{}", reg_out.0, reg_out.1, reg_out.2, aloss);
    
    // println!("Predicted {:}\nExpected {:?}\nLoss {:}\n{:?}", output, target, loss, loss.to_data().to_vec::<f32>().unwrap());
}
