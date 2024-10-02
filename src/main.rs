mod rand_vec;
use rand_vec::rand_vec;
mod mergesort; 
use mergesort::mergesort;

fn main() {

    let mut disorder = rand_vec(10);
    println!("{:?}", disorder);

    let end: usize = disorder.len() - 1;
    mergesort(&mut disorder,0, end);

    println!("Test mergesort: {:?}",disorder);
}
