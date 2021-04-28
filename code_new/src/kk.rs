use crate::helpers;
use rand::Rng;
use std::collections::BinaryHeap;

type PartitionT = Vec<usize>;

fn kk_pop(heap: &mut BinaryHeap<i64>) -> i64 {
    return match heap.pop() {
        None => 0,
        Some(val) => val,
    };
}

pub fn kk_evaluate(a: &helpers::A) -> i64 {
    let mut heap = BinaryHeap::new();
    for val in a {
        heap.push(*val);
    }
    loop {
        let val1 = kk_pop(&mut heap);
        let val2 = kk_pop(&mut heap);
        if val2 == 0 {
            return val1;
        }
        heap.push(val1 - val2);
    }
}

pub fn apply_partition(a: &helpers::A, partition: &PartitionT) -> helpers::A {
    assert!(a.len() == partition.len());
    let mut new_a = vec![0; a.len()];
    for (i, group) in partition.iter().enumerate() {
        new_a[*group] += a[i];
    }
    return new_a;
}

pub fn evaluate(a: &helpers::A, partition: &PartitionT) -> i64 {
    return kk_evaluate(&apply_partition(a, partition));
}

pub fn gen_partition(len: usize) -> PartitionT {
    let mut rng = rand::thread_rng();
    let mut v = PartitionT::new();
    for _ in 0..len {
        v.push(rng.gen::<usize>() % len);
    }
    return v;
}

pub fn rand_edit(partition: &PartitionT) -> PartitionT {
    let mut new_partition = PartitionT::clone(partition);
    let (ind1, ind2) = helpers::gen_unequal(partition.len());
    new_partition[ind1] = new_partition[ind2];
    return new_partition;
}
