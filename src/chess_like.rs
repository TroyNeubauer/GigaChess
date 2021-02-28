use std::marker::PhantomData;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DefaultColorScheme {
    While,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RawSquare<PieceType, ColorType> {
    pub data: Option<(PieceType, ColorType)>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SquarePos<BoardType> {
    pos: u8,
    _board_type: PhantomData<BoardType>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, new)]
pub struct Move<BoardType> {
    pub src: SquarePos<BoardType>,
    pub dest: SquarePos<BoardType>,
}

pub trait GenericPiece<Piece, Board, IteratorType: Iterator<Item = Move<Board>>> {
    fn legal_moves_it(board: &Board) -> IteratorType;
}

#[derive(new)]
pub struct SquareIter<BoardType> {
    current: u8,
    max_size: u8,
    _board_type: PhantomData<BoardType>,
}

pub trait GenericBoard: Sized + PartialEq + Eq {
    type PieceType: PartialEq + Eq + Copy;
    type ColorType: PartialEq + Eq + Copy;
    type RawMoveIteratorType: Iterator<Item = Move<Self>>;

    fn side_len() -> u8;
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
}

impl<BoardType: GenericBoard> SquarePos<BoardType> {
    pub fn from_raw(pos: u8) -> SquarePos<BoardType> {
        SquarePos {
            pos,
            _board_type: PhantomData::default(),
        }
    }

    pub fn new(file: u8, rank: u8) -> SquarePos<BoardType> {
        SquarePos {
            pos: file * BoardType::side_len(),
            _board_type: PhantomData::default(),
        }
    }

    pub fn rank(&self) -> u8 {
        return self.pos % BoardType::side_len();
    }

    pub fn file(&self) -> u8 {
        return self.pos / BoardType::side_len();
    }

    pub fn raw_value(&self) -> u8 {
        self.pos
    }
}

impl<BoardType: GenericBoard> Iterator for SquareIter<BoardType> {
    type Item = SquarePos<BoardType>;

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
}



