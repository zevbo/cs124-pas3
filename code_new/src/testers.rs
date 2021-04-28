use core::num;

use crate::signs;
use crate::{helpers, kk};
use rand::Rng;

#[derive(Copy, Clone)]
pub enum Algorithm {
    Random,
    Climbing,
    Annealing,
}

fn t(prev_result: f64, multiplier: f64) -> f64 {
    return prev_result * multiplier;
}
fn threshold(iter: i32, last_residue: i64, new_residue: i64, result: f64) -> f64 {
    let prob =
       // 2.718f64.powf(-(7. * (new_residue as f64 / last_residue as f64 - 1.)) as f64 / result);
       2.718f64.powf((last_residue - new_residue) as f64 / result);
    if false && iter == 50 {
        println!(
            "Data: {}, {}, {}",
            prob,
            result,
            new_residue as f64 / last_residue as f64
        );
    }
    return prob;
}

pub fn partition_test(a: &helpers::A, max_iter: i32, algo: Algorithm) -> i64 {
    let mut partition = kk::gen_partition(a.len());
    let mut last_residue = kk::evaluate(a, &partition);
    let mut rng = rand::thread_rng();
    let multiplier = 0.8f64.powf(1.0 / 300.);
    /*print!("{}: [", &last_residue);
    for (val, group) in a.iter().zip(&partition) {
        print!("({}, {}), ", val, group);
    }
    println!("]");*/
    let mut prev_result = 10_i64.pow(10) as f64;
    for i in 1..max_iter {
        let new_partition = match algo {
            Algorithm::Random => kk::gen_partition(a.len()),
            Algorithm::Climbing | Algorithm::Annealing => kk::rand_edit(&partition),
        };
        let new_residue = kk::evaluate(&a, &new_partition);
        prev_result = t(prev_result, multiplier);
        let use_new = new_residue <= last_residue
            || match algo {
                Algorithm::Annealing => {
                    rng.gen_range(0.0..1.0) < threshold(i, last_residue, new_residue, prev_result)
                }
                _ => false,
            };
        if use_new {
            partition = new_partition;
            last_residue = new_residue
        }
    }
    return last_residue;
}

pub fn run_partition_tests(num_tests: usize, size: usize, max_iter: i32) -> (f32, f32, f32) {
    let mut sumRand = 0;
    let mut sumClimb = 0;
    let mut sumAnneal = 0;
    for _ in 0..num_tests {
        let a = helpers::gen_a(size);
        sumRand += partition_test(&a, max_iter, Algorithm::Random);
        sumClimb += partition_test(&a, max_iter, Algorithm::Climbing);
        sumAnneal += partition_test(&a, max_iter, Algorithm::Annealing);
    }
    return (
        (sumRand as f32) / (num_tests as f32),
        (sumClimb as f32) / (num_tests as f32),
        (sumAnneal as f32) / (num_tests as f32),
    );
}
