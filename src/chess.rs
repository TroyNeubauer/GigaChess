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
    type FileType = ChessFile;
    type RankType = ChessRank;
    type StorageType = u8;
    type RawMoveIteratorType = RawMoveIterator;
    type PieceIteratorType = DefaultPieceIter<ChessBoard>;

    fn side_len() -> Self::StorageType {
        8
    }

    fn new() -> ChessBoard {
        ChessBoard {
            board: [RawSquare::empty(); 64],
        }
    }

    #[rustfmt::skip]
    fn default() -> ChessBoard {
        let mut result = ChessBoard::new();

        result.set(ChessBoard::to_storage(ChessFile::A, ChessRank::R1), RawSquare::new(ChessPiece::Rook,   DefaultColorScheme::While));
        result.set(ChessBoard::to_storage(ChessFile::B, ChessRank::R1), RawSquare::new(ChessPiece::Knight, DefaultColorScheme::While));
        result.set(ChessBoard::to_storage(ChessFile::C, ChessRank::R1), RawSquare::new(ChessPiece::Bishop, DefaultColorScheme::While));
        result.set(ChessBoard::to_storage(ChessFile::D, ChessRank::R1), RawSquare::new(ChessPiece::Queen,  DefaultColorScheme::While));
        result.set(ChessBoard::to_storage(ChessFile::E, ChessRank::R1), RawSquare::new(ChessPiece::King,   DefaultColorScheme::While));
        result.set(ChessBoard::to_storage(ChessFile::F, ChessRank::R1), RawSquare::new(ChessPiece::Bishop, DefaultColorScheme::While));
        result.set(ChessBoard::to_storage(ChessFile::G, ChessRank::R1), RawSquare::new(ChessPiece::Knight, DefaultColorScheme::While));
        result.set(ChessBoard::to_storage(ChessFile::H, ChessRank::R1), RawSquare::new(ChessPiece::Rook,   DefaultColorScheme::While));

        result.set(ChessBoard::to_storage(ChessFile::A, ChessRank::R2), RawSquare::new(ChessPiece::Pawn,   DefaultColorScheme::While));
        result.set(ChessBoard::to_storage(ChessFile::B, ChessRank::R2), RawSquare::new(ChessPiece::Pawn,   DefaultColorScheme::While));
        result.set(ChessBoard::to_storage(ChessFile::C, ChessRank::R2), RawSquare::new(ChessPiece::Pawn,   DefaultColorScheme::While));
        result.set(ChessBoard::to_storage(ChessFile::D, ChessRank::R2), RawSquare::new(ChessPiece::Pawn,   DefaultColorScheme::While));
        result.set(ChessBoard::to_storage(ChessFile::E, ChessRank::R2), RawSquare::new(ChessPiece::Pawn,   DefaultColorScheme::While));
        result.set(ChessBoard::to_storage(ChessFile::F, ChessRank::R2), RawSquare::new(ChessPiece::Pawn,   DefaultColorScheme::While));
        result.set(ChessBoard::to_storage(ChessFile::G, ChessRank::R2), RawSquare::new(ChessPiece::Pawn,   DefaultColorScheme::While));
        result.set(ChessBoard::to_storage(ChessFile::H, ChessRank::R2), RawSquare::new(ChessPiece::Pawn,   DefaultColorScheme::While));

 
        result.set(ChessBoard::to_storage(ChessFile::A, ChessRank::R7), RawSquare::new(ChessPiece::Pawn,   DefaultColorScheme::Black));
        result.set(ChessBoard::to_storage(ChessFile::B, ChessRank::R7), RawSquare::new(ChessPiece::Pawn,   DefaultColorScheme::Black));
        result.set(ChessBoard::to_storage(ChessFile::C, ChessRank::R7), RawSquare::new(ChessPiece::Pawn,   DefaultColorScheme::Black));
        result.set(ChessBoard::to_storage(ChessFile::D, ChessRank::R7), RawSquare::new(ChessPiece::Pawn,   DefaultColorScheme::Black));
        result.set(ChessBoard::to_storage(ChessFile::E, ChessRank::R7), RawSquare::new(ChessPiece::Pawn,   DefaultColorScheme::Black));
        result.set(ChessBoard::to_storage(ChessFile::F, ChessRank::R7), RawSquare::new(ChessPiece::Pawn,   DefaultColorScheme::Black));
        result.set(ChessBoard::to_storage(ChessFile::G, ChessRank::R7), RawSquare::new(ChessPiece::Pawn,   DefaultColorScheme::Black));
        result.set(ChessBoard::to_storage(ChessFile::H, ChessRank::R7), RawSquare::new(ChessPiece::Pawn,   DefaultColorScheme::Black));

        result.set(ChessBoard::to_storage(ChessFile::A, ChessRank::R8), RawSquare::new(ChessPiece::Rook,   DefaultColorScheme::Black));
        result.set(ChessBoard::to_storage(ChessFile::B, ChessRank::R8), RawSquare::new(ChessPiece::Knight, DefaultColorScheme::Black));
        result.set(ChessBoard::to_storage(ChessFile::C, ChessRank::R8), RawSquare::new(ChessPiece::Bishop, DefaultColorScheme::Black));
        result.set(ChessBoard::to_storage(ChessFile::D, ChessRank::R8), RawSquare::new(ChessPiece::Queen,  DefaultColorScheme::Black));
        result.set(ChessBoard::to_storage(ChessFile::E, ChessRank::R8), RawSquare::new(ChessPiece::King,   DefaultColorScheme::Black));
        result.set(ChessBoard::to_storage(ChessFile::F, ChessRank::R8), RawSquare::new(ChessPiece::Bishop, DefaultColorScheme::Black));
        result.set(ChessBoard::to_storage(ChessFile::G, ChessRank::R8), RawSquare::new(ChessPiece::Knight, DefaultColorScheme::Black));
        result.set(ChessBoard::to_storage(ChessFile::H, ChessRank::R8), RawSquare::new(ChessPiece::Rook,   DefaultColorScheme::Black));
       
        result
    }


    fn to_storage(file: Self::FileType, rank: Self::RankType) -> u8 {
        Self::FileType::to_storage(file) | (Self::RankType::to_storage(rank) << 3)
    }


    fn from_storage(storage: u8) -> (Self::FileType, Self::RankType) {
        (Self::FileType::from_storage((storage >> 0) & 0b111), Self::RankType::from_storage((storage >> 3) & 0b111))
    }


    fn raw_moves_for_piece(&self, pos: u8) -> RawMoveIterator {
        RawMoveIterator {
            board: self.clone(),
            last_move: None,
        }
    }

    fn raw_square_iter(&self) -> DefaultRawSquareIter<ChessBoard> {
        DefaultRawSquareIter::new(0, ChessBoard::side_len() * ChessBoard::side_len())
    }

    fn get(&self, pos: u8) -> &RawSquare<ChessPiece, DefaultColorScheme> {
        &self.board[pos as usize]
    }

    ///Swaps the piece on the board with the mutable piece specified
    fn swap(
        &mut self,
        pos: u8,
        piece: &mut RawSquare<ChessPiece, DefaultColorScheme>,
    ) {
        std::mem::swap(&mut self.board[pos as usize], piece);
    }

    fn set(
        &mut self,
        pos: u8,
        piece: RawSquare<ChessPiece, DefaultColorScheme>,
    ) -> RawSquare<ChessPiece, DefaultColorScheme> {
        let result = self.board[pos as usize];
        self.board[pos as usize] = piece;
        result
    }


    ///Enumerates all the pieces on the board
    fn pieces(&self) -> Self::PieceIteratorType {
        DefaultPieceIter::new(self.raw_square_iter(), None, self.clone())
    }

    ///Enumerates all the pieces on the board
    fn pieces_for_color(&self, color: Self::ColorType) -> Self::PieceIteratorType {
        DefaultPieceIter::new(self.raw_square_iter(), Some(color), self.clone())
    }

}

impl ToString for ChessBoard {
    fn to_string(&self) -> String {

        String::new()
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

impl GenericFile<ChessBoard> for ChessFile
{
    type StorageType = u8;

    fn to_storage(self) -> Self::StorageType {
        match self {
            ChessFile::A => 0,
            ChessFile::B => 1,
            ChessFile::C => 2,
            ChessFile::D => 3,
            ChessFile::E => 4,
            ChessFile::F => 5,
            ChessFile::G => 6,
            ChessFile::H => 7,
        }
    }

    fn from_storage(input: Self::StorageType) -> ChessFile {
        match input {
            0 => ChessFile::A,
            1 => ChessFile::B,
            2 => ChessFile::C,
            3 => ChessFile::D,
            4 => ChessFile::E,
            5 => ChessFile::F,
            6 => ChessFile::G,
            7 => ChessFile::H,
            _ => panic!(),
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

impl GenericRank<ChessBoard> for ChessRank
{
    type StorageType = u8;

    fn to_storage(self) -> Self::StorageType {
        match self {
            ChessRank::R1 => 0,
            ChessRank::R2 => 1,
            ChessRank::R3 => 2,
            ChessRank::R4 => 3,
            ChessRank::R5 => 4,
            ChessRank::R6 => 5,
            ChessRank::R7 => 6,
            ChessRank::R8 => 7,
        }
    }

    fn from_storage(input: Self::StorageType) -> ChessRank {
        match input {
            0 => ChessRank::R1,
            1 => ChessRank::R2,
            2 => ChessRank::R3,
            3 => ChessRank::R4,
            4 => ChessRank::R5,
            5 => ChessRank::R6,
            6 => ChessRank::R7,
            7 => ChessRank::R8,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_set_get_and_swap() {
        let board: ChessBoard = ChessBoard::new();
        let empty_square = RawSquare::<ChessPiece, DefaultColorScheme>::empty();


        let white_king = RawSquare::<ChessPiece, DefaultColorScheme>::new(
            ChessPiece::King,
            DefaultColorScheme::While,
        );
        let black_king = RawSquare::<ChessPiece, DefaultColorScheme>::new(
            ChessPiece::King,
            DefaultColorScheme::Black,
        );

        let square1 = ChessBoard::to_storage(ChessFile::A, ChessRank::R1);
        let square2 = ChessBoard::to_storage(ChessFile::E, ChessRank::R4);

        crate::chess_like::test::basic_set_get_and_swap(board, square1, square2, white_king, black_king, empty_square);

        println!("Regular board size: {}", std::mem::size_of::<ChessBoard>());

        

   }
}

