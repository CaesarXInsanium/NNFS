pub fn practice1(){
    let inputs : [f64;3] = [1.0,2.0,3.0];
    let weights: [f64;3] = [0.2,0.8,-0.5];
    let bias: f64 = 2.0;

    let output = {
        inputs[0] * weights[0] +
        inputs[1] * weights[1] +
        inputs[2] * weights[2] + bias
    };
    println!("Output: {:?}", output);
}

pub fn practice2(){
    let inputs: [f64;4] = [1.0,2.0,3.0,2.5];
    let weights:[[f64;4];3] = [
        [0.2, 0.8, -0.5, 1.0],
        [0.5, -0.91, 0.26, -0.5],
        [-0.26, -0.27, 0.17, 0.87]];
    let biases: [f64;3] = [2.0,3.0,0.5];
    let mut layer_output : Vec<f64> = Vec::new();
    println!("inputs: {:?}", inputs);
    println!("weights: {:?}", weights);
    println!("biases: {:?}", biases);
    
    for (neuron_weights, neuron_bias) in weights.iter().zip(&biases){
        let mut neuron_output: f64 = 0.0;
        for (n_input, weight) in inputs.iter().zip(neuron_weights){
            neuron_output += n_input * weight;
        }
        neuron_output += neuron_bias;
        layer_output.push(neuron_output);
    }

    println!("Outputs: {:?}", layer_output);
}