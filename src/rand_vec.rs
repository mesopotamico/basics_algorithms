use rand::Rng;
pub fn rand_vec(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let disorder: Vec<i32> = (0..size)
        .map(|_| rng.gen_range(0..100))
        .collect();

    disorder
}
