use colored::Colorize;
use rand::Rng;
use crate::board_state::BoardState;
use crate::mankala::Mankala;
/*
make min max random,
    make it useful for odd numbers,
    minimize recalculation

 */
pub mod mankala;
pub mod board_state;

fn randy(board: &BoardState) -> u8 {
    let valid_choices = board.get_valid_choices();
    let index = rand::thread_rng().gen_range(0..valid_choices.len());
    valid_choices[index]
}

fn human(board: &BoardState) -> u8 {
    println!("{}", board);
    let valid_choices = board.get_valid_choices();
    println!("Available choices: {:?}", valid_choices.iter().map(|i| i + 1).collect::<Vec<_>>());
    match board.get_current_player() {
        true => println!("{}", "Player 1's choice: ".red()),
        false => println!("{}", "Player 2's choice: ".green())
    }
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


fn ai_creator(depth: u8) -> impl Fn(&BoardState) -> u8 {
    move |board| {
        board.get_valid_choices().iter()
            .map(|&choice| (choice, *board.clone().make_move(choice)))
            .map(|(choice, board)| (choice, min_max(&board, depth)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(choice, _)| choice)
            .unwrap()
    }
}

fn min_max(board: &BoardState, depth: u8) -> f64 {

    fn recursive(board: &BoardState, depth: u8, use_max: bool) -> f64 {
        if board.is_won() {
            return if use_max {
                0.0
            } else {
                1.0
            }
        }

        if depth == 0 {
            let balls_at_op: u8 = board.opponent_side().iter().sum();
            let balls_at_me: u8 = board.current_side().iter().sum();
            return (balls_at_op as f64) / ((balls_at_op + balls_at_me) as f64);
        }

        let ratings = board.get_valid_choices().into_iter()
            .map(|choice| *board.clone().make_move(choice))
            .map(|board| recursive(&board, depth - 1, !use_max))
            .collect::<Vec<_>>();
        if use_max {
            *ratings.iter().max_by(|a, b| a.partial_cmp(&b).unwrap())
                .unwrap()
        }
        else {
            *ratings.iter().min_by(|a, b| a.partial_cmp(&b).unwrap())
                .unwrap()
        }
    }

    recursive(board, depth, true)
}

fn main() {
    let mut mankala = Mankala::new(randy, ai_creator(10));
    //mankala.set_board(BoardState::from([1; 14]));
    mankala.stats(10);
}
