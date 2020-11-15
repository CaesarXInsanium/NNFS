mod chap2;
use chap2::{transpose_vec};

fn main() {
    let inputs: Vec<Vec<f64>> = vec!(
                vec!(1.0, 2.0,3.0,4.5),
                vec!(5.0,6.0,-7.0,8.0),
                vec!(-9.5, 10.7, 11.3, -12.8),
           );
    
    println!("Tranposed: {:?}", transpose_vec(&inputs));
}


