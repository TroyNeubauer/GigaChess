use chrono::{offset, DateTime, Duration, Local};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;

use crate::algorithms;
use crate::chess;
use crate::chess_like;

#[derive(Debug, new)]
pub struct PlayerData<BoardType: chess_like::GenericBoard> {
    algorithm: Box<dyn Algorithm<BoardType>>,

    ///The amount of time the player had on their clock when their move started. None if no time limit
    ///is in use
    clock: Option<Duration>,

    ///If Some, indicates that this player's clock is running. The inner value is the instant their move started.
    ///Used for calculating time used in the current move (the current time - last_move_time)
    ///If None, then this player is not using their time and it is not their move
    last_move_time: Option<DateTime<offset::Local>>,
}

///This is all an algorithm gets to see when we ask it to move
#[derive(new)]
pub struct AlgorithmInput<BoardType: chess_like::GenericBoard> {
    ///The current board
    board: BoardType,

    ///The instant this move started
    move_start: DateTime<offset::Local>,

    ///The instant in time this player will flag if no move is made
    ///None if the time format is unlimited
    flag_instant: Option<DateTime<offset::Local>>,

    ///The time format used in this game
    time_format: TimeFormat,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TimeFormat {
    Increment {
        initial: Duration,
        increment: Duration,
    },
    Unlimited,
}

pub struct Game<BoardType: chess_like::GenericBoard, const PLAYER_COUNT: usize> {
    ///The players participating in this game
    players: [PlayerData<BoardType>; PLAYER_COUNT],

    ///The board
    pub board: BoardType,

    ///The time format for this game
    time_format: TimeFormat,

    ///An index into players pointing to the player who is to move
    move_index: usize,

    ///The number of completed turns (A turn is defined as the number of times each player makes a
    ///move). Starts at 0
    turn_count: usize,

    pub state: GameState<BoardType>,

    ///The list of moves that lead to this position
    moves: Vec<chess_like::Move<BoardType>>,
}

pub trait Algorithm<BoardType: chess_like::GenericBoard>: fmt::Debug {
    fn next_move(
        &self,
        input: AlgorithmInput<BoardType>,
    ) -> Result<chess_like::Move<BoardType>, Box<dyn Error + Sync + Send>>;
}

#[derive(Debug)]
pub enum DrawGameType {
    Stalemate,
    DeadPosition,
    DrawOffer,
}

#[derive(Debug)]
pub enum DecisiveGameType<BoardType: chess_like::GenericBoard> {
    ///The looser resigned
    Resign,
    ///The loosing algorithm returned a fatal error
    Err(Box<dyn Error + Sync + Send>),
    ///The looser ran out of time
    Flag,
    ///The looser was checkmated
    Checkmate,
    ///Some other specific end (for another game type TODO)
    Other,
    ///The looser tried to make an illegal move
    IllegalMove(chess_like::Move<BoardType>),
}

#[derive(Debug)]
pub enum GameEndState<BoardType: chess_like::GenericBoard> {
    Draw(DrawGameType),
    Decisive {
        winner: BoardType::ColorType,
        kind: DecisiveGameType<BoardType>,
    },
    Aborted,
}

#[derive(Debug)]
pub enum GameState<BoardType: chess_like::GenericBoard> {
    NotStarted,
    Running,
    Finished(GameEndState<BoardType>),
}

impl<BoardType: chess_like::GenericBoard, const PLAYER_COUNT: usize> Game<BoardType, PLAYER_COUNT>
where
    BoardType::ColorType: chess_like::GenericColor,
{
    ///Executes a move for one player in the game synchronously, returning true if the game is not over.
    ///This operation blocks on input (AI computation or waiting for the human player to move a piece),
    ///then executes the move on the board, tallies the time spent by the player, and finally swaps the player to move
    ///(I.E the clock of the opposing player starts ticking down, or for gams with > 2 players, play moves
    ///to the next player in a round robin fashion).
    ///However since this function only handles one move, it will return at this point.
    ///Therefore, this function should be called repeatedly until it returns false, indicating that the game is over.
    ///If there is any delay between one invocation of this function and the next, that delay will be registered as time spent against the
    ///player to move, even though the AI did not get this time to think.
    pub fn one_move(&mut self) -> bool {
        if !self.is_started() {
            let game_start = Local::now();
            self.players[0].last_move_time = Some(game_start);
        }
        let player = &self.players[self.move_index];

        //Computation:
        let result = player.algorithm.next_move(self.state_for_next_move(player));
        match result {
            Ok(generated_move) => self.apply(generated_move),
            Err(err) => {
                if self.turn_count == 0 {
                    self.abort_ending();
                } else {
                    self.decisive_ending(
                        BoardType::ColorType::try_from(self.move_index).ok().unwrap(),
                        DecisiveGameType::Err(err),
                    )
                }
            }
        }

        false
    }

    pub fn new(
        algorithms: Vec<Box<dyn Algorithm<BoardType>>>,
        time_format: TimeFormat,
    ) -> Game<BoardType, PLAYER_COUNT> {
        let temp_players: Vec<PlayerData<BoardType>> = algorithms
            .into_iter()
            .map(|algorithm: Box<dyn Algorithm<BoardType>>| {
                let clock = match time_format {
                    TimeFormat::Increment { initial, increment } => {
                        let _ = increment; //why rustfmt? We can't change the name
                        Some(initial)
                    }
                    TimeFormat::Unlimited => None,
                };

                PlayerData::new(algorithm, clock, None)
            })
            .collect();

        Game {
            players: temp_players.try_into().unwrap(),
            board: BoardType::default(),
            time_format,
            move_index: 0,
            turn_count: 0,
            state: GameState::NotStarted,
            moves: Vec::new(),
        }
    }

    pub fn is_started(&self) -> bool {
        self.turn_count != 0
            || self.move_index != 0
            || self.players[self.move_index].last_move_time.is_some()
    }

    fn state_for_next_move(
        &self,
        player_data: &PlayerData<BoardType>,
    ) -> AlgorithmInput<BoardType> {
        let now = Local::now();
        let flag_instant = match self.time_format {
            TimeFormat::Unlimited => None,
            TimeFormat::Increment { initial, increment } => Some(
                now + player_data
                    .clock
                    .expect("Expected game with the increment time format to have clocks"),
            ),
        };
        AlgorithmInput::new(self.board.clone(), now, flag_instant, self.time_format)
    }

    fn apply(&mut self, to_apply: chess_like::Move<BoardType>) {
        if !self.board.is_move_legal(to_apply) {
            self.decisive_ending(
                BoardType::ColorType::try_from((self.move_index + 1) % PLAYER_COUNT).ok().unwrap(),
                DecisiveGameType::IllegalMove(to_apply),
            );
            return;
        }
        self.board.apply_raw_move(to_apply);
    }

    fn decisive_ending(&mut self, winner: BoardType::ColorType, kind: DecisiveGameType<BoardType>) {
        self.state = GameState::Finished(GameEndState::Decisive { winner, kind });
    }

    fn draw_ending(&mut self, ending: DrawGameType) {
        self.state = GameState::Finished(GameEndState::Draw(ending));
    }

    fn abort_ending(&mut self) {
        self.state = GameState::Finished(GameEndState::Aborted);
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test() {
        let white = Box::new(algorithms::DumbAlgorithm::new());
        let black = Box::new(algorithms::RandomAlgorithm::new());
        //Not shared with the algos
        let mut game: Game<chess::ChessBoard, 2> = Game::new(
            vec![white, black],
            TimeFormat::Increment {
                initial: Duration::minutes(5),
                increment: Duration::seconds(0),
            },
        );

        while game.one_move() {
            println!("Moved!");
        }

        let white = Box::new(algorithms::RandomAlgorithm::new());
        let black = Box::new(algorithms::DumbAlgorithm::new());
        //Not shared with the algos
        let mut game: Game<chess::ChessBoard, 2> = Game::new(
            vec![white, black],
            TimeFormat::Increment {
                initial: Duration::minutes(5),
                increment: Duration::seconds(0),
            },
        );

        while game.one_move() {
            println!("Moved!");
        }
    }
}

impl fmt::Display for DrawGameType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DrawGameType::Stalemate => write!(f, "Stalemate"),
            DrawGameType::DeadPosition => write!(f, "Dead Position"),
            DrawGameType::DrawOffer => write!(f, "Draw Offer"),
        }
    }
}

impl<BoardType: chess_like::GenericBoard> fmt::Display for DecisiveGameType<BoardType> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DecisiveGameType::Resign => write!(f, "Resign"),
            DecisiveGameType::Err(err) => write!(f, "Error {}", err),
            DecisiveGameType::Flag => write!(f, "Flag"),
            DecisiveGameType::Checkmate => write!(f, "Checkmate"),
            DecisiveGameType::Other => write!(f, "Unknown"),
            DecisiveGameType::IllegalMove(illegal_move) => write!(f, "Illegal move {}", illegal_move),
        }
    }
}

