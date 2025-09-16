use colored::Colorize;
use dyn_clone::DynClone;
use crate::board_state::BoardState;

pub trait Player: Fn(&BoardState) -> u8 + DynClone + Send{}

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

    pub fn stats(&self, num_of_games: u32, is_parallel: bool) -> f64 {
        if is_parallel {
            std::thread::scope(|s| {
                let handles = (0..num_of_games)
                    .map(|_|  self.clone() )
                    .map( |mut game| {
                        s.spawn( move ||
                            game.play()
                        )
                    }).collect::<Vec<_>>();
                handles.into_iter().map(|hande| hande.join().unwrap() )
                    .filter(|&won| won)
                    .count()
            }) as f64 / num_of_games as f64
        } else {
            let first_player_won = (0..num_of_games).map(|_| {
                self.clone().play()
            }).filter(|&won| won)
                .count();
            first_player_won as f64 / num_of_games as f64
        }
    }
    pub fn print_stats(&self, num_of_games: u32, is_async: bool)  {

        let now = std::time::Instant::now();

        let player1_percent = self.stats(num_of_games, is_async) * 100f64;

        let elapsed = now.elapsed();

        println!("{} win rate: {}%", "Player 1".red(),  player1_percent );
        println!("{} win rate: {}%", "Player 2".green(), 100f64 - player1_percent);
        println!("Time / game: {:.2?}", elapsed / num_of_games);
        print!("Elapsed time: {:.2?}", elapsed);
    }
}