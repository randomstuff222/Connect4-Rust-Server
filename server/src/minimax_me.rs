use super::interface_me;
use crate::board_me;
use crate::board_me::{Board, WIDTH};
use crate::interface_me::Move;
//use pancurses::Window;
use rand::Rng;
use std::collections::VecDeque;

pub struct MinimaxBotMe {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct BotMove {
    confidence: u8,
    position: u8,
}

const MAX_DEPTH: u8 = 4;

// fn is_full(game_state: u128) -> bool{
//     // let target_state: u128 = 0b111111111111111111111111111111111111111111111111111111111111;
//     // if game_state == target_state {
//     //     return true;
//     // }
//     if game_state > 1 {
//         return true;
//     }
//     false;
// }

impl MinimaxBotMe {
    fn max(
        serialized_board: u128, //current board state serializedn (on our code, we have it as integer, we can convert it into this)
        player_color: usize, // originally was player_color: Token, however working only for an array, we need to change it like this
        opponent_color: usize, // opponent_color: Token, // same as above
        depth: u8,
    ) -> BotMove {
        // vecdequeue of botMoves
        let mut moves = VecDeque::new();

        // for loop for each column in connect 4
        for x in 1..=WIDTH {
            //print!("Depth:{}, adding column {}\n", depth, x);

            // this is a copy of the board, using the serialized board, we convert it
            // into a normal board, we can try to keep it a serialized board and operate using it
            // since later on recursion, it will serialize this board again and pass it down
            //let mut board = serialized_board;
            let mut board = Board::from_number(serialized_board);

            // this if statement takes the board (not serialized, and adds a token on x
            // x being the current column)
            if board
                .add_token(x, player_color)
                .expect("Error adding token")
            {
            // if board
            //     .add_token(x, &player_color)
            //     .expect("Error adding token")
            // {
                if board.have_winner_at_column(x) {
                    // Victory, we immediately return
                    return BotMove {
                        position: x,
                        confidence: 100,
                    };
                }
                // this function checks if the array of vectors(board) is the same size as the spaces in the board
                //if is_full(serialized_board){
                if board.is_full() {
                    // Draw
                    moves.push_back(BotMove {
                        position: x,
                        confidence: 50,
                    });
                    continue;
                }
                if depth < MAX_DEPTH {
                    let recursive_move = MinimaxBotMe::min(
                        board.to_number(),
                        opponent_color.clone(),
                        player_color.clone(),
                        depth + 1,
                    );
                    if recursive_move.confidence == 100 {
                        return BotMove {
                            position: x,
                            confidence: 100,
                        };
                    }
                    moves.push_back(BotMove {
                        position: x,
                        confidence: recursive_move.confidence,
                    });
                } else {
                    moves.push_back(BotMove {
                        position: x,
                        confidence: 20 + rand::thread_rng().gen_range(1, 20),
                    });
                }
            }
        }
        // take the largest number on the vecDeque
        match moves.iter().max() {
            Some(m) => m.clone(),
            None => BotMove {
                position: 0,
                confidence: 0,
            },
        }
    }

    fn min(
        serialized_board: u128,
        player_color: usize, // originally was player_color: Token, however working only for an array, we need to change it like this
        opponent_color: usize, // opponent_color: Token, // same as above
        depth: u8,
    ) -> BotMove {
        let mut moves = VecDeque::new();

        for x in 1..=WIDTH {
            let mut board = Board::from_number(serialized_board);
            //print!("Min: Depth: {} Pos: {}\n", depth, x);
            if board
                .add_token(x, player_color)
                .expect("Error adding token")
            {
                if board.have_winner_at_column(x) {
                    // Victory, we immediately return
                    return BotMove {
                        position: x,
                        confidence: 0,
                    };
                }
                
                if board.is_full() {
                    // Draw
                    moves.push_back(BotMove {
                        position: x,
                        confidence: 50,
                    });
                    continue;
                }
                if depth < MAX_DEPTH {
                    let recursive_move = MinimaxBotMe::max(
                        board.to_number(),
                        opponent_color.clone(),
                        player_color.clone(),
                        depth + 1,
                    );
                    if recursive_move.confidence == 0 {
                        return BotMove {
                            position: x,
                            confidence: 0,
                        };
                    }
                    moves.push_back(BotMove {
                        position: x,
                        confidence: recursive_move.confidence,
                    });
                } else {
                    moves.push_back(BotMove {
                        position: x,
                        confidence: 20 + rand::thread_rng().gen_range(1, 20),
                    });
                }
            }
        }
        match moves.iter().min() {
            Some(m) => m.clone(),
            None => BotMove {
                position: 0,
                confidence: 0,
            },
        }
    }
}

impl interface_me::GameInterface for MinimaxBotMe {
    fn name(&self) -> String {
        String::from("Bot")
    }

    fn play(
        &self,
        board: &board_me::Board,
        color: usize,
    ) -> u8 {
        // let mut moves: VecDeque<BotMove> = VecDeque::new();
        let opponent_color = match color {
            2=> 1,
            1 => 2,
            _ => 0
        };

        let target_position =
            MinimaxBotMe::max(board.to_number(), color, opponent_color, 0).position;

        return target_position;
        // cursor position is the input of the user, it can be between 1 and the widith of the board
        // this is just to move the arrow into the target position

        // this section is for window ui interaction, not needed
        // if target_position > cursor_position {
        //     for _ in cursor_position..target_position {
        //         moves.push_back("Right");
        //     }
        // } else {
        //     for _ in target_position..cursor_position {
        //         moves.push_back("LEFT");
        //     }
        // }
        // // this is just to tell the code to drop the token
        // moves.push_back("DROP");
        // return moves;
    }
}

