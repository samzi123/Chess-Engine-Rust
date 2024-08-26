// This file is used for testing purposes. It is not used in the lambda function.
use pleco::{Board,Player,PieceType,SQ,BitMove};
use crate::minimax::minimax;

//takes in a board, and prints the board to the console. Used for debugging
#[allow(dead_code)]
pub fn draw_board(board_obj : Board) {
    let board_str = board_obj.fen();
    let mut board = [['\0'; 8]; 8];

    let lines = board_str.split("/");
    let mut char_index = 0usize;

    //iterate through each line and fill the board
    for line in lines {
        for ch in line.chars(){
            if char_index >= 64 {
                break;
            }

            if ch.is_numeric() {
                let num_empty_squares = ch as usize - '0' as usize;

                for _ in 0..num_empty_squares {
                    board[char_index / 8][char_index % 8] = ' ';
                    char_index += 1;
                }
            }
            else{
                board[char_index / 8][char_index % 8] = ch;
                char_index += 1;
            }
        }
    }

    //iterate and print board
    for i in 0..8 {
        for j in 0..8 {
            print!("{}, ", board[i][j]);
        }
        println!("");
    }
}

// call to make the bot play against itself. Used for testing
pub fn play_against_itself(mut board: Board, depth: u8) {
    let mut total_time_taken = 0;

    while board.checkmate() == false && board.stalemate() == false {
        let start_time = std::time::Instant::now();
        let next_move = minimax(board.clone(), depth);
        let time_taken_to_find_move = start_time.elapsed();
        total_time_taken += time_taken_to_find_move.as_millis();

        println!("{} for {}", next_move, board.turn());
        board.apply_move(next_move);
        draw_board(board.clone());
        println!();
    }

    if board.checkmate(){
        if board.turn() == Player::Black {
            println!("White wins!");
        }
        else {
            println!("Black wins!");
        }
    }
    else if board.stalemate() {
        println!("Stalemate :(");
    }
    else {
        println!("Game didn't end in checkmate or stalemate (this shouldn't happen)");
    }

    println!("Total time taken: {} ms", total_time_taken);
}