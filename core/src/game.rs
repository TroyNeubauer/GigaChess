use chrono::prelude::*;
use smallvec::SmallVec;

use serde::{Deserialize, Serialize};

/// Stores a square on the board. Generic over all game kinds
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RawSquarePosition(u32);

/// A basic move, generic over all game kinds
#[derive(Serialize, Deserialize)]
pub struct RawMove {
    pub src: RawSquarePosition,
    pub dst: RawSquarePosition,
}

/// The identifier for a particular color. Values are game kind dependent but must be sequential
/// starting from 0 in move order. For example, in chess white is id 0, and black is is 1.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct ColorKind(u32);

/// A game's unique identifier. Never re-used within the same execution of this library
pub type ID = u64;

/// The kind of game. Currently only chess is supported hoverer more kinds may be added in the
/// future (eg. Contrasting chess (10x10 chess like game with different pieces), additive chess,
/// etc.)
/// Games determine the size of the board, the pieces used, and the moves that govern the game and
/// piece movement
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Kind {
    Chess,
}

/// Variants are small changes to a base chess game. Variant cannot change the pieces used, or the
/// size of the board. However, they can change the starting position and the rules.
/// Not all Variants are supported by a game type (for example using Chess960 with ContrastingChess
/// makes no sense and is not supported)
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Variant {
    Chess960,

    NoCastling,
}

#[derive(Serialize, Deserialize)]
pub enum GameEndCause {
    /// The king of the player to move is in check and has no legal moves
    Checkmate,

    /// The king of the player to move is not in check but has no legal moves
    Stalemate,

    /// Insufficient material for the game to have a decisive ending. Different from stalemate
    DeadPosition,

    /// The players agreed to a draw
    DrawOffer,

    /// The loosing player resigned
    Resign,

    /// The player to move ran out of time
    Flag,

    /// The player to move tried to make an illegal move
    IllegalMove(RawMove),
}

/// The clocks for all players in the game
#[derive(Serialize, Deserialize)]
pub struct Clocks {
    /// The clocks of the player's participating in the game
    data: SmallVec<[Clock; 2]>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum TimeFormat {
    Timed {
        /// The initial time a player gets on their clock in nanoseconds
        initial_nanos: u64,
        /// The number of nanoseconds added to a player's clock each move
        increment_nanos: u64,
        /// The number of nanoseconds a player's clock is delayed from ticking down at the beginning
        /// of each move
        delay_nanos: u64,
    },
    Unlimited,
}

/// The clock of a given player. Really just the points in time they made a move. The time of move
/// one is in index 0, move 5 is in index 4, etc. Moves that have not yet been made are indicated by
/// the end of the Vec
#[derive(Serialize, Deserialize)]
pub struct Clock {
    pub times: Vec<DateTime<Utc>>,
    pub time_format: TimeFormat,
    /// The amount of time in nanoseconds left on a player's clock
    /// None if a game with unlimited time is being played
    pub nanos_on_clock: Option<u64>,
}

impl Clocks {
    pub fn get_clock(&self, player: ColorKind) -> Option<&Clock> {
        self.data.get(player.0 as usize)
    }
}

impl Kind {
    pub fn supports_variant(&self, variant: &Variant) -> bool {
        match *self {
            Kind::Chess => match *variant {
                Variant::Chess960 => true,
                Variant::NoCastling => true,
            },
        }
    }
}
