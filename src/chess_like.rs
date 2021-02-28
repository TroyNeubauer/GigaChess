use std::ops::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RawSquare<PieceType, ColorType> {
    pub data: Option<(PieceType, ColorType)>,
}

pub trait GenericRank<StorageType: GenericStorage>: Copy + Clone + PartialEq + Eq {
    fn to_storage(self) -> StorageType;
    fn from_storage(input: StorageType) -> Self;
}

pub trait GenericFile<StorageType: GenericStorage>: Copy + Clone + PartialEq + Eq {
    fn to_storage(self) -> StorageType;
    fn from_storage(input: StorageType) -> Self;
}

pub trait GenericStorage: Copy + Clone + PartialEq + Eq + PartialOrd + Ord + Add + AddAssign + Mul + Div + Rem {}

impl GenericStorage for u8 {}

pub trait SquarePos: PartialEq + Eq + Copy + Clone {
    type FileType;
    type RankType;
    type StorageType;
    type BoardType;

    fn from_raw(pos: Self::StorageType) -> Self;
    fn new(file: Self::FileType, rank: Self::RankType) -> Self;

    fn side_len() -> Self::StorageType;

    fn rank(&self) -> Self::RankType;
    fn file(&self) -> Self::FileType;
    fn raw_value(&self) -> Self::StorageType;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, new)]
pub struct Move<BoardType: GenericBoard> {
    pub src: BoardType::PosType,
    pub dest: BoardType::PosType,
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

#[derive(new)]
pub struct SquareIter<PosType: SquarePos> {
    current: <PosType as SquarePos>::StorageType,
    max_size: <PosType as SquarePos>::StorageType,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DefaultSquarePos<BoardType: GenericBoard>
where
    BoardType: GenericBoard,
{
    pos: BoardType::StorageType,
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
    fn raw_moves_for_piece(&self, pos: Self::PosType) -> Self::RawMoveIteratorType;

    ///Returns a list of the locations of the pieces that attack a square. Attacking is defined as
    ///having a legal move that moves takes a potential attacker its starting position to pos
    fn get_attackers_of_square(&self, target_pos: Self::PosType) -> Vec<Self::PosType>;

    fn raw_square_iter(&self) -> SquareIter<Self::PosType>;

    fn get(&self, pos: Self::PosType) -> &RawSquare<Self::PieceType, Self::ColorType>;

    ///Swaps the piece on the board with the mutable piece specified
    fn swap(&self, pos: Self::PosType, piece: &mut RawSquare<Self::PieceType, Self::ColorType>);

    fn set(
        &mut self,
        pos: Self::PosType,
        piece: RawSquare<Self::PieceType, Self::ColorType>,
    ) -> RawSquare<Self::PieceType, Self::ColorType>;
}

impl<BoardType> SquarePos for DefaultSquarePos<BoardType>
where
    BoardType: GenericBoard,
    <BoardType::PosType as SquarePos>::StorageType: GenericStorage,
{
    type FileType = BoardType::FileType;
    type RankType = BoardType::RankType;
    type StorageType = BoardType::StorageType;
    type BoardType = BoardType;

    fn from_raw(
        pos: Self::StorageType,
    ) -> DefaultSquarePos<
        BoardType,
    > {
        DefaultSquarePos {
            pos,
        }
    }

    fn new(
        file: Self::FileType,
        rank: Self::RankType,
    ) -> DefaultSquarePos<BoardType> {
        DefaultSquarePos {
            pos: file.to_storage() * Self::BoardType::side_len(),
        }
    }

    fn rank(&self) -> Self::RankType {
        return self.pos % Self::BoardType::side_len();
    }

    fn file(&self) -> Self::FileType {
        return self.pos / Self::BoardType::side_len();
    }

    fn raw_value(&self) -> Self::StorageType {
        self.pos
    }

    fn side_len() -> Self::StorageType {
        BoardType::side_len()
    }
}

impl<PosType: SquarePos> Iterator for SquareIter<PosType> {
    type Item = PosType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.max_size {
            None
        } else {
            let result = self.current;
            self.current += 1;
            Some(SquarePos::from_raw(result))
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
