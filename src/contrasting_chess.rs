use crate::chess_like::*;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ContrastingChessPiece {
    King,
    Elephant,
    Bear,
    Horse,
    Dragon,
    Moose,
    Rodent,
}

impl GenericPiece for ContrastingChessPiece {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ContrastingChessBoard {
    board: [RawSquare<ContrastingChessPiece, DefaultColorScheme>; 100],
}

impl GenericBoard for ContrastingChessBoard {
    type PieceType = ContrastingChessPiece;
    type ColorType = DefaultColorScheme;
    type FileType = ContrastingChessFile;
    type RankType = ContrastingChessRank;
    type StorageType = u8;
    type PieceIteratorType = DefaultPieceIter<ContrastingChessBoard>;

    fn side_len() -> Self::StorageType {
        10
    }

    fn new() -> ContrastingChessBoard {
        ContrastingChessBoard {
            board: [RawSquare::empty(); 100],
        }
    }

    #[rustfmt::skip]
    fn default() -> ContrastingChessBoard {
        let mut result = ContrastingChessBoard::new();

        //TODO 

        result
    }

    fn to_storage(file: Self::FileType, rank: Self::RankType) -> u8 {
        Self::FileType::to_storage(file) | Self::RankType::to_storage(rank) << 3
    }

    fn from_storage(storage: u8) -> (Self::FileType, Self::RankType) {
        (
            Self::FileType::from_storage((storage >> 0) & 0b111),
            Self::RankType::from_storage((storage >> 3) & 0b111),
        )
    }

    fn moves_for_piece(&self, pos: u8) -> MoveList<ContrastingChessBoard> {
        let result = smallvec::SmallVec::new();

        result
    }

    fn raw_square_iter(&self) -> DefaultRawSquareIter<ContrastingChessBoard> {
        DefaultRawSquareIter::new(
            0,
            ContrastingChessBoard::side_len() * ContrastingChessBoard::side_len(),
        )
    }

    fn get(&self, pos: u8) -> &RawSquare<ContrastingChessPiece, DefaultColorScheme> {
        &self.board[pos as usize]
    }

    ///Swaps the piece on the board with the mutable piece specified
    fn swap(&mut self, pos: u8, piece: &mut RawSquare<ContrastingChessPiece, DefaultColorScheme>) {
        std::mem::swap(&mut self.board[pos as usize], piece);
    }

    fn set(
        &mut self,
        pos: u8,
        piece: RawSquare<ContrastingChessPiece, DefaultColorScheme>,
    ) -> RawSquare<ContrastingChessPiece, DefaultColorScheme> {
        let result = self.board[pos as usize];
        self.board[pos as usize] = piece;
        result
    }

    /// Returns true if the square at a given position plus offset is empty. Squares off of the
    /// board are always occupied (this function returns false)
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
            ContrastingChessFile::from_storage(dest_file as u8),
            ContrastingChessRank::from_storage(dest_rank as u8),
        );
        let square = self.get(new_pos);
        match square.0 {
            Some(piece) => None,
            None => Some(new_pos),
        }
    }

    fn pieces(&self) -> Self::PieceIteratorType {
        DefaultPieceIter::new(self.raw_square_iter(), None, self.clone())
    }

    fn pieces_for_color(&self, color: Self::ColorType) -> Self::PieceIteratorType {
        DefaultPieceIter::new(self.raw_square_iter(), Some(color), self.clone())
    }
}

impl ToString for ContrastingChessBoard {
    fn to_string(&self) -> String {
        String::new()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ContrastingChessFile {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
}

impl GenericFile<ContrastingChessBoard> for ContrastingChessFile {
    type StorageType = u8;

    fn to_storage(self) -> Self::StorageType {
        match self {
            ContrastingChessFile::A => 0,
            ContrastingChessFile::B => 1,
            ContrastingChessFile::C => 2,
            ContrastingChessFile::D => 3,
            ContrastingChessFile::E => 4,
            ContrastingChessFile::F => 5,
            ContrastingChessFile::G => 6,
            ContrastingChessFile::H => 7,
            ContrastingChessFile::I => 8,
            ContrastingChessFile::J => 9,
        }
    }

    fn from_storage(input: Self::StorageType) -> ContrastingChessFile {
        match input {
            0 => ContrastingChessFile::A,
            1 => ContrastingChessFile::B,
            2 => ContrastingChessFile::C,
            3 => ContrastingChessFile::D,
            4 => ContrastingChessFile::E,
            5 => ContrastingChessFile::F,
            6 => ContrastingChessFile::G,
            7 => ContrastingChessFile::H,
            8 => ContrastingChessFile::I,
            9 => ContrastingChessFile::J,
            _ => panic!(),
        }
    }
}

impl fmt::Display for ContrastingChessFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ContrastingChessFile::A => f.write_str("A"),
            ContrastingChessFile::B => f.write_str("B"),
            ContrastingChessFile::C => f.write_str("C"),
            ContrastingChessFile::D => f.write_str("D"),
            ContrastingChessFile::E => f.write_str("E"),
            ContrastingChessFile::F => f.write_str("F"),
            ContrastingChessFile::G => f.write_str("G"),
            ContrastingChessFile::H => f.write_str("H"),
            ContrastingChessFile::I => f.write_str("I"),
            ContrastingChessFile::J => f.write_str("J"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ContrastingChessRank {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
}

impl GenericRank<ContrastingChessBoard> for ContrastingChessRank {
    type StorageType = u8;

    fn to_storage(self) -> Self::StorageType {
        match self {
            ContrastingChessRank::R1 => 0,
            ContrastingChessRank::R2 => 1,
            ContrastingChessRank::R3 => 2,
            ContrastingChessRank::R4 => 3,
            ContrastingChessRank::R5 => 4,
            ContrastingChessRank::R6 => 5,
            ContrastingChessRank::R7 => 6,
            ContrastingChessRank::R8 => 7,
            ContrastingChessRank::R9 => 8,
            ContrastingChessRank::R10 => 10,
        }
    }

    fn from_storage(input: Self::StorageType) -> ContrastingChessRank {
        match input {
            0 => ContrastingChessRank::R1,
            1 => ContrastingChessRank::R2,
            2 => ContrastingChessRank::R3,
            3 => ContrastingChessRank::R4,
            4 => ContrastingChessRank::R5,
            5 => ContrastingChessRank::R6,
            6 => ContrastingChessRank::R7,
            7 => ContrastingChessRank::R8,
            9 => ContrastingChessRank::R9,
            10 => ContrastingChessRank::R10,
            _ => panic!(),
        }
    }
}

impl fmt::Display for ContrastingChessRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContrastingChessRank::R1 => f.write_str("1"),
            ContrastingChessRank::R2 => f.write_str("2"),
            ContrastingChessRank::R3 => f.write_str("3"),
            ContrastingChessRank::R4 => f.write_str("4"),
            ContrastingChessRank::R5 => f.write_str("5"),
            ContrastingChessRank::R6 => f.write_str("6"),
            ContrastingChessRank::R7 => f.write_str("7"),
            ContrastingChessRank::R8 => f.write_str("8"),
            ContrastingChessRank::R9 => f.write_str("9"),
            ContrastingChessRank::R10 => f.write_str("10"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_set_get_and_swap() {
        let board: ContrastingChessBoard = ContrastingChessBoard::new();
        let empty_square = RawSquare::<ContrastingChessPiece, DefaultColorScheme>::empty();

        let white_king = RawSquare::<ContrastingChessPiece, DefaultColorScheme>::new(
            ContrastingChessPiece::King,
            DefaultColorScheme::While,
        );
        let black_king = RawSquare::<ContrastingChessPiece, DefaultColorScheme>::new(
            ContrastingChessPiece::King,
            DefaultColorScheme::Black,
        );

        let square1 =
            ContrastingChessBoard::to_storage(ContrastingChessFile::A, ContrastingChessRank::R1);
        let square2 =
            ContrastingChessBoard::to_storage(ContrastingChessFile::E, ContrastingChessRank::R4);

        crate::chess_like::test::basic_set_get_and_swap(
            board,
            square1,
            square2,
            white_king,
            black_king,
            empty_square,
        );

        println!(
            "Contrasting board size: {}",
            std::mem::size_of::<ContrastingChessBoard>()
        );
    }
}
