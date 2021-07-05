use crate::tile_generator::RandomTileGenerator;
use crate::random::create_simple_generator;
use crate::random::RandomNumberGenerator;
use mockall::predicate::*;
use mockall::*;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn shift_board(board: &Vec<Vec<i32>>, direction: Direction) -> (Vec<Vec<i32>>, i32) {
    return match direction {
        Direction::Left => shift_board_left(&board),
        Direction::Right => shift_board_right(&board),
        Direction::Up => shift_board_up(&board),
        Direction::Down => shift_board_down(&board),
    };
}

fn shift_board_left(v: &Vec<Vec<i32>>) -> (Vec<Vec<i32>>, i32) {
    let mut vec = <Vec<Vec<i32>>>::with_capacity(v.len());
    let mut score = 0;
    for i in v {
        let (row, row_score) = shift_row_left(i);
        vec.push(row);
        score += row_score;
    }
    return (vec, score);
}

fn shift_board_right(v: &Vec<Vec<i32>>) -> (Vec<Vec<i32>>, i32) {
    let mut vec = v.clone();
    reverse_rows(&mut vec);

    let (mut shifted, score) = shift_board_left(&vec);
    reverse_rows(&mut shifted);
    return (shifted, score);
}

fn shift_board_up(v: &Vec<Vec<i32>>) -> (Vec<Vec<i32>>, i32) {
    let (new_board, score) = shift_board_left(&transpose(&v.clone()));
    return (transpose(&new_board), score);
}

fn shift_board_down(v: &Vec<Vec<i32>>) -> (Vec<Vec<i32>>, i32) {
    let (new_board, score) = shift_board_right(&transpose(&v.clone()));
    return (transpose(&new_board), score);
}

fn reverse_rows(v: &mut Vec<Vec<i32>>) {
    for i in v {
        i.reverse();
    }
}

fn shift_row_left(v: &Vec<i32>) -> (Vec<i32>, i32) {
    let mut vec = compactify_row(v);
    return combine_paired_cells_in_row(&mut vec);
}

fn shift_row_right(v: &Vec<i32>) -> (Vec<i32>, i32) {
    let mut vv = v.clone();
    vv.reverse();
    let (mut vec, score) = shift_row_left(&vv);
    vec.reverse();
    return (vec, score);
}

fn combine_paired_cells_in_row(v: &mut Vec<i32>) -> (Vec<i32>, i32) {
    let mut i = 0;
    let mut score = 0;
    while i < v.len() - 1 && v[i] != 0 {
        if v[i] == v[i + 1] {
            v[i] += v[i + 1];
            v[i + 1] = 0;
            score += v[i];
            i += 2;
        } else {
            i += 1
        }
    }
    return (compactify_row(v), score);
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

fn create_random_tile<R: RandomNumberGenerator>(
    v: &Vec<Vec<i32>>,
    generator: &mut RandomTileGenerator<R>,
) -> Vec<Vec<i32>> {
    let empty_cells = select_empty_cells(&v);
    let mut vec = v.clone();
    let (i, j, tile) = generator.generate_tile(&empty_cells);
    vec[i][j] = tile;
    return vec;
}

fn select_empty_cells(v: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut empty_cells = vec![];
    for i in 0..v.len() {
        for j in 0..v[i].len() {
            if v[i][j] == 0 {
                empty_cells.push((i, j));
            }
        }
    }
    return empty_cells;
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
    assert_eq!(shift_board(&v1, Direction::Left), (expected_left, 20));

    let expected_right = vec![
        vec![0, 0, 0, 4],
        vec![0, 0, 8, 2],
        vec![0, 0, 4, 4],
        vec![2, 4, 2, 4],
    ];
    assert_eq!(shift_board(&v1, Direction::Right), (expected_right, 20));

    let expected_up = vec![
        vec![4, 4, 2, 4],
        vec![2, 2, 4, 4],
        vec![0, 4, 4, 0],
        vec![0, 0, 0, 0],
    ];
    assert_eq!(shift_board(&v1, Direction::Up), (expected_up, 12));

    let expected_down = vec![
        vec![0, 0, 0, 0],
        vec![0, 4, 2, 0],
        vec![2, 2, 4, 4],
        vec![4, 4, 4, 4],
    ];
    assert_eq!(shift_board(&v1, Direction::Down), (expected_down, 12));
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
    assert_eq!(shift_board_left(&v1), (expected, 20));
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
    assert_eq!(shift_board_right(&v1), (expected, 20));
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
    assert_eq!(shift_board_up(&v1), (expected, 12));
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
    assert_eq!(shift_board_down(&v1), (expected, 12));
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
    assert_eq!(shift_row_left(&v1), (vec![4, 4, 0, 0, 0, 0, 0, 0], 4));

    let v2 = vec![2, 4, 8, 2, 4, 8, 2, 4];
    assert_eq!(shift_row_left(&v2), (vec![2, 4, 8, 2, 4, 8, 2, 4], 0));

    let v3 = vec![2, 2, 2, 2, 8, 4, 4, 2];
    assert_eq!(shift_row_left(&v3), (vec![4, 4, 8, 8, 2, 0, 0, 0], 16));

    let v4 = vec![0, 0, 0, 2, 2, 2, 2, 4];
    assert_eq!(shift_row_left(&v4), (vec![4, 4, 4, 0, 0, 0, 0, 0], 8));
}

#[test]
fn test_shift_row_right() {
    let v1 = vec![2, 0, 0, 0, 2, 0, 4, 0];
    assert_eq!(shift_row_right(&v1), (vec![0, 0, 0, 0, 0, 0, 4, 4], 4));

    let v2 = vec![2, 4, 8, 2, 4, 8, 2, 4];
    assert_eq!(shift_row_right(&v2), (vec![2, 4, 8, 2, 4, 8, 2, 4], 0));

    let v3 = vec![2, 2, 2, 2, 8, 4, 4, 2];
    assert_eq!(shift_row_right(&v3), (vec![0, 0, 0, 4, 4, 8, 8, 2], 16));

    let v4 = vec![0, 2, 0, 2, 2, 2, 2, 4];
    assert_eq!(shift_row_right(&v4), (vec![0, 0, 0, 0, 2, 4, 4, 4], 8));
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
        (vec![4, 4, 4, 8, 0, 0, 0, 0], 8)
    );

    let mut v2 = vec![2, 4, 8, 2, 4, 8, 2, 4];
    assert_eq!(
        combine_paired_cells_in_row(&mut v2),
        (vec![2, 4, 8, 2, 4, 8, 2, 4], 0)
    );

    let mut v3 = vec![2, 4, 4, 4, 0, 0, 0, 0];
    assert_eq!(
        combine_paired_cells_in_row(&mut v3),
        (vec![2, 8, 4, 0, 0, 0, 0, 0], 8)
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
fn test_select_empty_cells() {
    let board = vec![
        vec![4, 0, 0, 0],
        vec![8, 2, 0, 0],
        vec![4, 4, 0, 0],
        vec![2, 4, 2, 4],
    ];

    assert_eq!(
        select_empty_cells(&board),
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 2), (2, 3)]
    );
}

// TODO: reimplement test with mocks
// #[test]
// fn test_create_random_tile() {
//     let mut random_tile_generator = RandomTileGenerator::new(
//         vec![
//             TileOption {
//                 value: 2,
//                 probability: 10,
//             },
//             TileOption {
//                 value: 4,
//                 probability: 20,
//             },
//             TileOption {
//                 value: 8,
//                 probability: 30,
//             },
//             TileOption {
//                 value: 16,
//                 probability: 40,
//             },
//         ],
//         create_simple_generator(),
//     )
//     .unwrap();

//     let board = vec![
//         vec![4, 0, 0, 0],
//         vec![8, 2, 0, 0],
//         vec![4, 4, 0, 0],
//         vec![2, 4, 2, 4],
//     ];

//     let empty_cells = select_empty_cells(&board);
//     let new_board = create_random_tile(&board, &mut random_tile_generator);
//     let mut tile_row = 0;
//     let mut tile_col = 0;
//     for row in 0..board.len() {
//         for col in 0..board[0].len() {
//             if board[row][col] != new_board[row][col] {
//                 tile_row = row;
//                 tile_col = col;
//                 break;
//             }
//         }
//     }

//     assert!(empty_cells.contains(&(tile_row, tile_col)));
//     assert!(vec![2, 4, 8, 16].contains(&new_board[tile_row][tile_col]));
// }
