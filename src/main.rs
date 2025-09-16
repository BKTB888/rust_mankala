use colored::Colorize;
use rand::{random, Rng};
use crate::board_state::BoardState;
use crate::mankala::{Mankala, Player};
/*
make min max random,
    make it useful for odd numbers,
    minimize recalculation

 */
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
    print!(
        "{}",
        if board.get_current_player() {
            "Player 1's choice: ".red()
        } else {
            "Player 2's choice: ".green()
        }
    );
    let choice= loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line. Please try again.");

        let number: Result<u8, _> = input.trim().parse();
        let choice = number.unwrap_or(7) - 1;

        if valid_choices.contains(&choice) {
            break choice;
        } else {
            println!("Invalid choice. Please try again.");
        }
    };
    println!("Result: {}\n", board.clone().make_move(choice));
    choice
}


fn ai_creator(eval: fn(&BoardState) -> f32) -> impl Player{
    move |board| {
        board.get_valid_choices().iter()
            .map(|&choice| (choice, *board.clone().make_move(choice)))
            .map(|(choice, board)| (choice, eval(&board)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(choice, _)| choice)
            .unwrap()
    }
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
            .map(|choice| *board.clone().make_move(choice))
            .map(|board| recursive(&board, depth - 1, !my_turn, zero_depth_eval))
            .min_by(|a, b| a.partial_cmp(&b).unwrap())
            .unwrap()
    }

    recursive(board, depth, true, zero_depth_eval)
}

fn rand_eval(_: &BoardState) -> f32 {
    random()
}


fn main() {
    //let mut mankala = Mankala::new(human, ai_creator(12));
    //mankala.set_board(BoardState::from([1; 14]));
    //mankala.print_play();
    /*
    Mankala::new(
        randy,
        ai_creator(base_eval),
    ).print_stats(10000);
    Mankala::new(
        randy,
        ai_creator(stupid_eval),
    ).print_stats(100);

     */


    for is_parallel in [true, false] {
        println!("\nFor {}:", if is_parallel {"Parallel"} else { "Sync" });
        Mankala::new(
            ai_creator(|board| {
                minimax(board, 6, stupid_eval)
            }),
            ai_creator(|board| {
                minimax(board, 6, rand_eval)
            }),
        ).print_stats(100, is_parallel)
    };
    /*
    Mankala::new(
        human,
        ai_creator(|board| {
            minimax(board, 11, stupid_eval)
        })
    ).print_play();

     */
}
