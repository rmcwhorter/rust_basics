fn fac(x: i128) -> i128 {
    let mut out: i128 = 1;
    for a in 1..x {
        out *= a;
    }
    return out;
}


fn integrate(f: &'static Fn(f64) -> f64, lb: f64, ub: f64, i: usize) -> f64 {
    let mut out = 0.0;
    let w = (ub - lb)/(i as f64);
    for a in 0..i {
        out += f(lb + (a as f64 * w));
    }
    return out*w;
}

fn linear(x: f64) -> f64 {
    return x;
}

fn e(prec: f64) -> f64 {
    return (1.0 + (1.0/prec)).powf(prec);
}

fn main() {
    let f64_e = e(2646515.0);
    let s = integrate(&linear, 0.0, 10.0, 10000000);
    println!("{}", s);
    println!("{}", e(2646514.0));
    println!("{}", f64_e);
    println!("{}", e(2646516.0));
}
