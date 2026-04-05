use ndarray::prelude::*;
use rand::Rng;


pub struct EchoStateNetwork{
    pub reservoir_size: usize,
    pub input_size: usize,
    pub win: Array2<f64>,
    pub w: Array2<f64>,
    pub spectral_radius: f64,
    pub leaking_rate: f64,
    pub state: Array2<f64>,
}

impl EchoStateNetwork{
    pub fn new(input_size:usize, reservoir_size:usize, spectral_radius: f64, leaking_rate: f64)->Self{
        let mut rng = rand::thread_rng();
        let win = Array2::random((reservoir_size, input_size), rand::distributions::Uniform::new(-0.5, 0.5));
        
    }
}