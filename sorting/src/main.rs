#![allow(dead_code, unused_variables, unused_mut, unused_imports)]
#![recursion_limit="1000"]
extern crate rand;
use rand::{thread_rng, Rng};

fn main() {
    println!("SOF");

    let k:i64 = 100;

    //let mut arr = [0; 100];
    let mut arr = [0; 100];
    let mut sorted = [-7,0,1,2,3,4,5,5,5,6,7];

    
    
    for a in 0..arr.len(){
        let mut rng = thread_rng();
        arr[a] = rng.gen_range(1,101) as i64;
    }

    for a in arr.iter(){
        print!("{} ",a);
    }
    println!("\n");

    println!("{}",is_sorted(&arr));
    
    let mut sorted_long = neighbor_sort(&mut arr);
}

fn is_sorted(array: &[i64]) -> bool{
    let mut flag = true;

    for a in 0..array.len()-1{
        if array[a] > array[a+1] {
            flag = false;
            break;
        }
    }
    return flag;
}

fn sum_int(array: &[i64]) -> f64{
    let mut out:f64 = 0.0;

    for a in 0..array.len(){
        out += array[a] as f64;
    }

    return out;
}

fn print_arr(array: &[i64]){
    print!("[");
    for a in 0..array.len()-1{
        print!("{} ",array[a]);
    }
    print!("{}",array[array.len()-1]);
    print!("]");
}

fn fibonacci(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n-1) + fibonacci(n-2)
    }
}

fn factorial_rec(n: i128) -> i128{
    if n == 0 {
        return 1;
    }else{
        return n * factorial_rec(n-1);
    }
}

fn min_arr(array: &[i64]) -> i64{
    let mut min_value = &array[0];

    for a in array.iter(){
        if a < min_value{
            min_value = a;
        }
    }

    return *min_value;
}

fn neighbor_sort(array: &mut[i64]) -> &[i64]{
    let mut out = array;

    while !is_sorted(out){
        for a in 0..out.len()-1{
            if &out[a] > &out[a+1]{
                let tmp_high = out[a];
                let tmp_low = out[a+1];

                out[a] = tmp_low;
                out[a+1] = tmp_high;
            }
        }
    }

    return out;
}

fn min_vec(array: &Vec<i64>) -> i64{
    let mut min_value = &array[0];

    for a in array.iter(){
        if a < min_value{
            min_value = a;
        }
    }

    return *min_value;
}

fn is_sorted_vector(invec: &Vec<i64>) -> bool{
    let mut flag = true;

    for a in 0..invec.len()-1{
        if invec[a] > invec[a+1] {
            flag = false;
            break;
        }
    }
    return flag;
}

fn permutation_sort(invec: &Vec<i64>) -> Vec<i64>{
    let mut local_instance = invec;
    let mut tmp_vec = Vec::new();

    let s = local_instance.len();

    while !is_sorted_vector(&tmp_vec){
        let mut tmp_vec = Vec::new();
        let mut temporary_instance = local_instance;

        for a in 0..s{
            let mut rng = thread_rng();
            let index: usize = rng.gen_range(0,s-a) as usize;

            tmp_vec.push(temporary_instance[index]);
            temporary_instance.swap_remove(index);
        }
        
    }

    return tmp_vec;
}