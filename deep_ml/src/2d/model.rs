#![allow(dead_code)]
extern crate rand;
extern crate ndarray;
extern crate ndarray_rand;

use self::ndarray::*;
use self::ndarray_rand::*;
use self::rand::distributions::Uniform;

use layer::{Layer, __LAYER__INIT__};
use std::fmt;
use activation::*;

pub struct Model {
    pub input_shape: (usize, usize),
    pub layers: Vec<Layer>
}

fn float_comp(a: f64, b: f64) -> bool {
    if (a-b).abs() < 0.0000000000000001 {
        return true;
    } else {
        return false;
    }
}

impl Model {
    pub fn feedforward(&mut self, input: &Array2<f64>) -> Array2<f64> {
        self.layers[0].feedforward(&input);

        if self.layers.len() > 1 {
            for a in 1..self.layers.len(){
                let tmp = self.layers[a-1].matrix.clone();
                self.layers[a].feedforward(&tmp);
            }
        }
        return self.layers.last().unwrap().matrix.clone();
    }

    pub fn train(&mut self, x: &Vec<Array2<f64>>, y: &Vec<Array2<f64>>) {

    }
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write!(f, "({}, {})", self.x, self.y)
        let mut out = String::new();
        out.push_str(&format!("{:?} => ", vec![self.input_shape.0, self.input_shape.1]));
        for a in &self.layers {
            out.push_str(&format!("{:?} => ", a.matrix.shape()));
        }
        out = out[0..out.len()-4].to_string();
        out.push_str("\n");
        for a in &self.layers {
            out.push_str(&format!("{:?} => ", a.in_synapses.shape()));
        }
        out = out[0..out.len()-4].to_string();
        out.push_str("\n");
        for a in &self.layers {
            if float_comp((a.activation_fn)(-0.5, false), sigmoid(-0.5, false)) {
                out.push_str("sigmoid");
            } else if float_comp((a.activation_fn)(-0.5, false), relu(-0.5, false)) {
                out.push_str("relu");
            } else if float_comp((a.activation_fn)(-0.5, false), softmax(-0.5, false)) {
                out.push_str("softmax");
            } else {
                out.push_str("unknown function");
            }
            out.push_str(" => ")
        }

        out = out[0..out.len()-4].to_string();
        write!(f, "{}", out)
    }
}

pub fn __MODEL__INIT__(shapes: Vec<(usize, usize)>, activation_fns: Vec<&'static Fn(f64, bool) -> f64>) -> Model{
    /*let mut l1 = layer::__init__((28,1), &input, &activation::softmax);
    let mut l2 = layer::__init__((5,1), &l1.matrix, &activation::sigmoid);*/

    let mut layers = vec![];
    for tick in 0..shapes.len()-1 {
        //pub fn __init__(shape: (usize, usize), prior_matrix: &Array2<f64>, activation_fn: &'static Fn(f64, bool) -> f64) -> Layer{
        layers.push(__LAYER__INIT__(shapes[tick+1], &Array::zeros(shapes[tick]), activation_fns[tick]));
    }

    return Model {
        input_shape: shapes[0],
        layers: layers
    }
}