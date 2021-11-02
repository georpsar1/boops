#[cfg(test)]
mod tests {
}

pub struct Network {
    layers: Vec<Layer>,
}

struct Layer {
    neurons: Vec<Neuron>,
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>
}

impl Network {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {

        let mut inputs = inputs;

        for layer in &self.layers {
            inputs = layer.propagate(inputs)
        }
        inputs
    }
}

impl Layer {
    fn propagate(&self,inputs: Vec<f32>) -> Vec<f32> {
        todo!()

    }
}


impl Neuron {
    fn propagate(&self,inputs: Vec<f32>) -> f32 {

        let mut activation: f32;

        for input in inputs {
            activation += input;
        }


        
    }
}
