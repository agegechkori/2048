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
}
