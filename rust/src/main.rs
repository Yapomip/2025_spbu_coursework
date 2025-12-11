// #![recursion_limit = "131"]
// use burn::{
//     backend::{Autodiff, WebGpu, Wgpu, }, config::Config, data::dataset::Dataset, optim::AdamConfig
// };
#![recursion_limit = "256"]

use burn::{backend::Autodiff, tensor::backend::Backend, optim::AdamConfig, data::dataset::Dataset};
// #[cfg(feature = "wgpu")]
// mod wgpu {
//     use burn_autodiff::Autodiff;
//     use burn_wgpu::{Wgpu, WgpuDevice};
//     use mnist::training;
// }
use burn::prelude::Config;

mod data;
mod inference;
mod model;
mod training;
mod dataset;

use dataset::TestDataset;
use model::ModelConfig;
use training::{TrainingConfig};

static ARTIFACT_DIR: &str = "./tmp/";

#[cfg(any(
    feature = "ndarray",
    feature = "ndarray-blas-netlib",
    feature = "ndarray-blas-openblas",
    feature = "ndarray-blas-accelerate",
))]
mod ndarray {
    use burn::backend::ndarray::{NdArray, NdArrayDevice};

    pub fn run() {
        let device = NdArrayDevice::Cpu;
        super::run::<NdArray>(device.clone());
    }
}

#[cfg(feature = "tch-gpu")]
mod tch_gpu {
    use burn::backend::libtorch::{LibTorch, LibTorchDevice};

    pub fn run() {
        println!("Start!");
        #[cfg(not(target_os = "macos"))]
        let device = LibTorchDevice::Vulkan;
        #[cfg(target_os = "macos")]
        let device = LibTorchDevice::Mps;

        super::run::<LibTorch>(device);
    }
}

#[cfg(feature = "wgpu")]
mod wgpu {
    use burn::backend::wgpu::{Wgpu, WgpuDevice};

    pub fn run() {
        let device = WgpuDevice::default();
        super::run::<Wgpu>(device);
    }
}

#[cfg(feature = "tch-cpu")]
mod tch_cpu {
    use burn::backend::libtorch::{LibTorch, LibTorchDevice};
    use simple_regression::training;
    pub fn run() {
        let device = LibTorchDevice::Cpu;
        super::run::<LibTorch>(device);
    }
}

#[cfg(feature = "vulkan")]
mod vulkan {
    use burn::backend::{Vulkan};

    pub fn run() {
        let device = Default::default();
        super::run::<Vulkan>(device);
    }
}
#[cfg(feature = "rocm")]
mod rocm {
    use burn::backend::{Rocm, rocm::RocmDevice};

    pub fn run() {
        let device = RocmDevice::new(0);
        super::run::<Rocm>(device);
    }
}

/// Train a regression model and predict results on a number of samples.
pub fn run<B: Backend>(device: B::Device) {
    let config = TrainingConfig::load("./config.json").unwrap_or(TrainingConfig::new(ModelConfig::new(), AdamConfig::new()));
    
    println!("Backend:\n{device:?}");
    println!("{config}");

    training::train::<Autodiff<B>>(
        ARTIFACT_DIR,
        config.clone(),
        device.clone(),
    );

    let _ = config.save("./config.json").inspect_err(|e| println!("{e}") );
    
    let mut dataset = TestDataset::new();
    dataset.shufle();
    let items = dataset.iter().take(5).collect::<Vec<_>>();
    inference::infer::<B>(
        ARTIFACT_DIR,
        device,
        items
    );
}


fn main() {
    #[cfg(feature = "vulkan")]
    vulkan::run();
    #[cfg(feature = "rocm")]
    rocm::run();
    #[cfg(feature = "tch-gpu")]
    tch_gpu::run();
    #[cfg(any(
        feature = "ndarray",
        feature = "ndarray-blas-netlib",
        feature = "ndarray-blas-openblas",
        feature = "ndarray-blas-accelerate",
    ))]
    ndarray::run();
    #[cfg(feature = "wgpu")]
    wgpu::run();
    #[cfg(feature = "tch-cpu")]
    tch_cpu::run();
    #[cfg(feature = "remote")]
    remote::run();
}
