use std::ops::IndexMut;
use std::ops::Index;

struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Matrix<T> {
    fn new(matrix_data: Vec<Vec<T>>) -> Matrix<T> {
        let rows = matrix_data.len();
        let cols = matrix_data[0].len();
        let mut data = vec![];
        for row in matrix_data {
            for col in row {
                data.push(col);
            }
        }
        return Matrix { data, rows, cols };
    }

    fn translate(&self, row: usize, col: usize) -> usize {
        return row * self.cols + col;
    }  
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        return &self.data[self.translate(index, 0)..self.translate(index, self.cols)];
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = self.translate(index, 0);
        let end = self.translate(index, self.cols);
        return &mut self.data[start..end];
    }
}

#[test]
fn test_new() {
    let m = Matrix::new(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]);
    assert_eq!(m.data, vec![1, 0, 0, 0, 1, 0, 0, 0, 1]);
    assert_eq!(m.rows, 3);
    assert_eq!(m.cols, 3);
}

#[test]
fn test_index() {
    let m = Matrix::new(vec![
        vec![1, 3, 5, 12, 18, 22],
        vec![7, 2, 9, 14, 15, 16],
        vec![13, 11, 17, 19, 20, 21],
        vec![0, 6, 4, 8, 10, 23],
    ]);

    assert_eq!(m[0][0], 1);
    assert_eq!(m[0][1], 3);
    assert_eq!(m[0][2], 5);
    assert_eq!(m[1][0], 7);
    assert_eq!(m[1][1], 2);
    assert_eq!(m[1][2], 9);
    assert_eq!(m[2][0], 13);
    assert_eq!(m[2][1], 11);
    assert_eq!(m[2][2], 17);
    assert_eq!(m[3][0], 0);
    assert_eq!(m[3][1], 6);
    assert_eq!(m[3][2], 4);
}

#[test]
fn test_index_mut() {
    let mut m = Matrix::new(vec![
        vec![1, 3, 5, 12, 18, 22],
        vec![7, 2, 9, 14, 15, 16],
        vec![13, 11, 17, 19, 20, 21],
        vec![0, 6, 4, 8, 10, 23],
    ]);

    m[0][0] = 10;
    m[0][1] = 30;
    m[0][2] = 50;
    m[1][0] = 70;
    m[1][1] = 20;
    m[1][2] = 90;
    m[2][0] = 130;
    m[2][1] = 110;
    m[2][2] = 170;
    m[3][0] = 100;
    m[3][1] = 60;
    m[3][2] = 40;

    assert_eq!(m[0][0], 10);
    assert_eq!(m[0][1], 30);
    assert_eq!(m[0][2], 50);
    assert_eq!(m[1][0], 70);
    assert_eq!(m[1][1], 20);
    assert_eq!(m[1][2], 90);
    assert_eq!(m[2][0], 130);
    assert_eq!(m[2][1], 110);
    assert_eq!(m[2][2], 170);
    assert_eq!(m[3][0], 100);
    assert_eq!(m[3][1], 60);
    assert_eq!(m[3][2], 40);
}

