struct Matrix{
    matrix: Vec<Vec<f64>>,
    shape: [u32;2]
}

impl Matrix{
    pub fn new(array: Vec<Vec<f64>>) -> Matrix{
        Matrix{
            matrix: array,
            shape: [array.len() as u32, array[0].len() as u32]
        }
    }
}
