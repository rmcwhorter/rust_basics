#![allow(dead_code)]
extern crate rand;
extern crate ndarray;
extern crate ndarray_rand;

use self::ndarray::*;
use self::ndarray_rand::*;
use self::rand::distributions::Uniform;

pub fn create_synapses(inmatrix: &Array<f64, Ix2>, outmatrix: &Array<f64, Ix2>) -> Array2<f64>{// -> ArrayD<f64>
    let shape = (inmatrix.raw_dim()[0], outmatrix.raw_dim()[1]);
    //println!("{:?} {:?} {:?}", inmatrix.raw_dim(), outmatrix.raw_dim(), shape);
    //println!("{:?}", shape);
    let out: Array2<f64> = Array::random(shape, Uniform::new(-1.0, 1.0));

    return out;
}

pub struct Layer {
    pub matrix : Array2<f64>, //this represents the values we will compute
    pub in_synapses : Array2<f64>, //this represents the weights that we will use to compute the next layer
    pub activation_fn : &'static Fn(f64, bool) -> f64 //this represents the activation function we will use to compute this layer
}

pub fn __LAYER__INIT__(shape: (usize, usize), prior_matrix: &Array2<f64>, activation_fn: &'static Fn(f64, bool) -> f64) -> Layer{
        Layer {
            matrix: Array::zeros(shape),
            in_synapses: create_synapses(prior_matrix, &Array::zeros(shape)),
            activation_fn: activation_fn
        }
    }

impl Layer {
    pub fn feedforward(&mut self, x: &Array2<f64>) {
        self.matrix = x.dot(&self.in_synapses);
        self.matrix = self.matrix.mapv(|a| (self.activation_fn)(a, false));
        println!("FF: {:?} dot {:?} => {:?}", x.shape(), &self.in_synapses.shape(), &self.matrix.shape());
    }
}