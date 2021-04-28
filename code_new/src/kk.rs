use crate::helpers;
use rand::Rng;
use std::collections::BinaryHeap;

type PartitionT = Vec<usize>;

fn kk_pop(heap: &mut BinaryHeap<usize>) -> usize {
    return match heap.pop() {
        None => 0,
        Some(val) => val,
    };
}

pub fn kk_evaluate(a: &Vec<usize>) -> usize {
    let mut heap = BinaryHeap::new();
    for val in a {
        heap.push(*val);
    }
    loop {
        let val1 = kk_pop(&mut heap);
        let val2 = kk_pop(&mut heap);
        if val2 == 0 {
            heap.push(val1);
            break;
        }
        heap.push(val1 - val2);
    }
    return kk_pop(&mut heap);
}

pub fn apply_partition(a: &Vec<usize>, partition: &PartitionT) -> Vec<usize> {
    let mut new_a = vec![0; a.len()];
    for (i, group) in partition.iter().enumerate() {
        new_a[*group] += a[i];
    }
    return new_a;
}

pub fn evaluate(a: &Vec<usize>, partition: &PartitionT) -> usize {
    return kk_evaluate(&apply_partition(a, partition));
}

pub fn gen_partition(len: usize) -> PartitionT {
    let mut rng = rand::thread_rng();
    let mut v = PartitionT::new();
    for _ in 0..len {
        v.push(rng.gen::<usize>() % len + 1);
    }
    return v;
}

pub fn rand_edit(partition: &PartitionT) -> PartitionT {
    let mut new_partition = PartitionT::clone(partition);
    let (ind1, ind2) = helpers::gen_unequal(partition.len());
    new_partition[ind1] = new_partition[ind2];
    return new_partition;
}
