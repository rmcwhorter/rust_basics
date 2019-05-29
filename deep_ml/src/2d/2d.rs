#![allow(dead_code)]
#[macro_use(array)]
extern crate ndarray;
extern crate rand;
extern crate ndarray_rand;

mod activation;
mod layer;
mod mnist;
mod model;

use mnist::*;
use activation::*;
use layer::*;
use model::*;
use ndarray::*;

fn main() {
    let mnist_db = mnist();
    let input: &Array<f64, Ix2> = &mnist_db["5"][0];
    let output: Array<f64, Ix2> = array![
        [0.0], //This isn't a zero
        [0.0], //This isn't a 1
        [0.0],
        [0.0],
        [0.0],
        [1.0], //This is a 5
        [0.0],
        [0.0],
        [0.0],
        [0.0]
    ];

    println!("{:?}\n=>\n{:?}\n\n", input, output);
    
    /*
    let mut l1 = layer::__init__((28,1), &input, &activation::softmax);
    let mut l2 = layer::__init__((5,1), &l1.matrix, &activation::sigmoid);

    println!("{:?} => {:?} => {:?}", input, l1.matrix, l2.matrix);
    l1.feedforward(&input);
    l2.feedforward(&l1.matrix);
    println!("\n\n\n\n\n{:?} => {:?} => {:?}", input, l1.matrix, l2.matrix);
    */
    let shapes = vec![
        (28,28),
        (1,1)
    ];
    let fns: Vec<&'static Fn(f64, bool) -> f64> = vec![
        &relu,
        &relu
    ];

    let mut new_model = __MODEL__INIT__(shapes, fns);
    println!("{}", new_model);
}