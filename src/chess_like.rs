use std::marker::PhantomData;

pub trait GenericBoard<PieceType, BoardType, IteratorType: Iterator> {
    fn is_move_legal(&self, board_move: Move) -> bool;
    fn legal_moves_for_piece(&self, pos: SquarePos) -> IteratorType;
}

pub struct SquarePos {
    pos: u8,
}

pub struct Move {
    src: SquarePos,
    dest: SquarePos,
}


pub trait GenericPiece<Piece, Board, IteratorType: Iterator> {
    fn legal_moves_it(board: &Board) -> IteratorType;
}



