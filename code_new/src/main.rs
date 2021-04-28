use std::print;

mod helpers;
mod kk;
mod signs;

fn main() {
    let len = 4;
    let partition = kk::gen_partition(len);
    for val in &partition {
        println!("{}", val);
    }
    let mut a = Vec::new();
    for _i in 0..len {
        a.push(1);
    }
    println!("{}", kk::evaluate(&a, &partition));
    println!("Hello, world!");
}
