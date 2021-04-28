use std::{cmp::min, print};

use testers::Algorithm;

mod helpers;
mod kk;
mod signs;
mod testers;

fn main() {
    println!("Hello, world!");
    let (resultRand, resultClimb, resultAnneal) = testers::run_partition_tests(5, 100, 25000);
    println!("{}, {}, {}", resultRand, resultClimb, resultAnneal);
    println!("Winner: {}", resultRand.min(resultClimb).min(resultAnneal));
    println!("Loser: {}", resultRand.max(resultClimb).max(resultAnneal));
}
