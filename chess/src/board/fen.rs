use crate::board::board_structs::Board;
use regex::Regex;

pub fn is_fen_valid(string: &str) -> bool {
    // check syntax
    let piece_placement = "([prnbqkPRNBQK12345678]{1,8}/){7}[prnbqkPRNBQK12345678]{1,8}"; //done
    let active_color = "(w|b)"; //done
    let castling_rights = r"(\-|(KQ?k?q?)|(Qk?q?)|(kq?)|q)"; //done
    let possible_en_passant = r"(\-|([abcdefgh][36]))"; //done
    let halfmove_clock = "([0-9]|([1-9][0-9])|100)"; //done
    let fullmove_clock = "(0|([1-9][0-9]*))"; //done
    let regex = Regex::new(&format!("^({piece_placement} {active_color} {castling_rights} {possible_en_passant} {halfmove_clock} {fullmove_clock})$")).unwrap();

    if !regex.is_match(string) {
        return false;
    }

    // check semantics
    let piece_placement_slice = string.split(' ').collect::<Vec<&str>>()[0];
    let rows = piece_placement_slice.split('/').collect::<Vec<&str>>();

    for row in rows {
        let mut count = 0;
        for char in row.chars() {
            match char {
                '1'..='8' => count += char.to_digit(10).unwrap(),
                _ => count += 1,
            }
        }

        if count > 8 {
            return false;
        }
    }

    return true;
}

pub fn load_fen(mut board: Board, fen: &str) -> Result<Board, &str> {
    if !is_fen_valid(fen) {
        return Err("FEN-String is invalid!");
    }

    return Ok(board);
}
