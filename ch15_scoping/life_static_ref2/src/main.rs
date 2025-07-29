extern crate rand;
use rand::Fill;

#[allow(deprecated)]
fn random_vec() -> &'static [u128; 100] {
    let mut rng = rand::thread_rng();
    let mut boxed = Box::new([0; 100]);
    boxed.fill(&mut rng);
    Box::leak(boxed)
}

fn main() {
    let first: &'static [u128; 100] = random_vec();
    let second: &'static [u128; 100] = random_vec();
    assert_ne!(first, second)
}