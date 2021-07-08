use crate::random::RandomNumberGenerator;
use crate::tile_generator::TileGenerator;

struct Board<R: RandomNumberGenerator> {
    cells: Vec<Vec<i32>>,
    generator: TileGenerator<R>,
}

impl<R: RandomNumberGenerator> Board<R> {}

fn transpose(cells: &mut Vec<Vec<i32>>) {
    for i in 0..cells.len() {
        for j in 0..i {
            let temp = cells[i][j];
            cells[i][j] = cells[j][i];
            cells[j][i] = temp;
        }
    }
}

fn compactify_row_left(v: &mut [i32]) {
    let first_empty_opt = find_first_empty(v);
    let mut first_empty = match first_empty_opt {
        Some(index) => index,
        None => return,
    };
    let mut current = first_empty + 1;
    while current < v.len() {
        while current < v.len() && v[current] == 0 {
            current += 1;
        }
        if current == v.len() {
            return;
        }
        v.swap(first_empty, current);
        first_empty += 1;
        current += 1;
    }
}

fn compactify_row_right(v: &mut [i32]) {
    let first_empty_opt = find_last_empty(v);
    let mut first_empty = match first_empty_opt {
        Some(index) => index,
        None => return,
    };
    let mut current: i32 = first_empty as i32 - 1;
    while current > -1 {
        while current > -1 && v[current as usize] == 0 {
            current -= 1;
        }
        if current == -1 {
            return;
        }
        v.swap(first_empty, current as usize);
        first_empty -= 1;
        current -= 1;
    }
}

fn find_first_empty(v: &[i32]) -> Option<usize> {
    return v.iter().position(|&x| x == 0);
}

fn find_last_empty(v: &[i32]) -> Option<usize> {
    return v.iter().rposition(|&x| x == 0);
}

#[test]
fn test_compactify_row_left() {
    let mut v1 = vec![2, 0, 0, 0, 2, 0, 4, 0];
    compactify_row_left(&mut v1);
    assert_eq!(v1, vec![2, 2, 4, 0, 0, 0, 0, 0]);

    let mut v2 = vec![0, 0, 2, 0, 2, 0, 4, 0];
    compactify_row_left(&mut v2);
    assert_eq!(v2, vec![2, 2, 4, 0, 0, 0, 0, 0]);

    let mut v3 = vec![2, 2, 4, 0, 0, 0, 0, 0];
    compactify_row_left(&mut v3);
    assert_eq!(v3, vec![2, 2, 4, 0, 0, 0, 0, 0]);

    let mut v4 = vec![2, 2, 4];
    compactify_row_left(&mut v4);
    assert_eq!(v4, vec![2, 2, 4]);

    let mut v5 = vec![0, 2, 0, 0, 0, 2, 4, 4, 0, 2, 0, 2, 2];
    compactify_row_left(&mut v5);
    assert_eq!(v5, vec![2, 2, 4, 4, 2, 2, 2, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_compactify_row_right() {
    let mut v1 = vec![2, 0, 0, 0, 2, 0, 4, 0];
    compactify_row_right(&mut v1);
    assert_eq!(v1, vec![0, 0, 0, 0, 0, 2, 2, 4]);

    let mut v2 = vec![0, 0, 2, 0, 2, 0, 4, 0];
    compactify_row_right(&mut v2);
    assert_eq!(v2, vec![0, 0, 0, 0, 0, 2, 2, 4]);

    let mut v3 = vec![2, 2, 4, 0, 0, 0, 0, 0];
    compactify_row_right(&mut v3);
    assert_eq!(v3, vec![0, 0, 0, 0, 0, 2, 2, 4]);

    let mut v4 = vec![2, 2, 4];
    compactify_row_right(&mut v4);
    assert_eq!(v4, vec![2, 2, 4]);

    let mut v5 = vec![0, 2, 0, 0, 0, 2, 4, 4, 0, 2, 0, 2, 2];
    compactify_row_right(&mut v5);
    assert_eq!(v5, vec![0, 0, 0, 0, 0, 0, 2, 2, 4, 4, 2, 2, 2]);
}

#[test]
fn test_transpose_square_matrix() {
    let mut v = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    ];
    transpose(&mut v);
    assert_eq!(
        v,
        vec![
            vec![1, 5, 9, 13],
            vec![2, 6, 10, 14],
            vec![3, 7, 11, 15],
            vec![4, 8, 12, 16]
        ]
    );
}

#[test]
fn test_transpose_identity_matrix() {
    let mut v = vec![
        vec![1, 0, 0, 0],
        vec![0, 1, 0, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 0, 1],
    ];
    transpose(&mut v);
    assert_eq!(
        v,
        vec![
            vec![1, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1]
        ]
    );
}
