use colored::{ColoredString, Colorize};
use dyn_clone::DynClone;
use crate::board_state::BoardState;
use rayon::prelude::*;
pub trait Player: Fn(&BoardState) -> u8 + DynClone + Send {}
//Solve current player problem
impl<T: Fn(&BoardState) -> u8 + Clone + Send> Player for T {}
dyn_clone::clone_trait_object!(Player);
#[derive(Clone)]
pub struct Mankala {
    board: BoardState,
    players: [Box<dyn Player>; 2],
}

impl Mankala {
    pub fn new(p1_logic: impl Player + 'static, p2_logic: impl Player + 'static) -> Self {
        Mankala {
            board: BoardState::new(),
            players: [Box::new(p1_logic), Box::new(p2_logic)]
        }
    }

    pub fn set_board(&mut self, board: BoardState) {
        self.board = board;
    }


    //Make current, player tied to board_state players
    pub fn play(&mut self) -> bool {
        'game: loop {
            for player in &self.players {
                self.board.make_move(player(&self.board));
                if self.board.is_won() {
                    break 'game;
                }
            }
        }
        !self.board.get_current_player()
    }
    fn get_colored(is_first: bool) -> ColoredString {
        if is_first {
            "Player 1".red()
        } else {
            "Player 2".green()
        }
    }
    pub fn print_play(&mut self) -> bool {
        println!("Starting Board: {}\n", self.board);
        let mut i = 1;
        'game: loop {
            for player in &self.players {
                let choice = player(&self.board);
                println!("{}: {}", Self::get_colored(self.board.get_current_player()), choice + 1);
                println!("{i}: {}\n", self.board);
                self.board.make_move(choice);
                if self.board.is_won() {
                    break 'game;
                }
            }
            i += 1;
        }
        let winner = !self.board.get_current_player();
        if winner {
            println!("{} won!", "Player 1".red());
        } else {
            println!("{} won!", "Player 2".green());
        }

        winner
    }

    pub fn stats(&self, num_of_games: u32, is_parallel: bool) -> f64 {
        return  if is_parallel {
            (0..num_of_games)
                .map(|_| self.clone() )
                .fold(Vec::with_capacity(num_of_games as usize), |mut acc, game| {
                    acc.push(game);
                    acc
                }).into_par_iter()
                .map( |mut game| {
                    game.play()
                }).filter(|&won| won).count()
        } else {
            (0..num_of_games).map(|_| {
                self.clone().play()
            }).filter(|&won| won)
                .count()
        } as f64 / num_of_games as f64
    }
    pub fn print_stats(&self, num_of_games: u32, is_parallel: bool)  {

        let now = std::time::Instant::now();

        let player1_percent = self.stats(num_of_games, is_parallel) * 100f64;

        let elapsed = now.elapsed();

        println!("\nFor {}:", if is_parallel {"Parallel"} else { "Sync" });
        println!("{} win rate: {}%", Self::get_colored(true),  player1_percent );
        println!("{} win rate: {}%", Self::get_colored(false), 100f64 - player1_percent);
        println!("Time / game: {:.2?}", elapsed / num_of_games);
        println!("Elapsed time: {:.2?}", elapsed);
    }
}