#![allow(dead_code)]
extern crate rand;
extern crate ndarray;
extern crate ndarray_rand;

use self::ndarray::*;
use self::ndarray_rand::*;
use self::rand::distributions::Uniform;

pub struct Layer {
    pub matrix : Array1<f64>, //this represents the values we will compute
    pub in_synapses : Array2<f64>, //this represents the weights that we will use to compute the next layer
    pub activation_fn : &'static Fn(f64, bool) -> f64 //this represents the activation function we will use to compute this layer
}

pub fn create_synapses(inmatrix: usize, outmatrix: usize) -> Array2<f64>{// -> ArrayD<f64>
    let mut out = vec![];
    for a in 0..inmatrix {
        out.push(vec![0.0; outmatrix]);
    }

    return [0,1,2,3,5,6].into();
}