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
        return self.rng.gen();
    }

    fn next_in_range(&mut self, range: Range<i32>) -> i32 {
        return self.rng.gen_range(range);
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
    assert!(random_float >= 0.0 && random_float < 1.0);
}

#[test]
fn test_next_in_range() {
    let mut srng = SimpleGenerator {
        rng: rand::thread_rng(),
    };
    let random_int = srng.next_in_range(10..25);
    assert!(random_int >= 10 && random_int < 25);
}
