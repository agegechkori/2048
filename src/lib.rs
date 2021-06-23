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
    enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    fn shift(board: &Vec<Vec<i32>>, direction: Direction) -> Vec<Vec<i32>> {
        let mut v = board.clone();

        match direction {
            Direction::Down => {
                println!("Down");
            }
            Direction::Up => println!("Up"),
            Direction::Left => {
                println!("Left");
                for i in 0..v.len() {
                    for j in 1..v[i].len() {
                        if v[i][j] == 0 {
                            v[i][j] = v[i][j + 1];
                            v[i][j + 1] = 0;
                        }
                    }
                    for j in 1..v[i].len() {
                        if v[i][j] == 0 {
                            break;
                        }
                        if v[i][j] == v[i][j + 1] {
                            v[i][j] += v[i][j + 1];
                            v[i][j + 1] = 0;
                        }
                    }
                }
            }
            Direction::Right => println!("Right"),
        }
        return v;
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
    fn test_combine_paired_cells() {
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
}
