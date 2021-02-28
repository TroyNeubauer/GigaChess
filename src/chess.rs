use crate::chess_like::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ChessPiece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl GenericPiece for ChessPiece {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ChessBoard {
    board: [RawSquare<ChessPiece, DefaultColorScheme>; 64],
}

pub struct RawMoveIterator {
    board: ChessBoard,
    last_move: Option<Move<ChessBoard>>,
}

impl GenericBoard for ChessBoard {
    type PieceType = ChessPiece;
    type ColorType = DefaultColorScheme;
    type RawMoveIteratorType = RawMoveIterator;
    type StorageType = u8;
    type FileType = ChessFile;
    type RankType = ChessRank;

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

    fn raw_moves_for_piece(&self, pos: SquarePos<Self>) -> RawMoveIterator {
        RawMoveIterator {
            board: self.clone(),
            last_move: None,
        }
    }

    fn get_attackers_of_square(&self, target_pos: SquarePos<Self>) -> Vec<SquarePos<Self>> {
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

    fn get(&self, pos: SquarePos<Self>) -> &RawSquare<ChessPiece, DefaultColorScheme> {
        &self.board[pos.raw_value() as usize]
    }

    ///Swaps the piece on the board with the mutable piece specified
    fn swap(&self, pos: SquarePos<Self>, piece: &mut RawSquare<ChessPiece, DefaultColorScheme>) {}

    fn set(
        &mut self,
        pos: SquarePos<Self>,
        piece: RawSquare<ChessPiece, DefaultColorScheme>,
    ) -> RawSquare<ChessPiece, DefaultColorScheme> {
        let result = self.board[pos.raw_value() as usize];
        self.board[pos.raw_value() as usize] = piece;
        result
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ChessFile {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl GenericFile<<ChessBoard as GenericBoard>::StorageType> for ChessFile {
    fn to_storage(self) -> u8 {
        match self {
            ChessFile::A => 0,
            ChessFile::B => 1,
            ChessFile::C => 2,
            ChessFile::D => 3,
            ChessFile::E => 4,
            ChessFile::F => 5,
            ChessFile::G => 6,
            ChessFile::H => 7,
            _ => unreachable!(),
        }
    }

    fn from_storage(input: <ChessBoard as GenericBoard>::StorageType) -> ChessFile {
        match input {
            0 => ChessFile::A,
            1 => ChessFile::B,
            2 => ChessFile::C,
            3 => ChessFile::D,
            4 => ChessFile::E,
            5 => ChessFile::F,
            6 => ChessFile::G,
            7 => ChessFile::H,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ChessRank {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

impl GenericRank<<ChessBoard as GenericBoard>::StorageType> for ChessRank {
    fn to_storage(self) -> u8 {
        match self {
            ChessRank::R1 => 0,
            ChessRank::R2 => 1,
            ChessRank::R3 => 2,
            ChessRank::R4 => 3,
            ChessRank::R5 => 4,
            ChessRank::R6 => 5,
            ChessRank::R7 => 6,
            ChessRank::R8 => 7,
            _ => unreachable!(),
        }
    }

    fn from_storage(input: <ChessBoard as GenericBoard>::StorageType) -> ChessRank {
        match input {
            0 => ChessRank::R1,
            1 => ChessRank::R2,
            2 => ChessRank::R3,
            3 => ChessRank::R4,
            4 => ChessRank::R5,
            5 => ChessRank::R6,
            6 => ChessRank::R7,
            7 => ChessRank::R8,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_set_get_and_swap() {
        let mut board: ChessBoard = ChessBoard::new();
        let empty_square = RawSquare::<ChessPiece, DefaultColorScheme>::empty();

        assert_eq!(
            board.get(SquarePos::<ChessBoard>::new(1, 1)),
            &empty_square
        );
        assert_eq!(
            board.get(SquarePos::<ChessBoard>::new(0, 0)),
            &empty_square
        );

        let white_king = RawSquare::<ChessPiece, DefaultColorScheme>::new(
            ChessPiece::King,
            DefaultColorScheme::While,
        );
        let black_king = RawSquare::<ChessPiece, DefaultColorScheme>::new(
            ChessPiece::King,
            DefaultColorScheme::Black,
        );
        let last_piece = board.set(SquarePos::<ChessBoard>::new(0, 0), white_king);
        assert_eq!(last_piece, empty_square);
        assert_eq!(
            board.get(SquarePos::<ChessBoard>::new(0, 0)),
            &white_king
        );

        //Start with a black king in our "hand" then swap it with the white king on E4
        let mut hand_piece = black_king;

        board.swap(SquarePos::<ChessBoard>::new(0, 0), &mut hand_piece);
        assert_eq!(hand_piece, white_king);
        assert_eq!(
            board.get(SquarePos::<ChessBoard>::new(0, 0)),
            &black_king
        );
    }
}



