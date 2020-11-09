
pub mod layer_n;
use layer_n::chap1::Neuron;



fn main() {
    let input:[i64;3] = [1,2,3];
    let weights:[f64;3] = [0.2, 0.8, -0.5];
    let bias = 2;
    let n:Neuron = Neuron::build_neuron_slice(weights);

    



}
