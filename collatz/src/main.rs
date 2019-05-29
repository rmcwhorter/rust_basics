mod collatz;
use collatz::*;

use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn write_out(f: &str, w: &str) -> io::Result<()> {
    let mut out = File::create(f)?;
    write!(out, "{}", w)?;
    Ok(())
}

fn main() {
    let mut collatz_tree = Arena::<i128> {
        nodes : Vec::new()
    };

    collatz_tree.new_node(1);

    let start = SystemTime::now();
    let k: i128 = 101;
    for a in 2..k {
        collatz_tree.augment(a, None);
    }
    let elapsed = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("We computed {} Collatz paths in {:?} milliseconds.", k-2, elapsed);

    let mut outstr = String::new();
    for n in &collatz_tree.nodes {
        //println!("{:?} => {:?} => {:?}", n.parent, n.data, n.child);
        outstr.push_str(&format!("{}\n", n));
        println!("{} Child's Data: {}", n, match n.child {Some(x) => x.index,
        None => std::usize::MAX});
    }

    write_out("collatz_tree.txt", &outstr);
}
