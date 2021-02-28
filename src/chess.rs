use crate::chess_like::*;

#[derive(Copy, Clone)]
enum ChessPiece {
    KING,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN,
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
    fn is_move_legal(&self, board_move: Move) -> bool {
        false
    }

    fn raw_moves_for_piece(&self, pos: SquarePos) -> LegalChessMovesIterator {
        LegalChessMovesIterator {
            board: self.clone(),
            last_move: None,
        }
    }

    fn get_attackers_of_square(&self, target_pos: SquarePos) -> Vec<SquarePos> {
        let mut result = Vec::new();
        for pos in self.square_iter() {
            if self.is_move_legal(Move::new(pos, target_pos)) {
                result.push(pos);
            }

        }
        result
    }

    fn square_iter(&self) -> SquareIter {
        let side_len = self.board.len();
        let max_size: u8 = (side_len * side_len) as u8;
        SquareIter::new(max_size, 0)
    }

}

impl Iterator for LegalChessMovesIterator {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
