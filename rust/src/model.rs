use burn::{
    nn::{
        Dropout, DropoutConfig, Linear, LinearConfig, Relu,
        conv::{Conv2d, Conv2dConfig},
        pool::{AdaptiveAvgPool2d, AdaptiveAvgPool2dConfig},
    },
    prelude::*,
};

#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    activation: Relu,
    input: Linear<B>,
    linear1: Linear<B>,
    linear2: Linear<B>,
    linear3: Linear<B>,
    // linear4: Linear<B>,
    output: Linear<B>,
}

#[derive(Config, Debug)]
pub struct ModelConfig {
    
    #[config(default = "51")]
    input_size: usize,
    #[config(default = "100")]
    hidden_size: usize,
}

impl ModelConfig {
    /// Returns the initialized model.
    pub fn init<B: Backend>(&self, device: &B::Device) -> Model<B> {
        Model {
            activation: Relu::new(),
            input: LinearConfig::new(self.input_size, self.hidden_size).with_bias(true).init(device),
            linear1: LinearConfig::new(self.hidden_size, self.hidden_size).with_bias(true).init(device),
            linear2: LinearConfig::new(self.hidden_size, self.hidden_size).with_bias(true).init(device),
            linear3: LinearConfig::new(self.hidden_size, self.hidden_size).with_bias(true).init(device),
            // linear4: LinearConfig::new(self.hidden_size, self.hidden_size).with_bias(true).init(device),
            output: LinearConfig::new(self.hidden_size, 3).with_bias(true).init(device),
        }
    }
}

impl<B: Backend> Model<B> {
    /// # Shapes
    ///   - Images [batch_size, height, width]
    ///   - Output [batch_size, class_prob]
    pub fn forward(&self, x: Tensor<B, 2>) -> Tensor<B, 2> {
        let x = self.input.forward(x);
        let x = self.activation.forward(x);

        let x = self.linear1.forward(x);
        let x = self.activation.forward(x);

        let x = self.linear2.forward(x);
        let x = self.activation.forward(x);

        let x = self.linear3.forward(x);
        let x = self.activation.forward(x);

        // let x = self.linear4.forward(x);
        // let x = self.activation.forward(x);

        let x = self.output.forward(x);
        x
    }
}