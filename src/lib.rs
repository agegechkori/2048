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

    fn compactify(v: &Vec<i32>) -> Vec<i32> {
        let mut vec = Vec::new();
        for i in v {
            if *i != 0 {
                vec.push(*i);
            }
        }
        vec.resize(v.len(), 0);
        return vec;
    }

    fn combine_paired_cells(v: &mut Vec<i32>) -> Vec<i32> {
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
        return compactify(v);
    }

    #[test]
    fn test_compactify() {
        let v1 = vec![2, 0, 0, 0, 2, 0, 4, 0];
        assert_eq!(compactify(&v1), vec![2, 2, 4, 0, 0, 0, 0, 0]);

        let v2 = vec![0, 0, 2, 0, 2, 0, 4, 0];
        assert_eq!(compactify(&v2), vec![2, 2, 4, 0, 0, 0, 0, 0]);

        let v3 = vec![2, 2, 4, 0, 0, 0, 0, 0];
        assert_eq!(compactify(&v3), vec![2, 2, 4, 0, 0, 0, 0, 0]);

        let v4 = vec![2, 2, 4];
        assert_eq!(compactify(&v4), vec![2, 2, 4]);
    }
    #[test]
    fn test_combine_paired_cells() {
        let mut v1 = vec![2, 2, 2, 2, 4, 8, 0, 0];
        assert_eq!(combine_paired_cells(&mut v1), vec![4, 4, 4, 8, 0, 0, 0, 0]);

        let mut v2 = vec![2, 4, 8, 2, 4, 8, 2, 4];
        assert_eq!(combine_paired_cells(&mut v2), vec![2, 4, 8, 2, 4, 8, 2, 4]);

        let mut v3 = vec![2, 4, 4, 4, 0, 0, 0, 0];
        assert_eq!(combine_paired_cells(&mut v3), vec![2, 8, 4, 0, 0, 0, 0, 0]);
    }
}
