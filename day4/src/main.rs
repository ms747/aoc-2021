fn board(board: &str) -> Vec<Vec<u32>> {
    let board: Vec<&str> = board.split('\n').collect();
    let board: Vec<_> = board
        .iter()
        .map(|line| {
            line.split(' ')
                .filter(|x| !(*x).is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    board
}

fn find_roll_index(roll: u32, board: &[Vec<u32>]) -> Option<(usize, usize)> {
    for row in 0..5 {
        for col in 0..5 {
            if board[row][col] == roll {
                return Some((row, col));
            }
        }
    }
    None
}

fn mark(bidx: usize, row: usize, col: usize, markers: &mut [Vec<Vec<bool>>]) {
    markers[bidx][row][col] = true;
}

/*
fn debug(boards: &[Vec<Vec<bool>>]) {
    for board in boards {
        for row in board {
            for col in row {
                print!("{col:7}");
            }
            println!();
        }
        println!();
    }
}
*/

fn check_bingo(markers: &[Vec<bool>]) -> bool {
    fn check_rows(markers: &[Vec<bool>]) -> bool {
        for row in 0..5 {
            let mut count = 0;
            for col in 0..5 {
                if markers[row][col] {
                    count += 1;
                }
            }
            if count == 5 {
                return true;
            }
        }
        false
    }

    fn check_cols(markers: &[Vec<bool>]) -> bool {
        for col in 0..5 {
            let mut count = 0;
            for row in 0..5 {
                if markers[row][col] {
                    count += 1;
                }
            }
            if count == 5 {
                return true;
            }
        }
        false
    }

    if check_rows(markers) {
        return true;
    }
    check_cols(markers)
}

fn sum_unmarked(board: &[Vec<u32>], marker: &[Vec<bool>]) -> u32 {
    let mut sum = 0;
    for row in 0..5 {
        for col in 0..5 {
            if !marker[row][col] {
                sum += board[row][col];
            }
        }
    }
    sum
}

fn part1(mut deck: Vec<u32>, boards: &[Vec<Vec<u32>>]) -> u32 {
    let mut markers = vec![vec![vec![false; 5]; 5]; boards.len()];
    let mut result = 0;

    'bingo: while !deck.is_empty() {
        let roll = deck.remove(0);
        for (idx, board) in boards.iter().enumerate() {
            if let Some((row, col)) = find_roll_index(roll, board) {
                mark(idx, row, col, &mut markers);
            }

            if check_bingo(&markers[idx]) {
                result = sum_unmarked(board, &markers[idx]) * roll;
                break 'bingo;
            }
        }
    }

    result
}

fn part2(mut deck: Vec<u32>, boards: &[Vec<Vec<u32>>]) -> u32 {
    let mut markers = vec![vec![vec![false; 5]; 5]; boards.len()];
    let mut done_boards = vec![false; boards.len()];

    let mut last_done_board = 0;
    let mut last_roll = 0;

    loop {
        let roll = deck.remove(0);
        for (idx, board) in boards.iter().enumerate() {
            if let Some((row, col)) = find_roll_index(roll, board) {
                if !done_boards[idx] {
                    mark(idx, row, col, &mut markers);
                }
            }

            if check_bingo(&markers[idx]) && !done_boards[idx] {
                done_boards[idx] = true;
                last_done_board = idx;
                last_roll = roll;
            }
        }

        let done = done_boards.iter().filter(|board| **board).count();

        if done == boards.len() {
            break;
        }
    }
    let sum_of_last_board = sum_unmarked(&boards[last_done_board], &markers[last_done_board]);
    sum_of_last_board * last_roll
}

use std::time::Instant;

fn main() {
    let input: Vec<&str> = include_str!("input.txt").split("\n\n").collect();

    let deck1: Vec<u32> = input[0]
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let deck2 = deck1.clone();

    let boards: Vec<_> = input[1..].iter().map(|x| board(x.trim())).collect();

    // Part 1
    let it = Instant::now();
    let result = part1(deck1, &boards);
    println!("Part 1: {result} Time: {:?}", it.elapsed());

    // Part 2
    let it = Instant::now();
    let result = part2(deck2, &boards);
    println!("Part 2: {result} Time: {:?}", it.elapsed());
}
