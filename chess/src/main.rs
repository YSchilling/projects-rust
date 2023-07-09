mod board;
mod pieces;

use crate::board::fen::is_fen_valid;

fn main() {
    let string = "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50";
    println!("{}", is_fen_valid(string));
}
