use std::convert::Into;
use std::fmt::Debug;
use std::ops::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RawSquare<PieceType, ColorType> {
    pub data: Option<(PieceType, ColorType)>,
}

pub trait GenericRank<StorageType: GenericStorage>: Copy + Clone + Debug + PartialEq + Eq {
    fn to_storage(self) -> StorageType;
    fn from_storage(input: StorageType) -> Self;
}

pub trait GenericFile<StorageType: GenericStorage>: Copy + Clone + Debug + PartialEq + Eq {
    fn to_storage(self) -> StorageType;
    fn from_storage(input: StorageType) -> Self;
}

pub trait GenericStorage:
    Copy + Clone + Debug + PartialEq + Eq + PartialOrd + Ord + Add + AddAssign + Mul + Div + Rem
{
    //We need to implement our own math operations to avoid fussing around with
    //std::ops::Mul::Output in generic code
    fn multiply(self, rhs: Self) -> Self;
    fn divide(self, rhs: Self) -> Self;
    fn remainder(self, rhs: Self) -> Self;

    fn increment(&mut self, rhs: Self);

    fn one() -> Self;
}

impl GenericStorage for u8 {
    fn multiply(self, rhs: Self) -> Self {
        self.mul(rhs)
    }

    fn divide(self, rhs: Self) -> Self {
        self.div(rhs)
    }

    fn remainder(self, rhs: Self) -> Self {
        self.rem(rhs)
    }

    fn increment(&mut self, rhs: Self) {
        self.add_assign(rhs)
    }

    fn one() -> Self {
        1
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, new)]
pub struct Move<BoardType: GenericBoard> {
    pub src: SquarePos<BoardType>,
    pub dest: SquarePos<BoardType>,
}

pub trait GenericPiece: PartialEq + Eq + Copy {}

pub trait GenericColor: PartialEq + Eq + Copy {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DefaultColorScheme {
    While,
    Black,
}

impl GenericColor for DefaultColorScheme {}

//Beefy structs

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SquarePos<BoardType: GenericBoard> {
    pos: BoardType::StorageType,
}

#[derive(new)]
pub struct SquareIter<BoardType: GenericBoard> {
    current: BoardType::StorageType,
    max_size: BoardType::StorageType,
}

pub trait GenericBoard: Sized + Copy + Clone + PartialEq + Eq {
    type PieceType: GenericPiece;
    type ColorType: GenericColor;
    type StorageType: GenericStorage;
    type FileType: GenericFile<Self::StorageType>;
    type RankType: GenericRank<Self::StorageType>;
    type RawMoveIteratorType: Iterator<Item = Move<Self>>;

    fn side_len() -> Self::StorageType;
    ///Creates an empty board
    fn new() -> Self;

    ///Creates a board with pieces placed in their default positions
    fn default() -> Self;

    fn is_move_legal(&self, board_move: Move<Self>) -> bool;

    ///Enumerates the 'raw' moves using the movement rules for the piece occupying the requested
    ///square. Raw means the list may contain moves that transitively are illegal because they
    ///cause checks.
    fn raw_moves_for_piece(&self, pos: SquarePos<Self>) -> Self::RawMoveIteratorType;

    ///Returns a list of the locations of the pieces that attack a square. Attacking is defined as
    ///having a legal move that moves takes a potential attacker its starting position to pos
    fn get_attackers_of_square(&self, target_pos: SquarePos<Self>) -> Vec<SquarePos<Self>>;

    fn raw_square_iter(&self) -> SquareIter<Self>;

    fn get(&self, pos: SquarePos<Self>) -> &RawSquare<Self::PieceType, Self::ColorType>;

    ///Swaps the piece on the board with the mutable piece specified
    fn swap(&self, pos: SquarePos<Self>, piece: &mut RawSquare<Self::PieceType, Self::ColorType>);

    fn set(
        &mut self,
        pos: SquarePos<Self>,
        piece: RawSquare<Self::PieceType, Self::ColorType>,
    ) -> RawSquare<Self::PieceType, Self::ColorType>;
}

impl<BoardType> SquarePos<BoardType>
where
    BoardType: GenericBoard,
    BoardType::StorageType: GenericStorage,
{
    pub fn from_raw(pos: BoardType::StorageType) -> SquarePos<BoardType> {
        SquarePos { pos }
    }

    pub fn new(file: BoardType::FileType, rank: BoardType::RankType) -> SquarePos<BoardType> {
        SquarePos {
            pos: BoardType::StorageType::from(file.to_storage().multiply(BoardType::side_len())),
        }
    }

    pub fn rank(&self) -> BoardType::RankType {
        BoardType::RankType::from_storage(self.pos.remainder(BoardType::side_len()))
    }

    pub fn file(&self) -> BoardType::FileType {
        BoardType::FileType::from_storage(self.pos.divide(BoardType::side_len()))
    }

    pub fn raw_value(&self) -> BoardType::StorageType {
        self.pos
    }

    pub fn side_len() -> BoardType::StorageType {
        BoardType::side_len()
    }
}

impl<BoardType: GenericBoard> Iterator for SquareIter<BoardType> {
    type Item = SquarePos<BoardType>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.max_size {
            None
        } else {
            self.current.increment(BoardType::StorageType::one());
            Some(SquarePos::from_raw(self.current))
        }
    }
}

impl<PieceType, ColorType> RawSquare<PieceType, ColorType> {
    pub fn empty() -> RawSquare<PieceType, ColorType> {
        RawSquare { data: None }
    }

    pub fn new(piece: PieceType, color: ColorType) -> RawSquare<PieceType, ColorType> {
        RawSquare {
            data: Some((piece, color)),
        }
    }
}
