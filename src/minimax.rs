use crate::constants;

use pleco::{Board,Player,PieceType,SQ,BitMove};
use std::cmp::max;
use std::cmp::min;
use constants::{NUM_TURNS_TO_LOOK_AHEAD, ROOK_SCORE_GRID, PAWN_SCORE_GRID, BISHOP_SCORE_GRID, KNIGHT_SCORE_GRID, QUEEN_SCORE_GRID, KING_SCORE_GRID, EMPTY_SCORE_GRID, square_to_int, get_piece_value};

// returns the best move as a string for the current player in the given board state
pub fn calculate_next_move(mut board: Board) -> String {
    let next_move = minimax(board.clone(), NUM_TURNS_TO_LOOK_AHEAD);
    next_move.to_string().parse().unwrap()
}

// takes in a board and returns the best move to make for the current player
pub fn minimax(mut board: Board, depth: u8) -> BitMove {
    let possible_moves = board.generate_moves();
    let mut alpha = -9999999;
    let mut beta = 9999999;

    if possible_moves.len() == 0 {
        panic!("No possible moves for this position");
    }

    let mut best_move : BitMove = possible_moves[0];
    let curr_player = board.turn();

    for curr_move in possible_moves {
        //make move
        board.apply_move(curr_move);
        //evaluate
        let score = minimax_helper(board.clone(), depth - 1, alpha, beta);
        //undo move
        board.undo_move();

        if curr_player == Player::White && score > alpha {
            alpha = score;
            best_move = curr_move;
        }
        else if curr_player == Player::Black && score < beta {
            beta = score;
            best_move = curr_move;
        }
    }
    println!("alpha: {}, beta: {}", alpha, beta);
    best_move
}

fn minimax_helper(mut board: Board, depth: u8, mut alpha: i32, mut beta: i32) -> i32 {
    if board.stalemate() {
        return 0;
    }
    else if board.checkmate() {
        if board.turn() == Player::White {
            return -1000000;
        }
        else {
            return 1000000;
        }
    }

    if depth <= 0 {
        return evaluate(&board);
    }

    let curr_player = board.turn();
    let possible_moves = board.generate_moves();

    for curr_move in possible_moves {
        //make move
        board.apply_move(curr_move);
        let score = minimax_helper(board.clone(), depth - 1, alpha, beta);
        //undo move
        board.undo_move();

        //alpha beta pruning
        if curr_player == Player::White {
            alpha = max(alpha, score);

            if alpha >= beta {
                return alpha;
            }
        }
        else{
            beta = min(beta, score);

            if alpha >= beta {
                return beta;
            }
        }
    }

    if curr_player == Player::White {
        return alpha;
    }
    else {
        return beta;
    }
}

//takes in a board and returns its score
fn evaluate(board: &Board) -> i32 {
    if (*board).stalemate() {
        return 0;
    }
    else if (*board).checkmate() {
        if board.turn() == Player::White {
            return -1000000;
        }
        else {
            return 1000000;
        }
    }

    let mut score = 0i32;
    let piece_locations = (*board).get_piece_locations();

    for (sq, piece) in piece_locations {
        let player = match piece.player() {
            Some(x) => x,
            None => Player::White
        };

        score += get_piece_score_at_square(piece.type_of(), sq, player);
    }

    if (*board).in_check() {
        if board.turn() == Player::White {
            score -= 400;
        }
        else {
            score += 400;
        }
    }

    score
}

//gets score of a piece at a specific square
fn get_piece_score_at_square(piece: PieceType, square: SQ, player: Player) -> i32 {
    let score_grid = match piece {
        PieceType::R => ROOK_SCORE_GRID,
        PieceType::N => KNIGHT_SCORE_GRID,
        PieceType::B => BISHOP_SCORE_GRID,
        PieceType::Q => QUEEN_SCORE_GRID,
        PieceType::K => KING_SCORE_GRID,
        PieceType::P => PAWN_SCORE_GRID,
        _ => EMPTY_SCORE_GRID,
    };

    let piece_value = get_piece_value(piece);
    let mut index = square_to_int(square);

    let score_multiplier = match player {
        Player::White => 1,
        Player::Black => -1,
    };

    if player == Player::White {
        index = 63 - index;
    }

    //calculate score by adding the piece value to the score grid value
    (score_grid[index / 8][index % 8] + piece_value) * score_multiplier
}
