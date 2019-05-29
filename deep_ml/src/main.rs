#![allow(dead_code)]
extern crate rand;
extern crate ndarray;
extern crate ndarray_rand;

use self::ndarray::*;
use self::ndarray_rand::*;
use self::rand::distributions::Uniform;

mod layer;
use layer::*;

fn main() {
    //let s = Array::zeros(vec![28*28]);
    //println!("{:?}", s);
    //println!("{:?}", s.shape());
}