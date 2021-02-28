use crate::chess_like::*;

#[derive(Copy, Clone)]
enum ChessPiece {
	KING, QUEEN, ROOK, BISHOP, KNIGHT, PAWN
}

#[derive(Copy, Clone)]
struct ChessBoard {
    board: [[ChessPiece; 8]; 8],
}

struct LegalChessMovesIterator {
    board: ChessBoard,
    last_move: Option<Move>,
}


impl GenericBoard<ChessPiece, ChessBoard, LegalChessMovesIterator> for ChessBoard {

    fn is_move_legal(&self, board_move: Move) -> bool
    {
        false
    }

    fn legal_moves_for_piece(&self, pos: SquarePos) -> LegalChessMovesIterator {
        LegalChessMovesIterator {
            board: self.clone(),
            last_move: None

        }
    }
}

impl Iterator for LegalChessMovesIterator  {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {

        None
    }


}

