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

    let mut unsorted_vec = vec![];
    for a in 0..101{
        unsorted_vec.push(thread_rng().gen_range(1,101) as i64);
    }
    
    for a in 0..arr.len(){
        let mut rng = thread_rng();
        arr[a] = rng.gen_range(1,101) as i64;
    }

    for a in arr.iter(){
        print!("{} ",a);
    }
    println!("\n");

    println!("{}",is_sorted(&arr));
    
    let mut sorted_long = bubble_sort(&mut arr);
    let mut sorted_merge = merge_sort(unsorted_vec);

    for a in sorted_merge.iter(){
        print!("{} ",a);
    }
    println!("\n{}",is_sorted_vector(&sorted_merge));
    println!("\n{}",is_sorted_vector(&sorted_merge[0..(sorted_merge.len()/2)-1].to_vec()));
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

fn bubble_sort(array: &mut[i64]) -> &[i64]{
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

fn merge_sort(mut invec: Vec<i64>) -> Vec<i64>{
    if invec.len() > 1{
        let middle = invec.len()/2;

        let mut l = merge_sort(invec[0..middle].to_vec());
        let mut r = merge_sort(invec[middle..].to_vec());

        let mut i = 0;
        let mut j = 0;
        let mut k = 0;

        while i < l.len() && j < r.len(){
            if l[i] < r[j]{
                invec[k] = l[i];
                i += 1;
            }else{
                invec[k] = r[j];
                j += 1;
            }
            k += 1;
        }
    }

    return invec;
}
