use crate::random::{self, RandomNumberGenerator};

const TOTAL_PROBABILITY: i8 = 100;

struct TileOption {
    value: i32,
    probability: i8,
}

pub struct TileGenerator<R: RandomNumberGenerator> {
    options: Vec<TileOption>,
    probability_intervals: Vec<f64>,
    rng: R,
}

impl<R: RandomNumberGenerator> TileGenerator<R> {
    fn new(options: Vec<TileOption>, rng: R) -> Result<TileGenerator<R>, String> {
        let probability_intervals = create_probability_intervals(&options);
        return match probability_intervals {
            Ok(v) => Ok(TileGenerator {
                options: options,
                probability_intervals: v,
                rng: rng,
            }),
            Err(err) => Err(err),
        };
    }

    pub fn generate_tile(&mut self, empty_cells: &Vec<(usize, usize)>) -> (usize, usize, i32) {
        let (i, j) = self.select_empty_cell(empty_cells);
        let tile = self.next_tile();
        return (i, j, tile);
    }

    pub fn next_tile(&mut self) -> i32 {
        let p: f64 = self.rng.next_float();
        return self.next_tile_internal(p);
    }

    pub fn select_empty_cell(&mut self, empty_cells: &Vec<(usize, usize)>) -> (usize, usize) {
        return empty_cells[self.rng.next_in_range(0..empty_cells.len() as i32) as usize];
    }

    fn next_tile_internal(&self, random_number: f64) -> i32 {
        let mut index = 0;
        while index < self.probability_intervals.len()
            && random_number > self.probability_intervals[index]
        {
            index += 1;
        }

        return self.options[index].value;
    }
}

fn create_probability_intervals(options: &Vec<TileOption>) -> Result<Vec<f64>, String> {
    let mut probability_intervals = vec![];
    let mut cummulative_probability = 0;
    for option in options {
        cummulative_probability += option.probability;
        probability_intervals.push(cummulative_probability as f64 / TOTAL_PROBABILITY as f64);
    }
    if cummulative_probability != TOTAL_PROBABILITY {
        return Err(format!(
            "Probabilities should sum up to {}. Actual sum: {}",
            TOTAL_PROBABILITY, cummulative_probability
        ));
    }
    return Ok(probability_intervals);
}

#[test]
fn test_random_tile_generator() {
    let random_tile_generator = TileGenerator::new(
        vec![
            TileOption {
                value: 2,
                probability: 10,
            },
            TileOption {
                value: 4,
                probability: 20,
            },
            TileOption {
                value: 8,
                probability: 30,
            },
            TileOption {
                value: 16,
                probability: 40,
            },
        ],
        random::create_simple_generator(),
    )
    .unwrap();

    assert_eq!(random_tile_generator.next_tile_internal(0.61), 16);
    assert_eq!(random_tile_generator.next_tile_internal(0.35), 8);
    assert_eq!(random_tile_generator.next_tile_internal(0.25), 4);
    assert_eq!(random_tile_generator.next_tile_internal(0.09), 2);
}

#[test]
fn test_create_probability_intervals() {
    let probability_intervals_1 = create_probability_intervals(&vec![
        TileOption {
            value: 2,
            probability: 10,
        },
        TileOption {
            value: 4,
            probability: 20,
        },
        TileOption {
            value: 8,
            probability: 30,
        },
        TileOption {
            value: 16,
            probability: 40,
        },
    ]);
    assert_eq!(probability_intervals_1.unwrap(), vec![0.1, 0.3, 0.6, 1.0]);

    let probability_intervals_2 = create_probability_intervals(&vec![
        TileOption {
            value: 2,
            probability: 30,
        },
        TileOption {
            value: 4,
            probability: 10,
        },
        TileOption {
            value: 8,
            probability: 35,
        },
        TileOption {
            value: 16,
            probability: 25,
        },
    ]);
    assert_eq!(probability_intervals_2.unwrap(), vec![0.3, 0.4, 0.75, 1.0]);

    let invalid_probability_intervals_1 = create_probability_intervals(&vec![
        TileOption {
            value: 2,
            probability: 30,
        },
        TileOption {
            value: 4,
            probability: 80,
        },
    ]);
    assert!(invalid_probability_intervals_1.is_err());

    let invalid_probability_intervals_2 = create_probability_intervals(&vec![
        TileOption {
            value: 2,
            probability: 30,
        },
        TileOption {
            value: 4,
            probability: 60,
        },
    ]);
    assert!(invalid_probability_intervals_2.is_err());
}

#[test]
fn test_generate_tile() {
    // TODO implement test with mocks
}