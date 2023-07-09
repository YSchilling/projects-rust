use crate::pieces::piece_structs::Piece;

pub struct Board {
    fields: Vec<Field>,
}

pub struct Field {
    piece: Option<Piece>,
}
