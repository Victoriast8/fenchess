enum PieceType {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

struct Piece {
    ptype: PieceType,
    is_white: bool,
}

struct Position {
    row: u8,
    col: u8,
}

struct Game {
    board: Vec<Option<Piece>>,
    white_turn: bool,
    castle: u8,
    enpaissant: Option<Position>,
    halfmoves: u8,
    fullmoves: u32,
}

fn new_game() -> Game {
    parse_fen(new_fen())
}

fn new_fen() -> String {
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
}


/// Parsing functions
// lexeme removes leading whitespace
fn lexeme(s: &str) -> String {
    let mut iter = s.chars();
    match iter.next() {
        Some(' ') => lexeme(iter.as_str()),
        _ => s.to_string(),
    }
}

fn pCol(c: char) -> u8 {
    match c {
        'a' => 1, 'b' => 2, 'c' => 3, 'd' => 4,
        'e' => 5, 'f' => 6, 'g' => 7, 'h' => 8,
        _   => 0,
    }
}

fn pRow(c: char) -> u8 {
    match c.to_digit(10) {
        Some(c') if c' < 9 => c',
        _ => 0,
    }
}

fn pPosition(s: &mut impl Iterator<Item = char>) -> Option<Position> {
    None
}

fn isRow(x: i32) -> bool {
    x > 0 && x < 9
}

fn parse_fen(fen: String) -> Game {
    let fenit = fen.chars();
    let mut board_vec: Vec<Option<Piece>> = Vec::new();

    // build board
    while let Some(c) = fenit.next() {
        board_vec.push(match c {
            'r' => Some(Piece { ptype: PieceType::Rook,   is_white: false }),
            'n' => Some(Piece { ptype: PieceType::Knight, is_white: false }),
            'b' => Some(Piece { ptype: PieceType::Bishop, is_white: false }),
            'q' => Some(Piece { ptype: PieceType::Queen,  is_white: false }),
            'k' => Some(Piece { ptype: PieceType::King,   is_white: false }),
            'p' => Some(Piece { ptype: PieceType::Pawn,   is_white: false }),
            'R' => Some(Piece { ptype: PieceType::Rook,   is_white: true }),
            'N' => Some(Piece { ptype: PieceType::Knight, is_white: true }),
            'B' => Some(Piece { ptype: PieceType::Bishop, is_white: true }),
            'Q' => Some(Piece { ptype: PieceType::Queen,  is_white: true }),
            'K' => Some(Piece { ptype: PieceType::King,   is_white: true }),
            'P' => Some(Piece { ptype: PieceType::Pawn,   is_white: true }),
            ' ' => break,
            '/' => continue,
            _   => None,
        })
    }
    
    board_vec.reverse();

    let Some(turn) = fenit.next();
    let turn = if turn == 'w' { true } else { false };

    let mut castle_rights = 0;

    while let Some(c) = fenit.next() {
        match c {
            'k' => castle_rights += 1,
            'q' => castle_rights += 2,
            'K' => castle_rights += 4,
            'Q' => castle_rights += 8,
            _   => break,
        }
    }

    let Some(turn) = fenit.next();
    let turn = if turn == 'w' { true } else { false };

    Game { board: board_vec, white_turn: turn, castle: castle_rights }
}
