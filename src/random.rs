use rand::Rng;
use std::ops::Range;

struct SimpleGenerator<R: Rng> {
    rng: R,
}

pub trait RandomNumberGenerator {
    fn next_float(&mut self) -> f64;
    fn next_in_range(&mut self, range: Range<i32>) -> i32;
}

impl<R: Rng> RandomNumberGenerator for SimpleGenerator<R> {
    fn next_float(&mut self) -> f64 {
        self.rng.gen()
    }

    fn next_in_range(&mut self, range: Range<i32>) -> i32 {
        self.rng.gen_range(range)
    }
}

pub fn create_simple_generator() -> impl RandomNumberGenerator {
    SimpleGenerator {
        rng: rand::thread_rng(),
    }
}

#[test]
fn test_next_float() {
    let mut srng = SimpleGenerator {
        rng: rand::thread_rng(),
    };
    let random_float = srng.next_float();
    assert!((0.0..1.0).contains(&random_float));
}

#[test]
fn test_next_in_range() {
    let mut srng = SimpleGenerator {
        rng: rand::thread_rng(),
    };
    let random_int = srng.next_in_range(10..25);
    assert!((10..25).contains(&random_int));
}
