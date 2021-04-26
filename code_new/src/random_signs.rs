use rand::Rng;

pub fn gen_signs<'a>(len: i32) -> Vec<bool> {
    let mut rng = rand::thread_rng();
    let mut v = Vec::new();
    for _ in 0..len {
        let b: bool = rng.gen();
        v.push(b);
    }
    return v;
}
