pub use std::iter::Zip;

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

pub fn practice3(){
    let inputs: Vec<f64> = vec!(1.0,2.0,3.0,2.5);
    let weights: Vec<Vec<f64>> = vec!(
        vec!(0.2, 0.8, -0.5, 1.0),
        vec!(0.5, -0.91, 0.26, -0.5),
        vec!(-0.26, -0.27, 0.17, 0.87)
    );
    let biases: Vec<f64> = vec!(2.0,3.0, 0.5);
    let mut output: Vec<f64> = Vec::new();

    for (neuron_weights, neuron_bias) in weights.iter().zip(biases){
        let mut neuron_output : f64 = 0.0;
        for (n_input, weight) in inputs.iter().zip(neuron_weights){
            neuron_output += n_input * weight;
        }
        neuron_output += neuron_bias;
        output.push(neuron_output);
    }
    println!("Outputs: {:?}", output);
}

pub fn is_homologous<T>(x: &Vec<Vec<T>>) -> bool{
    let result = true;
    let n_columns = x[0].len();
    for i in x{
        if i.len() != n_columns{
            return false;
        }
    }
    result
}

// pub fn does_shape_match(x :&Vec<Vec<f64>>,y :&Vec<Vec<f64>>) -> bool{
//     x[0].len() == y.len()

// }
pub fn matrix_mul(x :&Vec<Vec<f64>>,y :&Vec<Vec<f64>>) -> Vec<Vec<f64>>{
    assert!(is_homologous(&x));
    assert!(is_homologous(&y));
    //assert!(does_shape_match(&x, &y));

    let mut result : [[f64;y[0].len()];x.len()];
    for i in 0..x.len()-1{
        for j in 0..y[0].len()-1{
            for k in 0..y[0].len()-1{
                result[i][j] += x[i][k] + y[k][j];
            }
        }
    }
    result
 
}

// pub fn practice4(){
//     let inputs: Vec<Vec<f64>> = vec!(
//         vec!(1.0, 2.0,3.0,2.5),
//         vec!(2.0,5.0,-1.0,2.0),
//         vec!(-1.5, 2.7, 3.3, -0.8),
//     );
//     let weights: Vec<Vec<f64>> = vec!(
//         vec!(0.2, 0.8, -0.5, 1.0),
//         vec!(0.5, -0.91, 0.26, -0.5),
//         vec!(-0.26, -0.27, 0.17, 0.87)
//     );
//     let biases: Vec<f64> = vec!(2.0,3.0, 0.5);

//     let output = matrix_mul(&inputs, &transpose_vec(&weights)();
//     println!("Output: {:?}", output)


// }

pub fn transpose_vec<T>(vec: &Vec<Vec<T>>) -> Vec<Vec<T>>
    where T: std::marker::Copy{
    assert!(is_homologous(vec));
    let mut result: Vec<Vec<T>> = Vec::new();
    for colum_index in 0..vec[0].len(){
        let mut result_row : Vec<T> = Vec::new();
        for vec_row in vec{
            result_row.push(vec_row[colum_index]);
        }
        result.push(result_row);
    }
    result
}