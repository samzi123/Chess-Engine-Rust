use pleco::{Board,Player,PieceType,SQ,BitMove};
use std::cmp::max;
use std::cmp::min;

use dotenv::dotenv;
use std::env;

use reqwest::Error;
use std::time::Duration;

use serde_json::Value;

const NUM_TURNS_TO_LOOK_AHEAD: u8 = 3;

// #[allow(dead_code)]
// #[tokio::main]
// async fn main() {
//     let board = Board::start_pos();
//     play_against_itself(board.clone(), 3);
// }

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    //respond_to_all_challenges().await?;
    make_moves_in_all_ongoing_games().await?;

    Ok(())
}

// finds all current games of the bot and makes a move in those where it is the bot's turn
async fn make_moves_in_all_ongoing_games() -> Result<(), Error> {
    let LICHESS_ACCESS_TOKEN = env::var("RUST_BOT_TOKEN").expect("You need to set a Lichess personal access token.");

    let ongoing_games = get_ongoing_games().await?;
    println!("ongoing_games: {}", ongoing_games);
    let ongoing_games_object: Value = serde_json::from_str(&ongoing_games).unwrap();
    let game_array_json = &ongoing_games_object["nowPlaying"];

    if game_array_json == &Value::Null && game_array_json == &Value::Array(vec![]) {
        println!("No ongoing games");
        return Ok(());
    }

    // loop through all current games and make a move in each
    if let Some(games) = game_array_json.as_array() {
        for game in games {
            let game_id = &game["gameId"].as_str().unwrap();
            let fen = &game["fen"].as_str().unwrap();
            let is_my_turn = &game["isMyTurn"].as_bool().unwrap();

            if *is_my_turn == true {
                let board = Board::from_fen(fen).unwrap();
                let next_move = calculate_next_move(board);
                println!("making move: {}", next_move);

                make_move(&game_id, &next_move).await?;
            }
        }
    }

    Ok(())
}

// calls API to make a move in the given game. Move should be a string in UCI format, e.g. "e2e4"
async fn make_move(game_id: &str, move_to_play: &str) -> Result<String, Error> {
    let LICHESS_ACCESS_TOKEN = env::var("RUST_BOT_TOKEN").expect("You need to set a Lichess personal access token.");
    let URL = "https://lichess.org/api/bot/game/".to_owned() + game_id + "/move/" + move_to_play;

    let client = reqwest::Client::new();
    let response = client
        .post(URL)
        .header("Authorization", "Bearer ".to_owned() + LICHESS_ACCESS_TOKEN.as_str())
        .timeout(Duration::from_secs(5))
        .send()
        .await?
        .text()
        .await?;

    println!("response after making a move: {}", response);

    Ok(response)
}

// fetches all current challenges for the bot, and accepts all that are at least 1 day per turn. It declines the rest
async fn respond_to_all_challenges() -> Result<(), Error> {
    let LICHESS_ACCESS_TOKEN = env::var("RUST_BOT_TOKEN").expect("You need to set a Lichess personal access token.");

    let current_challenges = list_current_challenges().await?;
    let challenges_object: Value = serde_json::from_str(&current_challenges).unwrap();
    
    let challenges_received = &challenges_object["in"];
    if challenges_received == &Value::Null && challenges_received == &Value::Array(vec![]) {
        println!("No challenges received");
        return Ok(());
    }

    // loop through challenges_received and accept all that are at least 1 day per turn
    if let Some(challenges) = challenges_received.as_array() {
        for challenge in challenges {
            let challenge_id = &challenge["id"].as_str().unwrap();
            println!("challenge_id: {}", challenge_id);

            let days_per_turn = &challenge["timeControl"]["daysPerTurn"];
            println!("days_per_turn: {}", days_per_turn);
            
            if days_per_turn == &Value::Null {
                // decline challenge as only accepting games with at least 1 day per turn
                println!("declining challenge");
                decline_challenge(challenge_id).await?;
            }
            else {
                // accept challenge
                println!("accepting challenge - need to re-enable");
                accept_challenge(challenge_id).await?;
            }
        }
    }

    Ok(())
}

// fetches all ongoing games for the bot
async fn get_ongoing_games() -> Result<String, Error> {
    let LICHESS_ACCESS_TOKEN = env::var("RUST_BOT_TOKEN").expect("You need to set a Lichess personal access token.");

    let client = reqwest::Client::new();
    let response = client
        .get("https://lichess.org/api/account/playing")
        .header("Authorization", "Bearer ".to_owned() + LICHESS_ACCESS_TOKEN.as_str())
        .timeout(Duration::from_secs(5))
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

// accepts a challenge with the given challenge ID
async fn accept_challenge(challenge_id: &str) -> Result<(), Error> {
    let LICHESS_ACCESS_TOKEN = env::var("RUST_BOT_TOKEN").expect("You need to set a Lichess personal access token.");
    let URL = "https://lichess.org/api/challenge/".to_owned() + challenge_id + "/accept";
    println!("URL: {}", URL);

    let client = reqwest::Client::new();
    let response = client
        .post(URL)
        .header("Authorization", "Bearer ".to_owned() + LICHESS_ACCESS_TOKEN.as_str())
        .timeout(Duration::from_secs(5))
        .send()
        .await?
        .text()
        .await?;

    println!("response: {}", response);

    Ok(())
}

// declines a challenge with the given challenge ID
async fn decline_challenge(challenge_id: &str) -> Result<(), Error> {
    let LICHESS_ACCESS_TOKEN = env::var("RUST_BOT_TOKEN").expect("You need to set a Lichess personal access token.");
    let URL = "https://lichess.org/api/challenge/".to_owned() + challenge_id + "/decline";
    println!("URL: {}", URL);

    let client = reqwest::Client::new();
    let response = client
        .post(URL)
        .header("Authorization", "Bearer ".to_owned() + LICHESS_ACCESS_TOKEN.as_str())
        .body("declineTooFast")
        .timeout(Duration::from_secs(5))
        .send()
        .await?
        .text()
        .await?;

    Ok(())
}

async fn list_current_challenges() -> Result<String, Error> {
    let LICHESS_ACCESS_TOKEN = env::var("RUST_BOT_TOKEN").expect("You need to set a Lichess personal access token.");

    let client = reqwest::Client::new();
    let response = client
        .get("https://lichess.org/api/challenge")
        .header("Authorization", "Bearer ".to_owned() + LICHESS_ACCESS_TOKEN.as_str())
        .timeout(Duration::from_secs(5))
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

// returns the best move as a string for the current player in the given board state
fn calculate_next_move(mut board: Board) -> String {
    let next_move = minimax(board.clone(), NUM_TURNS_TO_LOOK_AHEAD);
    next_move.to_string().parse().unwrap()
}

// call to make the bot play against itself. Used for testing
fn play_against_itself(mut board: Board, depth: u8) {
    while board.checkmate() == false && board.stalemate() == false {
        let next_move = minimax(board.clone(), depth);
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
}

// takes in a board and returns the best move to make for the current player
fn minimax(mut board: Board, depth: u8) -> BitMove {
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
        //println!("score: {}, player: {}, move: {}", score, board.turn(), curr_move);

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
        //println!("score: {}, player: {}, move: {}", score, board.turn(), curr_move);

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

//takes in a square and returns its index in the score grid, between 0 and 63
fn square_to_int(square: SQ) -> usize {
    match square {
        SQ::A1 => 0,
        SQ::B1 => 1,
        SQ::C1 => 2,
        SQ::D1 => 3,
        SQ::E1 => 4,
        SQ::F1 => 5,
        SQ::G1 => 6,
        SQ::H1 => 7,

        SQ::A2 => 8,
        SQ::B2 => 9,
        SQ::C2 => 10,
        SQ::D2 => 11,
        SQ::E2 => 12,
        SQ::F2 => 13,
        SQ::G2 => 14,
        SQ::H2 => 15,

        SQ::A3 => 16,
        SQ::B3 => 17,
        SQ::C3 => 18,
        SQ::D3 => 19,
        SQ::E3 => 20,
        SQ::F3 => 21,
        SQ::G3 => 22,
        SQ::H3 => 23,
        
        SQ::A4 => 24,
        SQ::B4 => 25,
        SQ::C4 => 26,
        SQ::D4 => 27,
        SQ::E4 => 28,
        SQ::F4 => 29,
        SQ::G4 => 30,
        SQ::H4 => 31,

        SQ::A5 => 32,
        SQ::B5 => 33,
        SQ::C5 => 34,
        SQ::D5 => 35,
        SQ::E5 => 36,
        SQ::F5 => 37,
        SQ::G5 => 38,
        SQ::H5 => 39,

        SQ::A6 => 40,
        SQ::B6 => 41,
        SQ::C6 => 42,
        SQ::D6 => 43,
        SQ::E6 => 44,
        SQ::F6 => 45,
        SQ::G6 => 46,
        SQ::H6 => 47,
        
        SQ::A7 => 48,
        SQ::B7 => 49,
        SQ::C7 => 50,
        SQ::D7 => 51,
        SQ::E7 => 52,
        SQ::F7 => 53,
        SQ::G7 => 54,
        SQ::H7 => 55,

        SQ::A8 => 56,
        SQ::B8 => 57,
        SQ::C8 => 58,
        SQ::D8 => 59,
        SQ::E8 => 60,
        SQ::F8 => 61,
        SQ::G8 => 62,
        SQ::H8 => 63,
        _      => 0, //should never happen
    }
}

fn get_piece_value(piece: PieceType) -> i32 {
    match piece {
        PieceType::P => 100,
        PieceType::N => 320,
        PieceType::B => 330,
        PieceType::R => 500,
        PieceType::Q => 900,
        PieceType::K => 20000,
        _ => 0,
    }
}

//scores assigned to each square for each piece type
const ROOK_SCORE_GRID: [[i32; 8]; 8] = [[0,  0,  0,  0,  0,  0,  0,  0],
                                        [5, 10, 10, 10, 10, 10, 10,  5],
                                        [-5,  0,  0,  0,  0,  0,  0, -5],
                                        [-5,  0,  0,  0,  0,  0,  0, -5],
                                        [-5,  0,  0,  0,  0,  0,  0, -5],
                                        [-5,  0,  0,  0,  0,  0,  0, -5],
                                        [-5,  0,  0,  0,  0,  0,  0, -5],
                                        [0,  0,  0,  5,  5,  0,  0,  0]];

const PAWN_SCORE_GRID: [[i32; 8]; 8] = [[ 0,  0,  0,  0,  0,  0,  0,  0],
                                        [50, 50, 50, 50, 50, 50, 50, 50],
                                        [10, 10, 20, 30, 30, 20, 10, 10],
                                        [ 5,  5, 10, 25, 25, 10,  5,  5],
                                        [ 0,  0,  0, 20, 20,  0,  0,  0],
                                        [ 5, -5,-10,  0,  0,-10, -5,  5],
                                        [ 5, 10, 10,-20,-20, 10, 10,  5],
                                        [ 0,  0,  0,  0,  0,  0,  0,  0]];

const BISHOP_SCORE_GRID: [[i32; 8]; 8] = [[-20,-10,-10,-10,-10,-10,-10,-20],
                                          [-10,  0,  0,  0,  0,  0,  0,-10],
                                          [-10,  0,  5, 10, 10,  5,  0,-10],
                                          [-10,  5,  5, 10, 10,  5,  5,-10],
                                          [-10,  0, 10, 10, 10, 10,  0,-10],
                                          [-10, 10, 10, 10, 10, 10, 10,-10],
                                          [-10,  5,  0,  0,  0,  0,  5,-10],
                                          [-20,-10,-10,-10,-10,-10,-10,-20]];

const KNIGHT_SCORE_GRID: [[i32; 8]; 8] = [[-50,-40,-30,-30,-30,-30,-40,-50],
                                          [-40,-20,  0,  0,  0,  0,-20,-40],
                                          [-30,  0, 10, 15, 15, 10,  0,-30],
                                          [-30,  5, 15, 20, 20, 15,  5,-30],
                                          [-30,  0, 15, 20, 20, 15,  0,-30],
                                          [-30,  5, 10, 15, 15, 10,  5,-30],
                                          [-40,-20,  0,  5,  5,  0,-20,-40],
                                          [-50,-40,-30,-30,-30,-30,-40,-50]];

const QUEEN_SCORE_GRID: [[i32; 8]; 8] = [[-20,-10,-10, -5, -5,-10,-10,-20],
                                         [-10,  0,  0,  0,  0,  0,  0,-10],
                                         [-10,  0,  5,  5,  5,  5,  0,-10],
                                         [ -5,  0,  5,  5,  5,  5,  0, -5],
                                         [  0,  0,  5,  5,  5,  5,  0, -5],
                                         [-10,  5,  5,  5,  5,  5,  0,-10],
                                         [-10,  0,  5,  0,  0,  0,  0,-10],
                                         [-20,-10,-10, -5, -5,-10,-10,-20]];

const KING_SCORE_GRID: [[i32; 8]; 8] = [[-30,-40,-40,-50,-50,-40,-40,-30],
                                        [-30,-40,-40,-50,-50,-40,-40,-30],
                                        [-30,-40,-40,-50,-50,-40,-40,-30],
                                        [-30,-40,-40,-50,-50,-40,-40,-30],
                                        [-20,-30,-30,-40,-40,-30,-30,-20],
                                        [-10,-20,-20,-20,-20,-20,-20,-10],
                                        [ 20, 20,  0,  0,  0,  0, 20, 20],
                                        [ 20, 30, 10,  0,  0, 10, 30, 20]];
                                         
const EMPTY_SCORE_GRID: [[i32; 8]; 8] = [[0; 8]; 8];

//takes in a board, and prints the board to the console
#[allow(dead_code)]
fn draw_board(board_obj : Board) {
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