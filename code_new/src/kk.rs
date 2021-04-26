use std::collections::BinaryHeap;

fn kk_pop(heap: &mut BinaryHeap<i32>) -> i32 {
    return match heap.pop() {
        None => 0,
        Some(val) => val,
    };
}

pub fn kk(a: &[i32]) -> i32 {
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
