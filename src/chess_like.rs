use std::marker::PhantomData;

pub trait GenericBoard<PieceType> {
    fn is_move_legal(board_move: Move) -> bool;
    fn legal_moves_for_piece(
}

pub struct SquarePos {
    pos: u8,
}

pub struct Move {
    src: SquarePos,
    dest: SquarePos,
}


pub struct LegalMovesIterator<Piece, Board> {
    board: Board,
    last_move: Move,
    piece: PhantomData<Piece>,
}

pub trait GenericPiece {
    fn legal_moves_it<Piece, Board>(board: &Board) -> LegalMovesIterator<Piece, Board>;
}



