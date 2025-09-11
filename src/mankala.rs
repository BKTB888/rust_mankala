use std::rc::Rc;
use colored::Colorize;
use crate::board_state::BoardState;
type Player = dyn Fn(&BoardState) -> u8;
#[derive(Clone)]
pub struct Mankala {
    board: BoardState,
    players: [Rc<Player>; 2],
}

impl Mankala {
    pub fn new(p1_logic: impl Fn(&BoardState) -> u8 + 'static, p2_logic: impl Fn(&BoardState) -> u8 + 'static) -> Self {
        Mankala {
            board: BoardState::new(),
            players: [Rc::new(p1_logic), Rc::new(p2_logic)]
        }
    }

    pub fn set_board(&mut self, board: BoardState) {
        self.board = board;
    }

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

    pub fn print_play(&mut self) -> bool {
        let mut i = 1;
        'game: loop {
            for player in &self.players {
                let choice = player(&self.board);
                println!("Player {}: {}", !self.board.get_current_player() as u8 + 1, choice + 1);
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

    pub fn stats(&self, num_of_games: u32) -> f64 {
        let mut first_player_won = 0;
        for _ in 0..num_of_games {
            if self.clone().play() {
                first_player_won += 1;
            }
        }
        first_player_won as f64 / num_of_games as f64
    }
    pub fn print_stats(&self, num_of_games: u32) {

        let now = std::time::Instant::now();

        let player1_percent = self.stats(num_of_games) * 100f64;

        let elapsed = now.elapsed();

        println!("{} win rate: {}%", "Player 1".red(),  player1_percent );
        println!("{} win rate: {}%", "Player 2".green(), 100f64 - player1_percent);
        println!("Time / game: {:.2?}", elapsed / num_of_games);
    }
}