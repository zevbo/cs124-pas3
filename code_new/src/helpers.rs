use rand::Rng;

pub fn gen_unequal(len: usize) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    let ind1: usize = rng.gen::<usize>() % len;
    let ind2_temp: usize = rng.gen::<usize>() % (len - 1);
    let ind2 = ind2_temp + if ind2_temp >= ind1 { 1 } else { 0 };
    assert_ne!(ind1, ind2);
    return (ind1, ind2);
}
