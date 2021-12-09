use crate::utils;
mod board;
use board::{Board, Cell};
use std::collections::VecDeque;

pub fn solve(input1: String, _: String, _: &[String]) {
    let mut lines: VecDeque<String> = VecDeque::from(utils::read_file_lines(&input1));
    let bingo_call_line = lines.pop_front().unwrap();
    let bingo_call = get_bingo_call(&bingo_call_line);

    lines.drain(0..=0);
    let mut boards = get_boards(&lines);
    
    // println!("Bingo call: {:?}", bingo_call);
    // println!("Boards {:?}", boards);

    let mut winner_found = false;

    for num in bingo_call.iter() {
        boards.iter_mut().for_each(|b| b.check_mark(num));
        let board_clone = boards.clone();
        let complete = board_clone.iter()
            .filter(|b| b.is_finished())
            .collect::<Vec<_>>();

        if !winner_found && !complete.is_empty() {
            winner_found = true;
            let winner = complete.get(0).unwrap();
            report_board(winner, num, true);
        }

        if boards.len() == 1 && complete.len() == 1 {
            let loser = boards.get(0).unwrap();
            report_board(loser, num, false);
            return
        }

        if !complete.is_empty() {
            boards.retain(|b| !complete.contains(&b))
        }
    }
}

fn get_bingo_call(line: &String) -> Vec<String> {
    line.split(',')
        .map(str::to_string)
        .collect::<Vec<_>>()
}

fn get_boards(lines: &VecDeque<String>) -> Vec<Board> {
    let mut boards = Vec::new();

    let mut board: Vec<Vec<Cell>> = Vec::new();
    for l in lines {
        if l.is_empty() {
            boards.push(Board::new(board));
            board = Vec::new();
        } else {
            let row = l.trim().split_whitespace()
                .map(str::to_string)
                .map(Cell::new)
                .collect::<Vec<_>>();
            board.push(row);
        }
    }

    if !board.is_empty() {
        boards.push(Board::new(board));
    }

    boards
}

fn report_board(board: &Board, num: &String, winner: bool) {
    let num_parsed = num.parse::<i32>().unwrap();
    let unchecked_sum = board.unchecked_sum();
    println!("{} board with result {} * {} = {}",
        if winner { "Winning" } else { "Losing" },
        num_parsed, unchecked_sum, num_parsed * unchecked_sum);
}