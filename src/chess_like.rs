use num_traits::One;
use std::fmt;
use std::fmt::Debug;
use std::string::ToString;

use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RawPiece<PieceType, ColorType> {
    pub piece: PieceType,
    pub color: ColorType,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RawSquare<PieceType, ColorType>(pub Option<RawPiece<PieceType, ColorType>>);

pub trait GenericStorage:
    num_traits::PrimInt
    + Debug
    + TryFrom<usize>
    + TryFrom<isize>
    + Into<isize>
    + PartialEq
    + Eq
    + fmt::Display
{
}
impl<
        T: num_traits::PrimInt
            + Debug
            + TryFrom<usize>
            + TryFrom<isize>
            + Into<isize>
            + PartialEq
            + Eq
            + fmt::Display,
    > GenericStorage for T
{
}

pub trait GenericRank<BoardType: GenericBoard>:
    Copy + Clone + Debug + PartialEq + Eq + fmt::Display
where
    BoardType::StorageType: GenericStorage,
{
    type StorageType: GenericStorage;

    ///Storage type contains just the data for indicating this rank, although usually in the
    ///context of Generic board it indicates a rank and file
    fn to_storage(self) -> Self::StorageType;
    fn from_storage(input: Self::StorageType) -> Self;
}

pub trait GenericFile<BoardType: GenericBoard>:
    Copy + Clone + Debug + PartialEq + Eq + fmt::Display
where
    BoardType::StorageType: GenericStorage,
{
    type StorageType: GenericStorage;

    fn to_storage(self) -> Self::StorageType;
    fn from_storage(input: Self::StorageType) -> Self;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, new)]
pub struct Move<BoardType: GenericBoard>
where
    BoardType::StorageType: GenericStorage,
{
    pub src: BoardType::StorageType,
    pub dest: BoardType::StorageType,
}

pub trait GenericPiece: PartialEq + Eq + Copy + Debug {}

pub trait GenericColor:
    PartialEq + Eq + Copy + Debug + TryFrom<usize> + Into<usize> + fmt::Display
{
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DefaultColorScheme {
    While,
    Black,
}

impl GenericColor for DefaultColorScheme {}

#[derive(new)]
pub struct DefaultRawSquareIter<BoardType: GenericBoard> {
    current: BoardType::StorageType,
    max_size: BoardType::StorageType,
}

#[derive(new)]
pub struct DefaultPieceIter<BoardType: GenericBoard> {
    raw_squares: DefaultRawSquareIter<BoardType>,
    color: Option<BoardType::ColorType>,

    //TODO use a reference instead of copying
    board: BoardType,
}

pub enum AddMoveResult {
    AddMoveStopIterating,
    AddMoveKeepIterating,
    NoAddMove,
}

pub type MoveList<BoardType> = smallvec::SmallVec<[<BoardType as GenericBoard>::StorageType; 16]>;

pub trait GenericBoard: Sized + Copy + Clone + PartialEq + Eq + ToString + Debug {
    type PieceType: GenericPiece;
    type ColorType: GenericColor;
    type FileType: GenericFile<Self>;
    type RankType: GenericRank<Self>;
    type StorageType: GenericStorage;
    type PieceIteratorType: Iterator<Item = Self::StorageType>;

    fn side_len() -> Self::StorageType;
    ///Creates an empty board
    fn new() -> Self;

    ///Creates a board with pieces placed in their default positions
    fn default() -> Self;

    fn to_storage(file: Self::FileType, rank: Self::RankType) -> Self::StorageType;
    fn from_storage(storage: Self::StorageType) -> (Self::FileType, Self::RankType);

    fn is_move_legal(&self, to_move: Self::ColorType, m: Move<Self>) -> bool {
        match self.get(m.src).0 {
            Some(m) => {
                if m.color != to_move {
                    //We can only move our own pieces
                    return false;
                }
            }
            None => {
                //Any move starting from an empty square is invalid
                return false;
            }
        };

        let moves = self.moves_for_piece(m.src);
        println!(
            "Got moves {} for piece {}. Playing as colol {}",
            PrintMoves::<Self>(&moves),
            m,
            to_move
        );
        for generated_move in moves.iter() {
            if *generated_move == m.dest {
                //We found a move starting at src and ending at dest in the list of legal moves
                return true;
            }
        }
        false
    }

    ///Enumerates the 'raw' moves using the movement rules for the piece occupying the requested
    ///square.
    fn moves_for_piece(&self, pos: Self::StorageType) -> MoveList<Self>;

    ///Enumerates all the pieces on the board
    fn pieces(&self) -> Self::PieceIteratorType;

    ///Enumerates all the pieces on the board
    fn pieces_for_color(&self, color: Self::ColorType) -> Self::PieceIteratorType;

    ///Returns a list of the locations of the pieces that attack a square. Attacking is defined as
    ///having a legal move that takes a potential attacker its starting position to `target_pos`
    fn get_attackers_of_square(&self, target_pos: Self::StorageType) -> Vec<Self::StorageType> {
        let mut result = Vec::new();
        for piece_pos in self.pieces() {
            let moves = self.moves_for_piece(piece_pos);
            for m in moves {
                if m == target_pos {
                    result.push(piece_pos);
                }
            }
        }
        result
    }

    fn raw_square_iter(&self) -> DefaultRawSquareIter<Self>;

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

    /// Clears all pieces off the board
    fn clear(&mut self) {
        for square in self.raw_square_iter() {
            self.set(
                square,
                RawSquare::<Self::PieceType, Self::ColorType>::empty(),
            );
        }
    }

    fn is_square_empty(&self, pos: Self::StorageType) -> bool {
        let square = self.get(pos);
        square.0.is_none()
    }

    /// Attempts to add file and rank to pos, returns Some(sum_pos) if the square is within the bounds
    /// of the board
    fn offset_pos(
        &self,
        pos: Self::StorageType,
        file: isize,
        rank: isize,
    ) -> Option<Self::StorageType>;

    /// Computes pos + file + rank and returns Some if the square is on the board and empty, None
    /// otherwise
    fn is_square_empty_offset(
        &self,
        pos: Self::StorageType,
        file: isize,
        rank: isize,
    ) -> Option<Self::StorageType> {
        match self.offset_pos(pos, file, rank) {
            Some(pos) => match self.get(pos).0 {
                Some(_square) => None,
                None => Some(pos),
            },
            None => return None,
        }
    }

    /// If pos + file + rank is on the board, f is called to determine weather on not the move
    /// should be added. f returns a tuple of (add, result) if add is true the move is added to
    /// the list and result is returned, otherwise false is returned.
    fn add_move(
        &self,
        pos: Self::StorageType,
        file: isize,
        rank: isize,
        moves: &mut MoveList<Self>,
        f: impl Fn(RawSquare<Self::PieceType, Self::ColorType>) -> AddMoveResult,
    ) -> bool {
        match self.offset_pos(pos, file, rank) {
            Some(pos) => match f(self.get(pos).clone()) {
                AddMoveResult::AddMoveKeepIterating => {
                    moves.push(pos);
                    true
                }
                AddMoveResult::AddMoveStopIterating => {
                    moves.push(pos);
                    false
                }
                AddMoveResult::NoAddMove => false,
            },
            None => false,
        }
    }

    /// After computing the dest move square by adding file and rank to pos, adds the dest move
    /// to move list if its square is empty and within the bounds of the board
    /// Returns true if the move was added to the move list, false if the square was off the board
    /// or occupied
    fn try_add_capture_or_move(
        &self,
        pos: Self::StorageType,
        file: isize,
        rank: isize,
        moves: &mut MoveList<Self>,
    ) -> bool {
        match self.is_square_empty_offset(pos, file, rank) {
            Some(dest_square) => {
                moves.push(dest_square);
                true
            }
            None => false,
        }
    }

    /// Makes a basic move on the board without checking for legality
    /// Returns what resided on the destination square before this move
    fn apply_raw_move(&mut self, m: Move<Self>) -> RawSquare<Self::PieceType, Self::ColorType> {
        //Move an empty square to where this piece came from
        let piece = self.set(m.src, RawSquare(None));
        // then move the piece from the source square to the dest square, returning what was captured
        self.set(m.dest, piece)
    }
}

enum MoveError {
    Rank,
    File,
    Both,
}

impl<BoardType: GenericBoard> Iterator for DefaultRawSquareIter<BoardType>
where
    BoardType: GenericBoard,
    BoardType::StorageType: GenericStorage,
{
    type Item = BoardType::StorageType;

    fn next(&mut self) -> Option<BoardType::StorageType> {
        if self.current >= self.max_size {
            None
        } else {
            let result = self.current;
            self.current = self.current + BoardType::StorageType::one();
            Some(result)
        }
    }
}

impl<BoardType: GenericBoard> Iterator for DefaultPieceIter<BoardType>
where
    BoardType: GenericBoard,
    BoardType::StorageType: GenericStorage,
{
    type Item = BoardType::StorageType;

    fn next(&mut self) -> Option<BoardType::StorageType> {
        let square = self.raw_squares.next();
        match square {
            Some(square) => None,
            None => None,
        }
    }
}

impl<PieceType, ColorType> RawSquare<PieceType, ColorType> {
    pub fn empty() -> RawSquare<PieceType, ColorType> {
        RawSquare(None)
    }

    pub fn new(piece: PieceType, color: ColorType) -> RawSquare<PieceType, ColorType> {
        RawSquare(Some(RawPiece { piece, color }))
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

impl fmt::Display for DefaultColorScheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DefaultColorScheme::While => write!(f, "White"),
            DefaultColorScheme::Black => write!(f, "Black"),
        }
    }
}

impl<BoardType: GenericBoard> fmt::Display for Move<BoardType> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} to {}", self.src, self.dest)
    }
}

impl TryFrom<usize> for DefaultColorScheme {
    //Give the user their size back on failure
    type Error = usize;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        String::from("");
        match value {
            0 => Ok(DefaultColorScheme::While),
            1 => Ok(DefaultColorScheme::Black),
            _ => Err(value),
        }
    }
}

impl Into<usize> for DefaultColorScheme {
    fn into(self) -> usize {
        match self {
            DefaultColorScheme::While => 0,
            DefaultColorScheme::Black => 1,
        }
    }
}

struct PrintMoves<'a, BoardType: GenericBoard>(&'a MoveList<BoardType>);

impl<'a, BoardType: GenericBoard> fmt::Display for PrintMoves<'a, BoardType>
where
    BoardType: GenericBoard,
    BoardType::StorageType: GenericStorage,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[")?;
        let mut first = true;
        for item in self.0 {
            if !first {
                f.write_str(", ")?;
            }
            first = false;

            let (file, rank) = BoardType::from_storage(*item);
            fmt::Display::fmt(&file, f)?;
            fmt::Display::fmt(&rank, f)?;
        }
        f.write_str("]")?;
        Ok(())
    }
}
