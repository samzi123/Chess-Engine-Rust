use pleco::{Board,Player,PieceType,SQ};

#[allow(dead_code)]
fn main() {
    //let board = Board::start_pos();
    let board = Board::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2").unwrap();

    draw_board(&board);

    let score = evaluate(&board);
    println!("Score: {}", score);
}

//takes in a board and returns its score
fn evaluate(board: &Board) -> i32 {
    let mut score = 0i32;
    let piece_locations = board.get_piece_locations();

    for (sq, piece) in piece_locations {
        let player = match piece.player() {
            Some(x) => x,
            None => Player::White
        };

        score += get_piece_score_at_square(piece.type_of(), sq, player);
    }

    score
}

//gets score of a piece at a specific square
fn get_piece_score_at_square(piece: PieceType, square: SQ, player: Player) -> i32 {
    let mut score = 0i32;

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
    score = (score_grid[index / 8][index % 8] + piece_value) * score_multiplier;

    score
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
fn draw_board(board_obj : &Board) {
    let board_str = &board_obj.fen();
    let mut board = [['\0'; 8]; 8];

    let lines = board_str.split("/");
    let mut char_index = 0usize;

    //iterate through each line and fill the board
    for line in lines{
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