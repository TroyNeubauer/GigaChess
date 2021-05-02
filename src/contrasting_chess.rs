use crate::chess_like::*;

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
    to_move: DefaultColorScheme,
}

pub struct RawMoveIterator {
    board: ContrastingChessBoard,
    last_move: Option<Move<ContrastingChessBoard>>,
}

impl GenericBoard for ContrastingChessBoard {
    type PieceType = ContrastingChessPiece;
    type ColorType = DefaultColorScheme;
    type FileType = ContrastingChessFile;
    type RankType = ContrastingChessRank;
    type StorageType = u8;
    type RawMoveIteratorType = RawMoveIterator;

    fn side_len() -> Self::StorageType {
        10
    }

    fn new() -> ContrastingChessBoard {
        ContrastingChessBoard {
            board: [RawSquare::empty(); 100],
            to_move: DefaultColorScheme::While,
        }
    }

    #[rustfmt::skip]
    fn default() -> ContrastingChessBoard {
        let mut result = ContrastingChessBoard::new();

        //TODO 

        result
    }

    fn to_storage(file: Self::FileType, rank: Self::RankType) -> u8 {
        Self::FileType::to_storage(file) << 3 | Self::RankType::to_storage(rank)
    }

    fn from_storage(storage: u8) -> (Self::FileType, Self::RankType) {
        (
            Self::FileType::from_storage((storage >> 3) & 0b111),
            Self::RankType::from_storage((storage >> 0) & 0b111),
        )
    }

    fn raw_moves_for_piece(&self, pos: u8) -> RawMoveIterator {
        RawMoveIterator {
            board: self.clone(),
            last_move: None,
        }
    }

    fn raw_square_iter(&self) -> DefaultRawSquareIter<ContrastingChessBoard> {
        let max_size = ContrastingChessBoard::side_len() * ContrastingChessBoard::side_len();
        DefaultRawSquareIter::new(max_size, 0)
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

    fn is_move_legal(&self, board_move: Move<ContrastingChessBoard>) -> bool {
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

impl ToString for ContrastingChessBoard {
    fn to_string(&self) -> String {
        String::new()
    }
}

impl Iterator for RawMoveIterator {
    type Item = Move<ContrastingChessBoard>;

    fn next(&mut self) -> Option<Self::Item> {
        None
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

        println!("Contrasting board size: {}", std::mem::size_of::<ContrastingChessBoard>());
    }
}

