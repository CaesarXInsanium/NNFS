mod chap1{
    mod l{
        pub struct Layer{
            lay: Vec<Neuron>
        }
        
        impl Layer{
            pub fn random_layer(number_neurons: i32) -> Layer{
                let mut result:Vec<Neuron>;
                let mut rng = rand::thread_rng();
        
                for i in 0..number_neurons{
                    let w:Vec<f32> = (0..number_neurons).map(|_| rng.gen_range(-1,1)).collect();
                    let b:Vec<f32> = (0..number_neurons).map(|_| rng.gen_range(-1,1)).collect();
                    result.push(Neuron::build_neuron_slice(&w.as_slice().clone(), &b.as_slice().clone()))
                }
                Layer{
                    lay: result
                }
            }
        }
    }

    mod n{
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
    
        }
    }
}