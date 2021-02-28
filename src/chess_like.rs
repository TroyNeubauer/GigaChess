pub trait GenericBoard<PieceType, BoardType, IteratorType: Iterator<Item = Move>> {
    fn is_move_legal(&self, board_move: Move) -> bool {
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
    ///Enumerates the 'raw' moves using the movement rules for the piece occupying the requested
    ///square. Raw means the list may contain moves that transitively are illegal because they
    ///cause checks.
    fn raw_moves_for_piece(&self, pos: SquarePos) -> IteratorType;

    ///Returns a list of the locations of the pieces that attack a square. Attacking is defined as
    ///having a legal move that moves takes a potential attacker its starting position to pos
    fn get_attackers_of_square(&self, target_pos: SquarePos) -> Vec<SquarePos>;

    fn square_iter(&self) -> SquareIter;
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct SquarePos {
    pos: u8,
}

#[derive(Copy, Clone, PartialEq, Eq, new)]
pub struct Move {
    src: SquarePos,
    dest: SquarePos,
}

pub trait GenericPiece<Piece, Board, IteratorType: Iterator<Item = Move>> {
    fn legal_moves_it(board: &Board) -> IteratorType;
}

#[derive(new)]
pub struct SquareIter {
    current: u8,
    max_size: u8,
}

impl SquarePos {
    pub fn from_raw(pos: u8) -> SquarePos {
        SquarePos { pos }
    }
}


impl Iterator for SquareIter {
    type Item = SquarePos;

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

