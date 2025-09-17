use std::io::Write;
use std::time::Duration;
use colored::Colorize;
use partial_application::partial;
use rand::{random, Rng};
use crate::board_state::BoardState;
use crate::mankala::{Mankala};
use rayon::prelude::*;

macro_rules! ai_creator {
    ($eval_func:expr, $with_parallel:expr) => {
        if $with_parallel {
            |board: &BoardState| {
                board.get_valid_choices()
                    .into_par_iter()
                    .map(|choice| (choice, board.clone()))
                    .map(|(choice, mut board)| {
                        board.make_move(choice);
                        (choice, board)
                    })
                    .map(|(choice, board)| (choice, $eval_func(&board)))
                    .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                    .map(|(choice, _)| choice)
                    .unwrap()
            }
        } else {
            |board: &BoardState| {
                board.get_valid_choices()
                    .into_iter()
                    .map(|choice| (choice, board.clone()))
                    .map(|(choice, mut board)| {
                        board.make_move(choice);
                        (choice, board)
                    })
                    .map(|(choice, board)| (choice, $eval_func(&board)))
                    .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                    .map(|(choice, _)| choice)
                    .unwrap()
            }
        }
    };
}
pub mod mankala;
pub mod board_state;

fn randy(board: &BoardState) -> u8 {
    let valid_choices = board.get_valid_choices();
    let index = rand::rng().random_range(0..valid_choices.len());
    valid_choices[index]
}

fn human(board: &BoardState) -> u8 {
    println!("{}", board);
    let valid_choices = board.get_valid_choices();
    println!("Available choices: {:?}", valid_choices.iter().map(|i| i + 1).collect::<Vec<_>>());
    let choice= loop {
        print!(
            "{}",
            if board.get_current_player() {
                "Player 1's choice: ".red()
            } else {
                "Player 2's choice: ".green()
            }
        );
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .unwrap_or_else(
                |_| {
                    println!("Failed to read line. Please try again.");
                    100
                }
            );

        let number: Result<u8, _> = input.trim().parse();
        let choice = number.unwrap_or(7) - 1;

        if valid_choices.contains(&choice) {
            break choice;
        } else {
            println!("Invalid choice. Please try again.");
        }
    };
    let mut copy = board.clone();
    copy.make_move(choice);
    println!("Result: {}\n", copy);
    choice
}
fn base_eval(board: &BoardState) -> f32 {
    let balls_at_op: u8 = board.opponent_side().iter().sum();
    let balls_at_me: u8 = board.current_side().iter().sum();
    let sum = (balls_at_op + balls_at_me) as f32;

    (balls_at_op as f32) / sum
}

fn stupid_eval(_: &BoardState) -> f32 {
    0.5
}

fn minimax(board: &BoardState, depth: u8, zero_depth_eval: fn(&BoardState) -> f32) -> f32 {

    fn recursive(board: &BoardState, depth: u8, my_turn: bool, zero_depth_eval: fn(&BoardState) -> f32) -> f32 {
        if board.is_won() {
            return 0.0
        }

        if depth == 0 {
            return zero_depth_eval(board)
        }

        1.0 - board.get_valid_choices().into_iter()
            .map(|choice| (choice, board.clone()))
            .map(|(choice, mut  board)| {
                board.make_move(choice);
                board
            })
            .map(|board| recursive(&board, depth - 1, !my_turn, zero_depth_eval))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }

    recursive(board, depth, true, zero_depth_eval)
}

fn rand_eval(_: &BoardState) -> f32 {
    random()
}
fn iterative_deepening_eval(depth_eval: impl Fn(&BoardState, u8) -> f32, time_constraint: Duration) -> impl Fn(&BoardState)-> f32 {
    move |board| {
        let start = std::time::Instant::now();
        let mut result = depth_eval(board, 0);
        let mut depth = 0;
        while time_constraint  > start.elapsed() {
            depth += 1;
            result = depth_eval(board, depth);
            if (result - 1.0).abs() < f32::EPSILON || result < f32::EPSILON {
                break;
            }
        }
        println!("Depth: {depth}, Result: {result}");
        result
    }
}

fn main() {
    Mankala::new(
        human,
        ai_creator!(
            iterative_deepening_eval(
                partial!(minimax => _, _, stupid_eval),
                Duration::from_secs(10)
            ),
            true
        )
    ).print_play();
}
