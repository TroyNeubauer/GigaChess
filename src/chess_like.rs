use std::marker::PhantomData;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RawSquare<PieceType, ColorType> {
    pub data: Option<(PieceType, ColorType)>,
}

pub trait GenericRank: Copy + Clone + PartialEq + Eq {}

pub trait GenericFile: Copy + Clone + PartialEq + Eq {}

pub trait GenericStorage: Copy + Clone + PartialEq + Eq + PartialOrd + Ord {}

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
    current: PosType::StorageType,
    max_size: PosType::StorageType,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DefaultSquarePos<
    BoardType: GenericBoard,
    StorageType: GenericStorage,
    FileType: GenericFile,
    RankType: GenericRank,
> {
    pos: StorageType,
    _file: PhantomData<FileType>,
    _rank: PhantomData<RankType>,
    _board: PhantomData<BoardType>,
}

pub trait GenericBoard: Sized + Copy + Clone + PartialEq + Eq {
    type PieceType: GenericPiece;
    type ColorType: GenericColor;
    type PosType: SquarePos;
    type RawMoveIteratorType: Iterator<Item = Move<Self>>;

    fn side_len() -> <<Self as GenericBoard>::PosType as SquarePos>::StorageType;
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

impl<BoardType, StorageType, FileType, RankType> SquarePos
    for DefaultSquarePos<BoardType, StorageType, FileType, RankType>
where
    BoardType: GenericBoard,
    StorageType: GenericStorage,
    FileType: GenericFile,
    RankType: GenericRank,
    BoardType::PosType: SquarePos,
    <BoardType::PosType as SquarePos>::StorageType: GenericStorage,
    <BoardType::PosType as SquarePos>::FileType: GenericFile,
    <BoardType::PosType as SquarePos>::RankType: GenericRank,
{
    type FileType = FileType;
    type RankType = RankType;
    type StorageType = StorageType;
    type BoardType = BoardType;

    fn from_raw(pos: StorageType) -> DefaultSquarePos<BoardType, <BoardType::PosType as SquarePos>::StorageType, FileType, RankType> {
        DefaultSquarePos {
            pos,
            _file: PhantomData::default(),
            _rank: PhantomData::default(),
            _board: PhantomData::default(),
        }
    }

    fn new(
        file: FileType,
        rank: RankType,
    ) -> DefaultSquarePos<BoardType, StorageType, FileType, RankType> {
        DefaultSquarePos {
            pos: file * Self::BoardType::side_len(),
            _file: PhantomData::default(),
            _rank: PhantomData::default(),
            _board: PhantomData::default(),
        }
    }

    fn rank(&self) -> RankType {
        return self.pos % Self::BoardType::side_len();
    }

    fn file(&self) -> FileType {
        return self.pos / Self::BoardType::side_len();
    }

    fn raw_value(&self) -> Self::StorageType {
        self.pos
    }

    fn side_len() -> Self::StorageType {
        BoardType::side_len().into()
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
