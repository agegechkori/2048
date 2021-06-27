mod field {
    struct Cell {
        row: i32,
        col: i32,
    }

    impl Cell {
        fn is_vertical_with(&self, other: &Cell) -> bool {
            return self.row == other.row;
        }

        fn is_horizontal_with(&self, other: &Cell) -> bool {
            return self.col == other.col;
        }
    }

    struct Lane {
        start: Cell,
        end: Cell,
    }

    impl Lane {
        fn is_vertical(&self) -> bool {
            return self.start.is_vertical_with(&self.end);
        }

        fn is_horizontal(&self) -> bool {
            return self.start.is_horizontal_with(&self.end);
        }
    }

    #[test]
    fn test_vertical() {
        let c1 = Cell { row: 0, col: 0 };
        let c2 = Cell { row: 0, col: 3 };
        let lane = Lane { start: c1, end: c2 };
        assert!(lane.is_vertical());
        assert!(!lane.is_horizontal());
    }

    #[test]
    fn test_horizontal() {
        let c1 = Cell { row: 0, col: 0 };
        let c2 = Cell { row: 3, col: 0 };
        let lane = Lane { start: c1, end: c2 };
        assert!(!lane.is_vertical());
        assert!(lane.is_horizontal());
    }
}

mod naive_impl {
    use mockall::predicate::*;
    use mockall::*;
    use rand::distributions::uniform::SampleRange;
    use rand::distributions::uniform::SampleUniform;
    use rand::distributions::DistIter;
    use rand::distributions::Standard;
    use rand::prelude::*;
    use rand::rngs::mock::StepRng;
    use rand::Error;
    use rand::Fill;

    const TOTAL_PROBABILITY: i8 = 100;

    enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    struct TileOption {
        value: i32,
        probability: i8,
    }

    struct RandomTileGenerator<R: Rng> {
        options: Vec<TileOption>,
        probability_intervals: Vec<f64>,
        rng: R,
    }

    impl<R: Rng> RandomTileGenerator<R> {
        fn new(options: Vec<TileOption>, rng: R) -> Result<RandomTileGenerator<R>, String> {
            let probability_intervals =
                RandomTileGenerator::<R>::create_probability_intervals(&options);
            return match probability_intervals {
                Ok(v) => Ok(RandomTileGenerator {
                    options: options,
                    probability_intervals: v,
                    rng: rng,
                }),
                Err(err) => Err(err),
            };
        }

        pub fn next_tile(&mut self) -> i32 {
            let p: f64 = self.rng.gen();
            let mut index = 0;
            while index < self.probability_intervals.len() && p > self.probability_intervals[index]
            {
                index += 1;
            }

            return self.options[index].value;
        }

        fn create_probability_intervals(options: &Vec<TileOption>) -> Result<Vec<f64>, String> {
            let mut probability_intervals = vec![];
            let mut cummulative_probability = 0;
            for option in options {
                cummulative_probability += option.probability;
                probability_intervals
                    .push(cummulative_probability as f64 / TOTAL_PROBABILITY as f64);
            }
            if cummulative_probability != TOTAL_PROBABILITY {
                return Err(format!(
                    "Probabilities should sum up to {}. Actual sum: {}",
                    TOTAL_PROBABILITY, cummulative_probability
                ));
            }
            return Ok(probability_intervals);
        }
    }

    fn shift_board(board: &Vec<Vec<i32>>, direction: Direction) -> Vec<Vec<i32>> {
        return match direction {
            Direction::Left => shift_board_left(&board),
            Direction::Right => shift_board_right(&board),
            Direction::Up => shift_board_up(&board),
            Direction::Down => shift_board_down(&board),
        };
    }

    fn shift_board_left(v: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut vec = <Vec<Vec<i32>>>::with_capacity(v.len());
        for i in v {
            vec.push(shift_row_left(i));
        }
        return vec;
    }

    fn shift_board_right(v: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut vec = v.clone();
        reverse_rows(&mut vec);

        let mut shifted = shift_board_left(&vec);
        reverse_rows(&mut shifted);
        return shifted;
    }

    fn shift_board_up(v: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        return transpose(&shift_board_left(&transpose(&v.clone())));
    }

    fn shift_board_down(v: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        return transpose(&shift_board_right(&transpose(&v.clone())));
    }

    fn reverse_rows(v: &mut Vec<Vec<i32>>) {
        for i in v {
            i.reverse();
        }
    }

    fn shift_row_left(v: &Vec<i32>) -> Vec<i32> {
        let mut vec = compactify_row(v);
        return combine_paired_cells_in_row(&mut vec);
    }

    fn shift_row_right(v: &Vec<i32>) -> Vec<i32> {
        let mut vv = v.clone();
        vv.reverse();
        let mut vec = shift_row_left(&vv);
        vec.reverse();
        return vec;
    }

    fn combine_paired_cells_in_row(v: &mut Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        while i < v.len() - 1 && v[i] != 0 {
            if v[i] == v[i + 1] {
                v[i] += v[i + 1];
                v[i + 1] = 0;
                i += 2
            } else {
                i += 1
            }
        }
        return compactify_row(v);
    }

    fn compactify_row(v: &Vec<i32>) -> Vec<i32> {
        let mut vec = Vec::new();
        for i in v {
            if *i != 0 {
                vec.push(*i);
            }
        }
        vec.resize(v.len(), 0);
        return vec;
    }

    fn transpose(v: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut vec = vec![vec![0; v.len()]; v[0].len()];
        for i in 0..vec.len() {
            for j in 0..vec[i].len() {
                vec[i][j] = v[j][i];
            }
        }
        return vec;
    }

    fn create_random_tile<R: Rng>(
        v: &Vec<Vec<i32>>,
        generator: &mut RandomTileGenerator<R>,
    ) -> Vec<Vec<i32>> {
        let mut empty_cells = vec![];
        let mut vec = v.clone();
        for i in 0..v.len() {
            for j in 0..v[i].len() {
                if v[i][j] == 0 {
                    empty_cells.push((i, j));
                }
            }
        }

        let mut rng = rand::thread_rng();
        let index: usize = rng.gen_range(0..empty_cells.len());
        vec[empty_cells[index].0][empty_cells[index].1] = generator.next_tile();
        return vec;
    }

    #[test]
    fn test_shift_board() {
        let v1 = vec![
            vec![2, 0, 2, 0],
            vec![0, 4, 4, 2],
            vec![2, 2, 2, 2],
            vec![2, 4, 2, 4],
        ];

        let expected_left = vec![
            vec![4, 0, 0, 0],
            vec![8, 2, 0, 0],
            vec![4, 4, 0, 0],
            vec![2, 4, 2, 4],
        ];
        assert_eq!(shift_board(&v1, Direction::Left), expected_left);

        let expected_right = vec![
            vec![0, 0, 0, 4],
            vec![0, 0, 8, 2],
            vec![0, 0, 4, 4],
            vec![2, 4, 2, 4],
        ];
        assert_eq!(shift_board(&v1, Direction::Right), expected_right);

        let expected_up = vec![
            vec![4, 4, 2, 4],
            vec![2, 2, 4, 4],
            vec![0, 4, 4, 0],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(shift_board(&v1, Direction::Up), expected_up);

        let expected_down = vec![
            vec![0, 0, 0, 0],
            vec![0, 4, 2, 0],
            vec![2, 2, 4, 4],
            vec![4, 4, 4, 4],
        ];
        assert_eq!(shift_board(&v1, Direction::Down), expected_down);
    }

    #[test]
    fn test_shift_board_left() {
        let v1 = vec![
            vec![2, 0, 2, 0],
            vec![0, 4, 4, 2],
            vec![2, 2, 2, 2],
            vec![2, 4, 2, 4],
        ];
        let expected = vec![
            vec![4, 0, 0, 0],
            vec![8, 2, 0, 0],
            vec![4, 4, 0, 0],
            vec![2, 4, 2, 4],
        ];
        assert_eq!(shift_board_left(&v1), expected);
    }

    #[test]
    fn test_shift_board_right() {
        let v1 = vec![
            vec![2, 0, 2, 0],
            vec![0, 4, 4, 2],
            vec![2, 2, 2, 2],
            vec![2, 4, 2, 4],
        ];
        let expected = vec![
            vec![0, 0, 0, 4],
            vec![0, 0, 8, 2],
            vec![0, 0, 4, 4],
            vec![2, 4, 2, 4],
        ];
        assert_eq!(shift_board_right(&v1), expected);
    }

    #[test]
    fn test_shift_board_up() {
        let v1 = vec![
            vec![2, 0, 2, 0],
            vec![0, 4, 4, 2],
            vec![2, 2, 2, 2],
            vec![2, 4, 2, 4],
        ];
        let expected = vec![
            vec![4, 4, 2, 4],
            vec![2, 2, 4, 4],
            vec![0, 4, 4, 0],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(shift_board_up(&v1), expected);
    }

    #[test]
    fn test_shift_board_down() {
        let v1 = vec![
            vec![2, 0, 2, 0],
            vec![0, 4, 4, 2],
            vec![2, 2, 2, 2],
            vec![2, 4, 2, 4],
        ];
        let expected = vec![
            vec![0, 0, 0, 0],
            vec![0, 4, 2, 0],
            vec![2, 2, 4, 4],
            vec![4, 4, 4, 4],
        ];
        assert_eq!(shift_board_down(&v1), expected);
    }

    #[test]
    fn test_reverse_rows() {
        let mut v1 = vec![
            vec![2, 0, 2, 0],
            vec![0, 4, 4, 2],
            vec![2, 2, 2, 2],
            vec![2, 4, 2, 4],
        ];
        let expected = vec![
            vec![0, 2, 0, 2],
            vec![2, 4, 4, 0],
            vec![2, 2, 2, 2],
            vec![4, 2, 4, 2],
        ];
        reverse_rows(&mut v1);
        assert_eq!(v1, expected);
    }

    #[test]
    fn test_shift_row_left() {
        let v1 = vec![2, 0, 0, 0, 2, 0, 4, 0];
        assert_eq!(shift_row_left(&v1), vec![4, 4, 0, 0, 0, 0, 0, 0]);

        let v2 = vec![2, 4, 8, 2, 4, 8, 2, 4];
        assert_eq!(shift_row_left(&v2), vec![2, 4, 8, 2, 4, 8, 2, 4]);

        let v3 = vec![2, 2, 2, 2, 8, 4, 4, 2];
        assert_eq!(shift_row_left(&v3), vec![4, 4, 8, 8, 2, 0, 0, 0]);

        let v4 = vec![0, 0, 0, 2, 2, 2, 2, 4];
        assert_eq!(shift_row_left(&v4), vec![4, 4, 4, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_shift_row_right() {
        let v1 = vec![2, 0, 0, 0, 2, 0, 4, 0];
        assert_eq!(shift_row_right(&v1), vec![0, 0, 0, 0, 0, 0, 4, 4]);

        let v2 = vec![2, 4, 8, 2, 4, 8, 2, 4];
        assert_eq!(shift_row_right(&v2), vec![2, 4, 8, 2, 4, 8, 2, 4]);

        let v3 = vec![2, 2, 2, 2, 8, 4, 4, 2];
        assert_eq!(shift_row_right(&v3), vec![0, 0, 0, 4, 4, 8, 8, 2]);

        let v4 = vec![0, 2, 0, 2, 2, 2, 2, 4];
        assert_eq!(shift_row_right(&v4), vec![0, 0, 0, 0, 2, 4, 4, 4]);
    }

    #[test]
    fn test_compactify() {
        let v1 = vec![2, 0, 0, 0, 2, 0, 4, 0];
        assert_eq!(compactify_row(&v1), vec![2, 2, 4, 0, 0, 0, 0, 0]);

        let v2 = vec![0, 0, 2, 0, 2, 0, 4, 0];
        assert_eq!(compactify_row(&v2), vec![2, 2, 4, 0, 0, 0, 0, 0]);

        let v3 = vec![2, 2, 4, 0, 0, 0, 0, 0];
        assert_eq!(compactify_row(&v3), vec![2, 2, 4, 0, 0, 0, 0, 0]);

        let v4 = vec![2, 2, 4];
        assert_eq!(compactify_row(&v4), vec![2, 2, 4]);
    }

    #[test]
    fn test_combine_paired_cells_in_row() {
        let mut v1 = vec![2, 2, 2, 2, 4, 8, 0, 0];
        assert_eq!(
            combine_paired_cells_in_row(&mut v1),
            vec![4, 4, 4, 8, 0, 0, 0, 0]
        );

        let mut v2 = vec![2, 4, 8, 2, 4, 8, 2, 4];
        assert_eq!(
            combine_paired_cells_in_row(&mut v2),
            vec![2, 4, 8, 2, 4, 8, 2, 4]
        );

        let mut v3 = vec![2, 4, 4, 4, 0, 0, 0, 0];
        assert_eq!(
            combine_paired_cells_in_row(&mut v3),
            vec![2, 8, 4, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_transpose_identity_matrix() {
        let v1 = vec![
            vec![1, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1],
        ];
        assert_eq!(transpose(&v1), v1);
    }

    #[test]
    fn test_transpose_rectangular_matrix() {
        let v1 = vec![
            vec![1, 0, 2, 5, 1, 1],
            vec![0, 1, 3, 6, 2, 7],
            vec![3, 8, 5, 4, 1, 9],
            vec![0, 1, 0, 3, 0, 7],
        ];
        let expected = vec![
            vec![1, 0, 3, 0],
            vec![0, 1, 8, 1],
            vec![2, 3, 5, 0],
            vec![5, 6, 4, 3],
            vec![1, 2, 1, 0],
            vec![1, 7, 9, 7],
        ];
        assert_eq!(transpose(&v1), expected);
        assert_eq!(transpose(&transpose(&v1)), v1);
    }

    #[test]
    fn test_random_tile_generator() {
        // mock! {
        //     MyRng {}     // Name of the mock struct, less the "Mock" prefix

        //     impl RngCore for MyRng {
        //         fn next_u32(&mut self) -> u32;
        //         fn next_u64(&mut self) -> u64;
        //         fn fill_bytes(&mut self, dest: &mut [u8]);
        //         fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error>;
        //     }

        //     impl<R: RngCore + ?Sized> Rng for MyRng {   // specification of the trait to mock
        //         fn gen<T>(&mut self) -> T where Standard: Distribution<T>;
        //         fn gen_range<T, R>(&mut self, range: R) -> T where T: SampleUniform, R: SampleRange<T>;
        //         fn sample<T, D: Distribution<T>>(&mut self, distr: D) -> T;
        //         fn sample_iter<T, D>(self, distr: D) -> DistIter<D, Self, T> where D: Distribution<T>, Self: Sized;
        //         fn fill<T: Fill + ?Sized>(&mut self, dest: &mut T);
        //         fn try_fill<T: Fill + ?Sized>(&mut self, dest: &mut T);
        //         fn gen_bool(&mut self, p: f64) -> bool;
        //         fn gen_ratio(&mut self, numerator: u32, denominator: u32) -> bool;
        //     }
        // }

        // RandomTileGenerator::new(
        //     vec![
        //         TileOption {
        //             value: 2,
        //             probability: 10,
        //         },
        //         TileOption {
        //             value: 4,
        //             probability: 20,
        //         },
        //         TileOption {
        //             value: 4,
        //             probability: 30,
        //         },
        //         TileOption {
        //             value: 4,
        //             probability: 40,
        //         },
        //     ],
        //     StepRng::new(2, 1),
        // );

        let mut r = StepRng::new(2, 1);
        let f: [u64; 3] = r.gen();
        println!("{:?}", f);
        let ff: f64 = r.gen();
        println!("{}", ff);
    }

    #[test]
    fn test_create_probability_intervals() {
        let probability_intervals_1 =
            RandomTileGenerator::<ThreadRng>::create_probability_intervals(&vec![
                TileOption {
                    value: 2,
                    probability: 10,
                },
                TileOption {
                    value: 4,
                    probability: 20,
                },
                TileOption {
                    value: 4,
                    probability: 30,
                },
                TileOption {
                    value: 4,
                    probability: 40,
                },
            ]);
        assert_eq!(probability_intervals_1.unwrap(), vec![0.1, 0.3, 0.6, 1.0]);

        let probability_intervals_2 =
            RandomTileGenerator::<ThreadRng>::create_probability_intervals(&vec![
                TileOption {
                    value: 2,
                    probability: 30,
                },
                TileOption {
                    value: 4,
                    probability: 10,
                },
                TileOption {
                    value: 4,
                    probability: 35,
                },
                TileOption {
                    value: 4,
                    probability: 25,
                },
            ]);
        assert_eq!(probability_intervals_2.unwrap(), vec![0.3, 0.4, 0.75, 1.0]);

        let invalid_probability_intervals_1 =
            RandomTileGenerator::<ThreadRng>::create_probability_intervals(&vec![
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

        let invalid_probability_intervals_2 =
            RandomTileGenerator::<ThreadRng>::create_probability_intervals(&vec![
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
}
