use chrono::{offset, DateTime, Duration, Local};
use std::convert::TryInto;
use std::error::Error;
use std::fmt::Debug;

#[derive(Debug)]
struct PlayerData<BoardType: crate::chess_like::GenericBoard> {
    algorithm: Box<dyn Algorithm<BoardType>>,
    calculated_time: f64,
    is_move: bool,
    last_move_time: DateTime<offset::Local>,
}

///This is all an algorithm gets to see when we ask it to move
#[derive(new)]
struct AlgorithmInput<BoardType: crate::chess_like::GenericBoard> {
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

enum TimeFormat {
    Fixed(Duration),
    Increment {
        initial: Duration,
        increment: Duration,
    },
    Unlimited,
}

struct Game<BoardType: crate::chess_like::GenericBoard, const PLAYER_COUNT: usize> {
    ///The players participating in this game
    players: [PlayerData<BoardType>; PLAYER_COUNT],

    ///The board
    board: BoardType,

    ///The time format for this game
    time_format: TimeFormat,

    ///An index into players pointing to the player who is to move
    move_index: usize,

    ///The number of completed turns (A turn is defined as the number of times each player makes a
    ///move). Starts at 0
    turn_count: usize,
}

trait Algorithm<BoardType: crate::chess_like::GenericBoard>: Debug {
    fn next_move(
        &self,
        input: AlgorithmInput<BoardType>,
    ) -> Result<crate::chess_like::Move<BoardType>, Box<dyn Error + Sync + Send>>;
}

impl<BoardType: crate::chess_like::GenericBoard, const PlayerCount: usize>
    Game<BoardType, PlayerCount>
{
    pub fn one_move(&mut self) -> bool {
        let player = &self.players[self.move_index];
        let result = player.algorithm.next_move(self.state_for_next_move(player));
        self.apply(result);
        false
    }

    pub fn new(
        players: Vec<Box<dyn Algorithm<BoardType>>>,
        time_format: TimeFormat,
    ) -> Game<BoardType, PlayerCount> {
        let mut temp_players: Vec<PlayerData<BoardType>> = Vec::new();
        Game {
            players: temp_players.try_into().unwrap(),
            board: BoardType::default(),
            time_format,
        }
    }

    fn state_for_next_move(&self, player_data: &PlayerData<BoardType>) -> AlgorithmInput<BoardType> {
        let now = Local::now();
        let time_to_flag = 
        AlgorithmInput::new(self.board.clone(), now, 
    }
}

#[derive(Debug, new)]
struct DumbAlgorithm {}

impl<BoardType: crate::chess_like::GenericBoard> Algorithm<BoardType> for DumbAlgorithm {
    fn next_move(
        &self,
        state: PlayerData<BoardType>,
    ) -> Result<crate::chess_like::Move<BoardType>, Box<dyn Error + Sync + Send>> {
        Err(String::from("I resign").into())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    fn test() {
        let white = Box::new(DumbAlgorithm::new());
        let black = Box::new(DumbAlgorithm::new());
        //Not shared with the algos
        let game: Game<crate::chess::ChessBoard, 2> =
            Game::new(vec![white, black], TimeFormat::Fixed(Duration::seconds(5)));

        while game.one_move() {}
    }
}
