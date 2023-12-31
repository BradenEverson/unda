use serde::{Serialize, Deserialize};

use crate::network::{matrix::Matrix,activations::Activations, input::Input, matrix3d::Matrix3D};

use super::layers::Layer;



#[derive(Serialize, Deserialize)]
pub struct Convolutional{
    filter_weights: Matrix3D,
    filter_biases: Vec<f32>,
    data: Matrix3D,
    stride: usize,
    filters: usize,
    shape: (usize, usize),
    input_shape: (usize, usize, usize),
    output_shape: (usize, usize, usize),
    loss: f32,

    activation_fn: Activations,
    learning_rate: f32
}

impl Convolutional{
    pub fn new(filters: usize, kernel_size: (usize, usize), input_shape: (usize, usize, usize), stride: usize, activation_fn: Activations, learning_rate: f32) -> Convolutional {
        let mut res = Convolutional{
            filter_weights: Matrix3D::new_random(kernel_size.0, kernel_size.1, filters),
            filter_biases: vec![0.0; filters],
            data: Matrix3D::new_empty(0,0,0),
            stride,
            filters,
            input_shape,
            shape: kernel_size,
            activation_fn,
            learning_rate,
            output_shape: (0,0,0),
            loss: 1.0
        };
        let res_len = Convolutional::get_res_size(input_shape.0, kernel_size.0, 0, stride);
        let res_width = Convolutional::get_res_size(input_shape.1, kernel_size.1, 0, stride);

        res.data = Matrix3D::new_empty(res_len, res_width, filters);
        res.output_shape = (res_len, res_width, 1);

        res
    }
    pub fn convolute(&mut self, idx: usize, input: Matrix) -> Matrix{
        let kernel = self.filter_weights.get_slice(idx);
        let mut output = Matrix::new_empty(self.output_shape.0, self.output_shape.1);
        //slide kernel over input, summing kernel and returning resultant matrix
        
        output
    }
    fn get_res_size(w: usize, k: usize, p: usize, s:usize) -> usize {
        (w - k + 2*p) / s + 1
    }
}

/*#[typetag::serde]
impl Layer for Convolutional {
    fn forward(&mut self,inputs: &Box<dyn Input>) -> Box<dyn Input> {
        
    }
    fn backward(&mut self,parsed:Box<dyn Input> ,errors:Box<dyn Input> ,data:Box<dyn Input>) -> Box<dyn Input> {
        
    }
    fn get_data(&self) -> Box<dyn Input> {
        self.data.to_box()
    }
    fn shape(&self) -> (usize,usize,usize) {
        self.input_shape
    }
    fn get_loss(&self) -> f32 {
        self.loss
    }
    fn update_gradient(&self) -> Box<dyn Input> {
        //TODO
        self.data.to_box()
    }
}*/
