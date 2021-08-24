use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use chrono::prelude::*;
use smallvec::SmallVec;

use crate::game;

/// The kinds of messages that are sent by the moderator to the engine
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum In {
    /// The engine has been loaded successfully by the moderator. This is always the first message
    /// that will be received.
    /// When this message is received by an engine, it must send a EngineInfo message back to the
    /// moderator to inform the moderator about itself
    EngineInit,

    /// Indicates that a game is beginning
    GameStart {
        variant: game::Kind,
        /// Contains the game defined piece chars. Always a square
        board: String,

        /// The path to a unix socket that the moderator listens on for traffic related to this game.
        /// Once the engine establishes a connection to this path, this socket is called the
        /// game socket, and all future communication about this game will happen there, using json
        /// serialized `GameIn` and `GameOut` messages
        game_listen_path: String,

        /// A unique identifier for the game. Never re-used within the same execution of the
        /// engine
        game_id: game::ID,

        /// The side this engine is playing as
        playing_as: game::ColorKind,

        /// The time format that our clock uses
        time_format: game::TimeFormat,

        /// The opponents playing against this engine, and what time format they are using
        /// Usually all time formats will be the same however some odds games have different time
        /// formats
        opponents: HashMap<game::ColorKind, (EngineInfo, game::TimeFormat)>,
    },

    GameEnd {
        /// The game id which is ending. More information will be sent on the game socket about why
        /// the game ended (checkmate, stalemate, illegal move, etc.)
        game_id: game::ID,
    },

    /// This is the last message that will be sent to the engine. This will be sent on a best
    /// effort basis. The moderator makes no guarantees that this will be sent. The engine
    /// process may be killed without warning.
    EngineShutdown,

    /// An invalid request was received by the moderator and ignored.
    /// An invalid request is a request that cannot be fulfilled for some reason, usually because
    /// it fail to parse.
    /// Note that Move requests containing illegal moves constitute legal, fulfillable requests,
    /// even though they are logically invalid.
    /// Illegal moves lead to the end of the same in a valid manner and not an
    /// InvalidRequest message being sent.
    InvalidRequest {
        /// A human readable message describing why the request is invalid
        message: String,

        /// The json of the invalid request
        request_json: String,

        /// The game in which the request originated (if any)
        related_game: Option<game::ID>,
    },
}

/// Contains information about an engine
#[derive(Serialize, Deserialize)]
pub struct EngineInfo {
    name: String,
    version: String,
    description: String,
    /// Author and email in the format Name <Email>. Ie: "Troy Neubauer <troyneubauer@gmail.com>"
    author: String,
    /// Link to repository containing the code for this engine
    repo: String,
}

/// The kinds of messages that are sent from the engine to the moderator
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Out {
    EngineInfo {
        /// Information about the engine
        info: EngineInfo,
        /// The list of supported games, mapped to which variants are supported for each game.
        /// A mapping between a game type and an empty variant list indicates that the stock
        /// version of this game is supported, but no variants are supported for that game
        supported_games: HashMap<game::Kind, SmallVec<[game::Variant; 2]>>,
    },
}

/// Messages from the moderator to the engine about a particular game
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GameIn {
    /// The opponent moved a piece from src to dest
    OpponentMove {
        /// The opponent's move. We cannot call this opponent_move because move is a rust keyword
        #[serde(rename = "move")]
        opponent_move: game::RawMove,

        /// The color of the opponent making this move
        opponent: game::ColorKind,
    },

    /// It is now this engine's move. This engine's time will begin ticking down, and flagging is
    /// possible.
    /// For a 2 player game the engine will always receive an OpponentMove message and then
    /// immediately a YourMove message. This may seem redundant, however this is critical in
    /// multiplayer games for the engine to know when its move is.
    ///
    /// During the slight window between the moderator sending an OpponentMove message and a
    /// YourMove message, no clocks advance. This is usually negligible because the actions
    /// performed during this step take < 1 microsecond. It is still important that the moderator
    /// functions in this way to keep fast games fair.
    YourMove {
        /// The instant this engine will flag
        flag_instant: DateTime<Utc>,
    },
    /// An opponent offers a draw. This engine can either ignore the offer, reject it by sending
    /// RejectDrawOffer, or accept the draw by sending DrawOffer.
    OpponentDrawOffer {
        /// The color of the player offering a draw.
        player: game::ColorKind,
    },
    GameOver {
        winner: Option<game::ColorKind>,
        cause: game::GameEndCause,
    },
    /// A response to a GetClocks request.
    /// Holds the most up to date information on the clocks for all players
    Clocks(game::Clocks),
}

/// Messages sent from this engine to the moderator. These include all the actions a player can
/// perform in a physical game of chess (moving a piece, resigning, sending a draw offer, etc.)
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GameOut {
    /// This engine resigns
    Resign,

    /// This engine wants to send a draw offer to the other player. If all players send draw
    /// offers, the game ends in a draw. If no other players have send a draw offer on this move,
    /// then this initiates a draw offer to all players
    DrawOffer,

    /// This engine rejects the pending draw offer from another player.
    /// If there is no pending draw offer, sending this message is a nop
    RejectDrawOffer,

    /// Asks the moderator to send a Clocks message to this engine telling it the move times for
    /// each player and how much time is left
    GetClocks,

    /// This engine wishes to move a piece from src to dst.
    /// If the move is valid: then is processed by the moderator and the opponent
    ///   receives the valid move.
    /// If move is non valid, (contains invalid squares, or is illegal):
    ///   The game is ended, this engine looses, and a game over message is sent to all players
    Move(game::RawMove),

    /// This engine encountered an error and cannot continue.
    /// This is effectively the same as resigning.
    /// This message should be sent when the engine encounters an invalid state including:
    ///   The moderator telling this engine that the opponent successfully made an invalid move
    ///   Memory allocation or I/O failures that prevent the engine from proceeding
    ///   Any other logical invariant preventing the engine from ever making a move
    Err { message: String },
}
