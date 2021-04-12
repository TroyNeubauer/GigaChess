use num_traits::Zero;
use std::fmt::Debug;
use std::string::ToString;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RawSquare<PieceType, ColorType> {
    pub data: Option<(PieceType, ColorType)>,
}

//pub type StorageType = u32;

pub trait GenericRank<BoardType: GenericBoard>: Copy + Clone + Debug + PartialEq + Eq
where
    BoardType::StorageType: num_traits::PrimInt,
{
    type StorageType: num_traits::PrimInt;

    fn to_storage(self) -> Self::StorageType;
    fn from_storage(input: Self::StorageType) -> Self;
}

pub trait GenericFile<BoardType: GenericBoard>: Copy + Clone + Debug + PartialEq + Eq
where
    BoardType::StorageType: num_traits::PrimInt,
{
    type StorageType: num_traits::PrimInt;

    fn to_storage(self) -> Self::StorageType;
    fn from_storage(input: Self::StorageType) -> Self;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, new)]
pub struct Move<BoardType: GenericBoard>
where
    BoardType::StorageType: num_traits::PrimInt,
{
    pub src: BoardType::StorageType,
    pub dest: BoardType::StorageType,
}

pub trait GenericPiece: PartialEq + Eq + Copy + Debug {}

pub trait GenericColor: PartialEq + Eq + Copy + Debug {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DefaultColorScheme {
    While,
    Black,
}

impl GenericColor for DefaultColorScheme {}

//Beefy structs

#[derive(new)]
pub struct SquareIter<BoardType: GenericBoard> {
    current: BoardType::StorageType,
    max_size: BoardType::StorageType,
}

pub trait GenericBoard: Sized + Copy + Clone + PartialEq + Eq + ToString + Debug {
    type PieceType: GenericPiece;
    type ColorType: GenericColor;
    type FileType: GenericFile<Self>;
    type RankType: GenericRank<Self>;
    type StorageType: num_traits::PrimInt;
    type RawMoveIteratorType: Iterator<Item = Move<Self>>;

    fn side_len() -> Self::StorageType;
    ///Creates an empty board
    fn new() -> Self;

    ///Creates a board with pieces placed in their default positions
    fn default() -> Self;

    fn to_storage(file: Self::FileType, rank: Self::RankType) -> Self::StorageType;
    fn from_storage(storage: Self::StorageType) -> (Self::FileType, Self::RankType);

    fn is_move_legal(&self, board_move: Move<Self>) -> bool;

    ///Enumerates the 'raw' moves using the movement rules for the piece occupying the requested
    ///square. Raw means the list may contain moves that transitively are illegal because they
    ///cause checks.
    fn raw_moves_for_piece(&self, pos: Self::StorageType) -> Self::RawMoveIteratorType;

    ///Returns a list of the locations of the pieces that attack a square. Attacking is defined as
    ///having a legal move that moves takes a potential attacker its starting position to pos
    fn get_attackers_of_square(&self, target_pos: Self::StorageType) -> Vec<Self::StorageType>;

    fn raw_square_iter(&self) -> SquareIter<Self>;

    fn get(&self, pos: Self::StorageType) -> &RawSquare<Self::PieceType, Self::ColorType>;

    ///Swaps the piece on the board with the mutable piece specified
    fn swap(
        &mut self,
        pos: Self::StorageType,
        piece: &mut RawSquare<Self::PieceType, Self::ColorType>,
    );

    fn set(
        &mut self,
        pos: Self::StorageType,
        piece: RawSquare<Self::PieceType, Self::ColorType>,
    ) -> RawSquare<Self::PieceType, Self::ColorType>;

    fn to_move(&self) -> Self::ColorType;
}

enum MoveError {
    Rank,
    File,
    Both,
}

impl<BoardType: GenericBoard> Iterator for SquareIter<BoardType>
where
    BoardType: GenericBoard,
    BoardType::StorageType: num_traits::PrimInt,
{
    type Item = BoardType::StorageType;

    fn next(&mut self) -> Option<BoardType::StorageType> {
        if self.current >= self.max_size {
            None
        } else {
            self.current = self.current + BoardType::StorageType::zero();
            Some(self.current)
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

#[cfg(test)]
pub mod test {
    use super::*;

    pub fn basic_set_get_and_swap<Board: GenericBoard>(
        mut board: Board,
        square1: Board::StorageType,
        square2: Board::StorageType,
        king1: RawSquare<Board::PieceType, Board::ColorType>,
        king2: RawSquare<Board::PieceType, Board::ColorType>,
        empty_square: RawSquare<Board::PieceType, Board::ColorType>,
    ) {
        assert_eq!(board.get(square1), &empty_square);
        assert_eq!(board.get(square2), &empty_square);

        let last_piece = board.set(square1, king1);
        assert_eq!(last_piece, empty_square);
        assert_eq!(board.get(square1), &king1);

        //Start with a black king in our "hand" then swap it with the white king on E4
        let mut hand_piece = king2;

        board.swap(square1, &mut hand_piece);
        assert_eq!(hand_piece, king1);
        assert_eq!(board.get(square1), &king2);
    }
}
