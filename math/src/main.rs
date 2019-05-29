extern crate chrono;
use chrono::prelude::*;

fn rec_fac(x: i64) -> i64{
    if x == 0_i64 {
        return 1_i64;
    }
    x * rec_fac(x-1)
}

fn reg_fac(mut x: i64) -> i64{
    let mut out: i64 = 1;
    while x > 0{
        out *= x;
        x -= 1;
    }
    out
}

fn main(){
    /*
    let mut now: DateTime<Utc> = Utc::now();
    let mut out = 0;
    let n = 10_i64.pow(9)+1;
    for a in 0..n {
        out += a;
    }
    println!("{}",out);
    let f0 = (Utc::now() - now).num_nanoseconds();
    println!("Occured in {:?} nanoseconds.", f0.unwrap());
    //println!("{}", now.elapsed());
    now = Utc::now();
    println!("{}",(n-1) * (n) / 2);
    let f1 = (Utc::now() - now).num_nanoseconds();
    println!("Occured in {:?} nanoseconds.", f1.unwrap());
    println!("{}", out == (n-1) * (n) / 2);
    println!("The mathy way was faster by a factor of {}", f0.unwrap()/f1.unwrap());
    */
    let n = 20;
    let mut start = Utc::now();
    let a = rec_fac(n as i64);
    println!("[RECURSIVE]   Came to value {} in {} seconds.", a, ((Utc::now() - start).num_nanoseconds().unwrap() as f64) / ((10 as i64).pow(9) as f64));

    start = Utc::now();
    let a = reg_fac(n as i64);
    println!("[REGULAR]    Came to value {} in {} seconds.", a, ((Utc::now() - start).num_nanoseconds().unwrap() as f64) / ((10 as i64).pow(9) as f64));

}

