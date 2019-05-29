#![allow(dead_code)]
use std::f64::consts::{E};

//Utility functions
fn fltmax(a: f64, b: f64) -> f64 {
    if a > b {
        return a;
    } else if a < b {
        return b;
    } else {
        return a;
    }
}

//Activation functions
pub fn sigmoid(x: f64, deriv: bool) -> f64 {
    if deriv {
        return E.powf(x) / (E.powf(x) + 1.0).powf(2.0);
    } else {
        return 1.0 / (1.0 + E.powf(-x));
    }
}

pub fn relu(x: f64, deriv: bool) -> f64 {
    if deriv {
        if x <= 0.0 {
            return 0.0;
        } else {
            return 1.0;
        }
    } else {
        return fltmax(0.0,x);
    }
}

pub fn softmax(x: f64, deriv: bool) -> f64 {
    if deriv {
        return E.powf(x) / (1.0 + E.powf(x));
    } else {
        return (1.0 + E.powf(x)).ln();
    }
}