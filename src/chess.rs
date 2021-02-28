use crate::chess_like::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ChessPiece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct ChessBoard {
    board: [RawSquare<ChessPiece, DefaultColorScheme>; 64],
}

struct RawMoveIterator {
    board: ChessBoard,
    last_move: Option<Move<ChessBoard>>,
}

impl GenericBoard for ChessBoard {
    type PieceType = ChessPiece;
    type ColorType = DefaultColorScheme;
    type RawMoveIteratorType = RawMoveIterator;

    fn side_len() -> u8 {
        8
    }

    fn new() -> ChessBoard {
        ChessBoard {
            board: [RawSquare::empty(); 64],
        }
    }

    fn default() -> ChessBoard {
        let mut result = ChessBoard::new();

        result
    }

    fn raw_moves_for_piece(&self, pos: SquarePos<ChessBoard>) -> RawMoveIterator {
        RawMoveIterator {
            board: self.clone(),
            last_move: None,
        }
    }

    fn get_attackers_of_square(
        &self,
        target_pos: SquarePos<ChessBoard>,
    ) -> Vec<SquarePos<ChessBoard>> {
        let mut result = Vec::new();
        for pos in self.raw_square_iter() {
            if self.is_move_legal(Move::new(pos, target_pos)) {
                result.push(pos);
            }
        }
        result
    }

    fn raw_square_iter(&self) -> SquareIter<ChessBoard> {
        let max_size = (ChessBoard::side_len() * ChessBoard::side_len()) as u8;
        SquareIter::new(max_size, 0)
    }

    fn get(&self, pos: SquarePos<ChessBoard>) -> &RawSquare<ChessPiece, DefaultColorScheme> {
        &self.board[pos.raw_value() as usize]
    }

    ///Swaps the piece on the board with the mutable piece specified
    fn swap(
        &self,
        pos: SquarePos<ChessBoard>,
        piece: &mut RawSquare<ChessPiece, DefaultColorScheme>,
    ) {
    }


    fn is_move_legal(&self, board_move: Move<ChessBoard>) -> bool {
        let it = self.raw_moves_for_piece(board_move.src);
        for generated_move in it {
            if generated_move == board_move {
                //If we can find a matching generated raw move then we are on the right track.
                //Now we just need to check for checks and we are good.
                return true;
            }
        }

        false
    }
 


}

impl Iterator for RawMoveIterator {
    type Item = Move<ChessBoard>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn legal_moves_it1() {
        let board: ChessBoard = ChessBoard::new();
    }
}
