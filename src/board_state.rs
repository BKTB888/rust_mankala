use std::fmt;
use std::fmt::Debug;
use colored::Colorize;

#[derive(Debug, Eq, Clone, Copy, Default)]
pub struct BoardState {
    board: [u8; 14],
    is_first_player: bool
}

impl BoardState {
    pub fn new() -> Self {
        BoardState {
            board: [
                6, 6, 6, 0, 6, 6, 6,
                6, 6, 6, 0, 6, 6, 6,
            ],
            is_first_player: true,
        }
    }

    pub fn from(custom_board: [u8; 14]) -> Self {
        BoardState {
            board: custom_board,
            is_first_player: true
        }
    }

    pub fn make_state(custom_board: [u8; 14], is_first_players_turn: bool) -> Self {
        BoardState {
            board: custom_board,
            is_first_player: is_first_players_turn
        }
    }

    pub fn make_move(&mut self, choice: u8) -> &mut Self {
        let mut idx = self.choice_to_idx(choice) as usize;

        loop {

            let mut hand = self.board[idx];
            self.board[idx] = 0;

            while hand > 0 {
                idx += 1;
                idx %= 14;
                self.board[idx] += 1;

                hand -= 1;
            }

            if idx == 3 || idx == 10 || self.board[idx] == 1 {
                break;
            }
        }
        self.is_first_player = !self.is_first_player;
        self
    }

    fn choice_to_idx(&self, choice: u8) -> u8 {
       self.get_idx(choice, self.is_first_player)
    }

    const fn get_idx(&self, choice: u8, is_first_player: bool) -> u8 {
        const DECISION_MATRIX: [[u8; 6]; 2] = [
            [7, 8, 9, 11, 12, 13],
            [0, 1, 2, 4, 5, 6]
        ];
        DECISION_MATRIX[is_first_player as usize][choice as usize]
    }

    fn get_side(&self, is_first_player: bool) -> [u8; 6] {
        (0..6).into_iter()
            .map(|choice| self.get_idx(choice, is_first_player))
            .map(|idx| self.board[idx as usize])
            .collect::<Vec<_>>().try_into().unwrap()
    }
    pub fn current_side(&self) -> [u8; 6] {
        self.get_side(self.is_first_player)
    }

    pub fn opponent_side(&self) -> [u8; 6] {
        self.get_side(!self.is_first_player)
    }

    pub fn is_won(&self) -> bool {
        const ZERO_ARRAY: [u8; 6] = [0u8; 6];

        self.get_side(true) == ZERO_ARRAY || self.get_side(false) == ZERO_ARRAY
    }

    pub fn get_current_player(&self) -> bool {
        self.is_first_player
    }

    pub fn get_valid_choices(&self) -> Vec<u8> {
        self.current_side().iter()
            .enumerate()
            .filter(|(_, &c)| c != 0)
            .map(|(i, _)| i as u8)
        .collect::<Vec<_>>()
    }
}

impl fmt::Display for BoardState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.board.iter()
            .map(|&num| num.to_string() + " ")
            .enumerate()
            .map(|(index, value)| match index {
                0..3 | 4..7 => value.red(),
                7..10 | 11..14 => value.green(),
                3 | 10 => value.blue().bold(),
                _ => value.black()
            })
            .map(|colored|
                write!(f, "{}", colored)
            )
            .collect::<fmt::Result>()
    }
}
impl PartialEq for BoardState {
    fn eq(&self, other: &Self) -> bool {
        self.get_side(true) == other.get_side(true) &&
            self.get_side(false) == other.get_side(false) &&
            self.is_first_player == other.is_first_player
    }
}

#[cfg(test)]
mod board_state_tests {
    fn setup() -> [u8; 14] {
        [0; 14]
    }

    mod move_tests {
        use crate::board_state::board_state_tests::setup;
        use crate::board_state::BoardState;
        #[test]
        fn simple() {
            let mut board_array = setup();
            board_array[2] = 15;
            let mut board = BoardState::from(board_array);
            board.make_move(2);

            let result_array = [1u8; 14];
            assert_eq!(board, BoardState::make_state(result_array, false));
        }

        #[test]
        fn complex_moves() {
            let mut test = BoardState::new();

            test.make_move(5);
            assert_eq!(test.board, [7, 7, 7, 1, 7, 0, 1, 8, 8, 8, 2, 8, 1, 7 ]);

            test.make_move(3);
            assert_eq!(test.board, [8, 8, 8, 2, 8, 1, 1, 8, 8, 8, 2, 0, 2, 8]);

            test.make_move(1);
            assert_eq!(test.board, [0, 2, 11, 5, 1, 4, 4, 11, 11, 2, 5, 2, 4, 10]);
        }
    }

    mod is_won_tests {
        use crate::board_state::board_state_tests::setup;
        use crate::board_state::BoardState;

        #[test]
        fn simple() {
            let mut board_array = setup();
            let board1 = BoardState::from(board_array);
            assert!(board1.is_won());

            board_array[0] = 1;
            let board2 = BoardState::from(board_array);
            assert!(board2.is_won());

            board_array[0] = 0;
            board_array[9] = 1;
            assert!(BoardState::from(board_array).is_won());

            board_array[0] = 1;
            assert!(!BoardState::from(board_array).is_won());
        }
    }
}
