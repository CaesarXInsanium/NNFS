pub use std::iter::Zip;
pub use std::ops::{Add,Sub,Div, Mul};
pub use std::any::Any;
pub use num_traits::cast::AsPrimitive;

pub mod chap1{
    pub struct Neuron{
        weights: Vec<f64>,
        bias: f64
    }


    impl Neuron {

        //associate function, like static function
        
        pub fn build_neuron_slice<T, U>(_weights:&[T], _bias:&U) -> Neuron
        where T: std::convert::Into<f64> + Clone,
            U: Clone + std::convert::Into<f64>
            {
            let mut w: Vec<f64> = Vec::new();
            for ww in  _weights{
                w.push(ww.clone().into());
            }
            Neuron{
                weights: w,
                bias: _bias.clone().into()
            }
        }

        pub fn input<T>(&self,inp: &[T]) -> f64 where T:std::convert::Into<f64> + Clone{
            assert!(self.weights.len() == inp.len());
            let mut total:f64 = 0.0;
            for (i, w) in inp.iter().zip(self.weights.as_slice()){
                total = {i.clone().into() * w} + total;
            }
            total + self.bias
        }
        
        // fn input(&self, &[f64]) -> [f64:]{

        // }
    }
}