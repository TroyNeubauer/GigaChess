use crate::chess_like::*;
use std::fmt;

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

impl GenericBoard for ChessBoard {
    type PieceType = ChessPiece;
    type ColorType = DefaultColorScheme;
    type FileType = ChessFile;
    type RankType = ChessRank;
    type StorageType = u8;
    type PieceIteratorType = DefaultPieceIter<ChessBoard>;

    fn side_len() -> Self::StorageType {
        8
    }

    fn new() -> ChessBoard {
        ChessBoard {
            board: [RawSquare::empty(); 64],
        }
    }
    fn to_storage(file: Self::FileType, rank: Self::RankType) -> u8 {
        Self::FileType::to_storage(file) | (Self::RankType::to_storage(rank) << 3)
    }

    fn from_storage(storage: u8) -> (Self::FileType, Self::RankType) {
        (
            Self::FileType::from_storage((storage >> 0) & 0b111),
            Self::RankType::from_storage((storage >> 3) & 0b111),
        )
    }

    fn moves_for_piece(&self, pos: u8) -> MoveList<ChessBoard> {
        let mut result = smallvec::SmallVec::new();
        match self.get(pos).0 {
            Some(piece) => match piece.piece {
                ChessPiece::King => {
                    self.try_add_move(pos, 0, -1, &mut result);
                }
                ChessPiece::Queen => {}
                ChessPiece::Rook => {}
                ChessPiece::Bishop => {}
                ChessPiece::Knight => {}
                ChessPiece::Pawn => {}
            },
            None => {}
        }

        result
    }

    fn raw_square_iter(&self) -> DefaultRawSquareIter<ChessBoard> {
        DefaultRawSquareIter::new(0, ChessBoard::side_len() * ChessBoard::side_len())
    }

    fn get(&self, pos: u8) -> &RawSquare<ChessPiece, DefaultColorScheme> {
        &self.board[pos as usize]
    }

    ///Swaps the piece on the board with the mutable piece specified
    fn swap(&mut self, pos: u8, piece: &mut RawSquare<ChessPiece, DefaultColorScheme>) {
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

    fn is_square_empty_offset(
        &self,
        pos: Self::StorageType,
        file: isize,
        rank: isize,
    ) -> Option<Self::StorageType> {
        let (src_file, src_rank) = Self::from_storage(pos);
        let dest_file: isize = Self::FileType::to_storage(src_file) as isize + file;
        let dest_rank: isize = Self::RankType::to_storage(src_rank) as isize + rank;
        if dest_file < 0 || dest_file >= Self::side_len().into() {
            //File off the board
            return None;
        }
        if dest_rank < 0 || dest_rank >= Self::side_len().into() {
            //File rank
            return None;
        }
        let new_pos: Self::StorageType = Self::to_storage(
            ChessFile::from_storage(dest_file as u8),
            ChessRank::from_storage(dest_rank as u8),
        );
        let square = self.get(new_pos);
        match square.0 {
            Some(piece) => None,
            None => Some(new_pos),
        }
    }

    fn default() -> ChessBoard {
        //TODO cleanup
        let mut result = ChessBoard::new();

        result.set(
            ChessBoard::to_storage(ChessFile::A, ChessRank::R1),
            RawSquare::new(ChessPiece::Rook, DefaultColorScheme::While),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::B, ChessRank::R1),
            RawSquare::new(ChessPiece::Knight, DefaultColorScheme::While),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::C, ChessRank::R1),
            RawSquare::new(ChessPiece::Bishop, DefaultColorScheme::While),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::D, ChessRank::R1),
            RawSquare::new(ChessPiece::Queen, DefaultColorScheme::While),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::E, ChessRank::R1),
            RawSquare::new(ChessPiece::King, DefaultColorScheme::While),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::F, ChessRank::R1),
            RawSquare::new(ChessPiece::Bishop, DefaultColorScheme::While),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::G, ChessRank::R1),
            RawSquare::new(ChessPiece::Knight, DefaultColorScheme::While),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::H, ChessRank::R1),
            RawSquare::new(ChessPiece::Rook, DefaultColorScheme::While),
        );

        result.set(
            ChessBoard::to_storage(ChessFile::A, ChessRank::R2),
            RawSquare::new(ChessPiece::Pawn, DefaultColorScheme::While),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::B, ChessRank::R2),
            RawSquare::new(ChessPiece::Pawn, DefaultColorScheme::While),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::C, ChessRank::R2),
            RawSquare::new(ChessPiece::Pawn, DefaultColorScheme::While),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::D, ChessRank::R2),
            RawSquare::new(ChessPiece::Pawn, DefaultColorScheme::While),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::E, ChessRank::R2),
            RawSquare::new(ChessPiece::Pawn, DefaultColorScheme::While),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::F, ChessRank::R2),
            RawSquare::new(ChessPiece::Pawn, DefaultColorScheme::While),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::G, ChessRank::R2),
            RawSquare::new(ChessPiece::Pawn, DefaultColorScheme::While),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::H, ChessRank::R2),
            RawSquare::new(ChessPiece::Pawn, DefaultColorScheme::While),
        );

        result.set(
            ChessBoard::to_storage(ChessFile::A, ChessRank::R7),
            RawSquare::new(ChessPiece::Pawn, DefaultColorScheme::Black),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::B, ChessRank::R7),
            RawSquare::new(ChessPiece::Pawn, DefaultColorScheme::Black),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::C, ChessRank::R7),
            RawSquare::new(ChessPiece::Pawn, DefaultColorScheme::Black),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::D, ChessRank::R7),
            RawSquare::new(ChessPiece::Pawn, DefaultColorScheme::Black),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::E, ChessRank::R7),
            RawSquare::new(ChessPiece::Pawn, DefaultColorScheme::Black),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::F, ChessRank::R7),
            RawSquare::new(ChessPiece::Pawn, DefaultColorScheme::Black),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::G, ChessRank::R7),
            RawSquare::new(ChessPiece::Pawn, DefaultColorScheme::Black),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::H, ChessRank::R7),
            RawSquare::new(ChessPiece::Pawn, DefaultColorScheme::Black),
        );

        result.set(
            ChessBoard::to_storage(ChessFile::A, ChessRank::R8),
            RawSquare::new(ChessPiece::Rook, DefaultColorScheme::Black),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::B, ChessRank::R8),
            RawSquare::new(ChessPiece::Knight, DefaultColorScheme::Black),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::C, ChessRank::R8),
            RawSquare::new(ChessPiece::Bishop, DefaultColorScheme::Black),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::D, ChessRank::R8),
            RawSquare::new(ChessPiece::Queen, DefaultColorScheme::Black),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::E, ChessRank::R8),
            RawSquare::new(ChessPiece::King, DefaultColorScheme::Black),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::F, ChessRank::R8),
            RawSquare::new(ChessPiece::Bishop, DefaultColorScheme::Black),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::G, ChessRank::R8),
            RawSquare::new(ChessPiece::Knight, DefaultColorScheme::Black),
        );
        result.set(
            ChessBoard::to_storage(ChessFile::H, ChessRank::R8),
            RawSquare::new(ChessPiece::Rook, DefaultColorScheme::Black),
        );

        result
    }
}

impl ToString for ChessBoard {
    fn to_string(&self) -> String {
        String::new()
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

impl GenericFile<ChessBoard> for ChessFile {
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

impl fmt::Display for ChessFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ChessFile::A => f.write_str("A"),
            ChessFile::B => f.write_str("B"),
            ChessFile::C => f.write_str("C"),
            ChessFile::D => f.write_str("D"),
            ChessFile::E => f.write_str("E"),
            ChessFile::F => f.write_str("F"),
            ChessFile::G => f.write_str("G"),
            ChessFile::H => f.write_str("H"),
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

impl GenericRank<ChessBoard> for ChessRank {
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

impl fmt::Display for ChessRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ChessRank::R1 => f.write_str("1"),
            ChessRank::R2 => f.write_str("2"),
            ChessRank::R3 => f.write_str("3"),
            ChessRank::R4 => f.write_str("4"),
            ChessRank::R5 => f.write_str("5"),
            ChessRank::R6 => f.write_str("6"),
            ChessRank::R7 => f.write_str("7"),
            ChessRank::R8 => f.write_str("8"),
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

        crate::chess_like::test::basic_set_get_and_swap(
            board,
            square1,
            square2,
            white_king,
            black_king,
            empty_square,
        );

        println!("Regular board size: {}", std::mem::size_of::<ChessBoard>());
    }
}
