use core::fmt;
use regex::Regex;

fn main() {
    print!(
        "{:?}",
        FenString::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w QKqk - 0 0".to_string())
    );
}

pub struct Board {
    pub squares: Vec<Vec<Option<Piece>>>,
    active_color: Color,
    castle_rights: [Option<CastleRight>; 4],
    en_passant_target: Square,
    halfmove_clock: u8,
    fullmove_clock: u32,
}

impl Board {
    pub fn from_fen(fen: String) -> Self {
        let fen_splitted = fen.split(' ').collect::<Vec<&str>>();

        let squares = fen_token_to_squares(String::from(fen_splitted[0])).unwrap();
        //let active_color = Color::from_char(fen_splitted[1]);

        Self {
            squares,
            active_color: Color::White,
            castle_rights: [None, None, None, None],
            en_passant_target: Square::new('a', 1).unwrap(),
            halfmove_clock: 0,
            fullmove_clock: 0,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output_string = String::new();
        let line_string = "-----------------\n";

        output_string += line_string;
        for rank in &self.squares {
            for square in rank {
                output_string += "|";

                match square {
                    None => output_string += " ",
                    Some(piece) => output_string += &piece.to_string(),
                }
            }
            output_string += "|\n";
        }
        output_string += line_string;
        write!(f, "{}", output_string)
    }
}

fn fen_token_to_squares(fen_squares: String) -> Result<Vec<Vec<Option<Piece>>>, String> {
    let mut squares: Vec<Vec<Option<Piece>>> = Vec::new();
    let square_tokens = fen_squares.split('/').collect::<Vec<&str>>();

    for rank_token in square_tokens {
        let mut rank: Vec<Option<Piece>> = Vec::new();

        for ch in rank_token.chars() {
            if ch.is_numeric() {
                let n = ch.to_digit(10).unwrap();
                for _ in 0..n {
                    rank.push(None);
                }
            } else {
                rank.push(Some(Piece::from_char(ch).expect("Fen invalid!")));
            }
        }
        squares.push(rank);
    }

    Ok(squares)
}

#[derive(Debug)]
struct FenString {
    value: String,
}

impl FenString {
    pub fn new(input_string: String) -> Result<Self, String> {
        //check validity
        let trimmed_string = input_string.trim();

        //6 tokens?
        if !(trimmed_string.matches(' ').count() == 5) {
            return Err("token count".to_string());
        }
        let tokenized_string: Vec<&str> = trimmed_string.split(' ').collect();

        //piece placement
        //TODO

        //active color
        let active_color_token = tokenized_string[1];
        match active_color_token {
            "w" | "b" => (),
            _ => return Err("active color".to_string()),
        }

        //castle rights
        let castle_rights_token = tokenized_string[2];
        let castle_rights_regex = Regex::new(r"^(-|(Q?K?q?k?))$").unwrap();
        if !(castle_rights_regex.is_match(castle_rights_token) && castle_rights_token != "") {
            return Err("castle rights".to_string());
        }

        //possible en passant target
        let en_passant_target_token = tokenized_string[3];
        if en_passant_target_token != "-" {
            if !(en_passant_target_token.len() == 2) {
                return Err("en passant target".to_string());
            }
            let square_result = Square::new(
                en_passant_target_token.chars().nth(0).unwrap(),
                en_passant_target_token
                    .chars()
                    .nth(1)
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as u8,
            );
            if !(square_result.is_ok()) {
                return Err("en passant target".to_string());
            }
            let rank = square_result.unwrap().rank;
            if !(rank == 3 || rank == 6) {
                return Err("en passant target".to_string());
            }
        }

        //halfmove clock
        let halfmove_clock_token = tokenized_string[4];
        let halfmove_clock_number_result: Result<u8, _> = halfmove_clock_token.parse();
        if halfmove_clock_number_result.is_err() {
            return Err("halfmove clock parsing error".to_string());
        }
        if halfmove_clock_number_result.unwrap() > 100 {
            return Err("halfmove clock value over 100".to_string());
        }

        //fullmove clock
        let fullmove_clock_token = tokenized_string[5];
        if fullmove_clock_token.parse::<u32>().is_err() {
            return Err("fullmove clock parsing error".to_string());
        }

        //all tests passed
        return Ok(FenString {
            value: "Y".to_string(),
        });
    }
}

struct Square {
    file: char,
    rank: u8,
}

impl Square {
    fn new(file: char, rank: u8) -> Result<Self, String> {
        match file {
            'a'..='h' => match rank {
                0..=8 => Ok(Self { file, rank }),
                _ => Err("could not parse string to square. invalid rank".to_string()),
            },
            _ => Err("could not parse string to square. invalid file".to_string()),
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}

enum CastleRight {
    Queen(Color),
    King(Color),
}

impl CastleRight {
    fn from_char(ch: char) -> Result<CastleRight, char> {
        match ch {
            'Q' => Ok(CastleRight::Queen(Color::White)),
            'K' => Ok(CastleRight::King(Color::White)),
            'q' => Ok(CastleRight::Queen(Color::Black)),
            'k' => Ok(CastleRight::King(Color::Black)),

            _ => Err(ch),
        }
    }
}

impl fmt::Display for CastleRight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CastleRight::Queen(Color::White) => write!(f, "Q"),
            CastleRight::King(Color::White) => write!(f, "K"),
            CastleRight::Queen(Color::Black) => write!(f, "q"),
            CastleRight::King(Color::Black) => write!(f, "k"),
        }
    }
}

enum Piece {
    Pawn(Color),
    Rook(Color),
    Knight(Color),
    Bishop(Color),
    Queen(Color),
    King(Color),
}

impl Piece {
    fn from_char(ch: char) -> Result<Piece, char> {
        match ch {
            'P' => Ok(Piece::Pawn(Color::White)),
            'R' => Ok(Piece::Rook(Color::White)),
            'N' => Ok(Piece::Knight(Color::White)),
            'B' => Ok(Piece::Bishop(Color::White)),
            'Q' => Ok(Piece::Queen(Color::White)),
            'K' => Ok(Piece::King(Color::White)),

            'p' => Ok(Piece::Pawn(Color::Black)),
            'r' => Ok(Piece::Rook(Color::Black)),
            'n' => Ok(Piece::Knight(Color::Black)),
            'b' => Ok(Piece::Bishop(Color::Black)),
            'q' => Ok(Piece::Queen(Color::Black)),
            'k' => Ok(Piece::King(Color::Black)),

            _ => Err(ch),
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch: char;
        match self {
            Piece::Pawn(Color::White) => ch = 'P',
            Piece::Rook(Color::White) => ch = 'R',
            Piece::Knight(Color::White) => ch = 'N',
            Piece::Bishop(Color::White) => ch = 'B',
            Piece::Queen(Color::White) => ch = 'Q',
            Piece::King(Color::White) => ch = 'K',

            Piece::Pawn(Color::Black) => ch = 'p',
            Piece::Rook(Color::Black) => ch = 'r',
            Piece::Knight(Color::Black) => ch = 'n',
            Piece::Bishop(Color::Black) => ch = 'b',
            Piece::Queen(Color::Black) => ch = 'q',
            Piece::King(Color::Black) => ch = 'k',
        }
        write!(f, "{}", ch)
    }
}

enum Color {
    White,
    Black,
}

impl Color {
    fn from_char(ch: char) -> Result<Color, char> {
        match ch {
            'w' => Ok(Self::White),
            'b' => Ok(Self::Black),

            _ => Err(ch),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::White => write!(f, "w"),
            Self::Black => write!(f, "b"),
        }
    }
}
