use crate::chess_like::*;

enum ChessPiece {
	KING, QUEEN, ROOK, BISHOP, KNIGHT, PAWN
}


struct ChessBoard {
    board: [[ChessPiece; 8]; 8],
}


impl GenericBoard<ChessPiece> for ChessBoard {

    fn is_move_legal(board_move: Move) -> bool
    {
        false
    }

}

impl Iterator for LegalMovesIterator<ChessPiece, ChessBoard>  {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {

        None
    }


}


