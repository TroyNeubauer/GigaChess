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
    Weasel,
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
    /*
        Weasel,
    */

    fn default() -> ContrastingChessBoard {
        let mut result = ContrastingChessBoard::new();
        result.set(
            ContrastingChessBoard::to_storage(ContrastingChessFile::A, ContrastingChessRank::R1),
            RawSquare::new(ContrastingChessPiece::Bear, DefaultColorScheme::While),
        );
        result.set(
            ContrastingChessBoard::to_storage(ContrastingChessFile::J, ContrastingChessRank::R1),
            RawSquare::new(ContrastingChessPiece::Bear, DefaultColorScheme::While),
        );

        result.set(
            ContrastingChessBoard::to_storage(ContrastingChessFile::B, ContrastingChessRank::R1),
            RawSquare::new(ContrastingChessPiece::Moose, DefaultColorScheme::While),
        );
        result.set(
            ContrastingChessBoard::to_storage(ContrastingChessFile::I, ContrastingChessRank::R1),
            RawSquare::new(ContrastingChessPiece::Moose, DefaultColorScheme::While),
        );

        result.set(
            ContrastingChessBoard::to_storage(ContrastingChessFile::C, ContrastingChessRank::R1),
            RawSquare::new(ContrastingChessPiece::Dragon, DefaultColorScheme::While),
        );

        result.set(
            ContrastingChessBoard::to_storage(ContrastingChessFile::H, ContrastingChessRank::R1),
            RawSquare::new(ContrastingChessPiece::Dragon, DefaultColorScheme::While),
        );

        result.set(
            ContrastingChessBoard::to_storage(ContrastingChessFile::D, ContrastingChessRank::R1),
            RawSquare::new(ContrastingChessPiece::Horse, DefaultColorScheme::While),
        );
        result.set(
            ContrastingChessBoard::to_storage(ContrastingChessFile::G, ContrastingChessRank::R1),
            RawSquare::new(ContrastingChessPiece::Horse, DefaultColorScheme::While),
        );

        result.set(
            ContrastingChessBoard::to_storage(ContrastingChessFile::E, ContrastingChessRank::R1),
            RawSquare::new(ContrastingChessPiece::Elephant, DefaultColorScheme::While),
        );
        result.set(
            ContrastingChessBoard::to_storage(ContrastingChessFile::F, ContrastingChessRank::R1),
            RawSquare::new(ContrastingChessPiece::King, DefaultColorScheme::While),
        );

        result.set(
            ContrastingChessBoard::to_storage(ContrastingChessFile::A, ContrastingChessRank::R2),
            RawSquare::new(ContrastingChessPiece::Elephant, DefaultColorScheme::While),
        );

        result.set(
            ContrastingChessBoard::to_storage(ContrastingChessFile::J, ContrastingChessRank::R2),
            RawSquare::new(ContrastingChessPiece::Elephant, DefaultColorScheme::While),
        );

        let pos =
            ContrastingChessBoard::to_storage(ContrastingChessFile::A, ContrastingChessRank::R2);
        for x in 1..9 {
            let pos = result.offset_pos(pos, x, 0).unwrap();
            result.set(
                pos,
                RawSquare::new(ContrastingChessPiece::Weasel, DefaultColorScheme::While),
            );
        }

        //Black
        result.set(
            ContrastingChessBoard::to_storage(ContrastingChessFile::A, ContrastingChessRank::R10),
            RawSquare::new(ContrastingChessPiece::Bear, DefaultColorScheme::Black),
        );
        result.set(
            ContrastingChessBoard::to_storage(ContrastingChessFile::J, ContrastingChessRank::R10),
            RawSquare::new(ContrastingChessPiece::Bear, DefaultColorScheme::Black),
        );

        result.set(
            ContrastingChessBoard::to_storage(ContrastingChessFile::B, ContrastingChessRank::R10),
            RawSquare::new(ContrastingChessPiece::Moose, DefaultColorScheme::Black),
        );
        result.set(
            ContrastingChessBoard::to_storage(ContrastingChessFile::I, ContrastingChessRank::R10),
            RawSquare::new(ContrastingChessPiece::Moose, DefaultColorScheme::Black),
        );

        result.set(
            ContrastingChessBoard::to_storage(ContrastingChessFile::C, ContrastingChessRank::R10),
            RawSquare::new(ContrastingChessPiece::Dragon, DefaultColorScheme::Black),
        );

        result.set(
            ContrastingChessBoard::to_storage(ContrastingChessFile::H, ContrastingChessRank::R10),
            RawSquare::new(ContrastingChessPiece::Dragon, DefaultColorScheme::Black),
        );

        result.set(
            ContrastingChessBoard::to_storage(ContrastingChessFile::D, ContrastingChessRank::R10),
            RawSquare::new(ContrastingChessPiece::Horse, DefaultColorScheme::Black),
        );
        result.set(
            ContrastingChessBoard::to_storage(ContrastingChessFile::G, ContrastingChessRank::R10),
            RawSquare::new(ContrastingChessPiece::Horse, DefaultColorScheme::Black),
        );

        result.set(
            ContrastingChessBoard::to_storage(ContrastingChessFile::E, ContrastingChessRank::R10),
            RawSquare::new(ContrastingChessPiece::Elephant, DefaultColorScheme::Black),
        );
        result.set(
            ContrastingChessBoard::to_storage(ContrastingChessFile::F, ContrastingChessRank::R10),
            RawSquare::new(ContrastingChessPiece::King, DefaultColorScheme::Black),
        );

        result.set(
            ContrastingChessBoard::to_storage(ContrastingChessFile::A, ContrastingChessRank::R9),
            RawSquare::new(ContrastingChessPiece::Elephant, DefaultColorScheme::Black),
        );

        result.set(
            ContrastingChessBoard::to_storage(ContrastingChessFile::J, ContrastingChessRank::R9),
            RawSquare::new(ContrastingChessPiece::Elephant, DefaultColorScheme::Black),
        );

        let pos =
            ContrastingChessBoard::to_storage(ContrastingChessFile::A, ContrastingChessRank::R9);
        for x in 1..9 {
            let pos = result.offset_pos(pos, x, 0).unwrap();
            result.set(
                pos,
                RawSquare::new(ContrastingChessPiece::Weasel, DefaultColorScheme::Black),
            );
        }

        result
    }

    fn to_storage(file: Self::FileType, rank: Self::RankType) -> u8 {
        Self::FileType::to_storage(file) + Self::RankType::to_storage(rank) * 10
    }

    fn from_storage(storage: u8) -> (Self::FileType, Self::RankType) {
        let file = storage % 10;
        let rank = storage / 10;
        (
            Self::FileType::from_storage(file),
            Self::RankType::from_storage(rank),
        )
    }

    fn moves_for_piece(&self, pos: u8) -> MoveList<ContrastingChessBoard> {
        let mut result = smallvec::SmallVec::new();
        match self.get(pos).0 {
            Some(piece) => match piece.piece {
                ContrastingChessPiece::King => {
                    //The king can move one anywhere in a 3x3 box centered around its current
                    //position except for the square it is currently on
                    for x in -1..2 {
                        for y in -1..2 {
                            if x == 0 && y == 0 {
                                continue;
                            }
                            //Always add moves since we control the bounds
                            self.add_move(pos, x, y, &mut result, |_| {
                                AddMoveResult::AddMoveKeepIterating
                            });
                        }
                    }
                }

                ContrastingChessPiece::Elephant => {
                    let func = |piece: RawSquare<Self::PieceType, Self::ColorType>| match piece.0 {
                        Some(piece) => {
                            if piece.piece == ContrastingChessPiece::Weasel {
                                AddMoveResult::NoAddMove
                            } else {
                                AddMoveResult::AddMoveStopIterating
                            }
                        }
                        None => AddMoveResult::AddMoveKeepIterating,
                    };
                    let mut x = 1;
                    loop {
                        if !self.add_move(pos, x, 0, &mut result, func) {
                            break;
                        }
                        x += 1;
                    }
                    let mut x = -1;
                    loop {
                        if !self.add_move(pos, x, 0, &mut result, func) {
                            break;
                        }
                        x -= 1;
                    }
                    let mut y = 1;
                    loop {
                        if !self.add_move(pos, 0, y, &mut result, func) {
                            break;
                        }
                        y += 1;
                    }
                    let mut y = -1;
                    loop {
                        if !self.add_move(pos, 0, y, &mut result, func) {
                            break;
                        }
                        y -= 1;
                    }

                    let mut y = -1;
                    let mut x = -1;
                    loop {
                        if !self.add_move(pos, x, y, &mut result, func) {
                            break;
                        }
                        y -= 1;
                        x -= 1;
                    }
                    let mut y = 1;
                    let mut x = -1;
                    loop {
                        if !self.add_move(pos, x, y, &mut result, func) {
                            break;
                        }
                        y += 1;
                        x -= 1;
                    }
                    let mut y = 1;
                    let mut x = 1;
                    loop {
                        if !self.add_move(pos, x, y, &mut result, func) {
                            break;
                        }
                        y += 1;
                        x += 1;
                    }
                    let mut y = -1;
                    let mut x = 1;
                    loop {
                        if !self.add_move(pos, x, y, &mut result, func) {
                            break;
                        }
                        y -= 1;
                        x += 1;
                    }
                }
                ContrastingChessPiece::Bear => {
                    let func = |piece: RawSquare<Self::PieceType, Self::ColorType>| match piece.0 {
                        Some(piece) => AddMoveResult::AddMoveStopIterating,
                        None => AddMoveResult::AddMoveKeepIterating,
                    };
                    let mut y = -1;
                    let mut x = -1;
                    loop {
                        if !self.add_move(pos, x, y, &mut result, func) || x < -4 {
                            break;
                        }
                        y -= 1;
                        x -= 1;
                    }
                    let mut y = 1;
                    let mut x = -1;
                    loop {
                        if !self.add_move(pos, x, y, &mut result, func) || x < -4 {
                            break;
                        }
                        y += 1;
                        x -= 1;
                    }
                    let mut y = 1;
                    let mut x = 1;
                    loop {
                        if !self.add_move(pos, x, y, &mut result, func) || x > 4 {
                            break;
                        }
                        y += 1;
                        x += 1;
                    }
                    let mut y = -1;
                    let mut x = 1;
                    loop {
                        if !self.add_move(pos, x, y, &mut result, func) || x > 4 {
                            break;
                        }
                        y -= 1;
                        x += 1;
                    }
                }
                ContrastingChessPiece::Horse => {}
                ContrastingChessPiece::Dragon => {}
                ContrastingChessPiece::Moose => {}
                ContrastingChessPiece::Weasel => {
                    let forward_direction = if piece.color == DefaultColorScheme::While {
                        1
                    } else {
                        -1
                    };
                    let forward = |piece: RawSquare<Self::PieceType, Self::ColorType>| match piece.0
                    {
                        Some(_piece) => AddMoveResult::AddMoveStopIterating,
                        None => AddMoveResult::AddMoveStopIterating,
                    };

                    self.add_move(pos, 0, forward_direction, &mut result, forward);
                }
            },
            None => {}
        }

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

    fn offset_pos(
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
        Some(Self::to_storage(
            ContrastingChessFile::from_storage(dest_file as u8),
            ContrastingChessRank::from_storage(dest_rank as u8),
        ))
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
            ContrastingChessRank::R10 => 9,
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
            8 => ContrastingChessRank::R9,
            9 => ContrastingChessRank::R10,
            _ => {
                println!("Got bad rank {}", input);
                panic!();
            }
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
