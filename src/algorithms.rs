
use crate::algorithm;
use crate::chess_like;

use std::error::Error;

#[derive(Debug, new)]
pub struct RandomAlgorithm {

}

impl<BoardType: chess_like::GenericBoard> algorithm::Algorithm <BoardType> for RandomAlgorithm {
    fn next_move(
        &self,
        input: algorithm::AlgorithmInput<BoardType>,
    ) -> Result<chess_like::Move<BoardType>, Box<dyn Error + Sync + Send>> {
        Err(String::from("I resign because there are no good moves").into())
    }

}

#[derive(Debug, new)]
pub struct DumbAlgorithm {}

impl<BoardType: crate::chess_like::GenericBoard> algorithm::Algorithm<BoardType> for DumbAlgorithm {
    fn next_move(
        &self,
        input: algorithm::AlgorithmInput<BoardType>,
    ) -> Result<crate::chess_like::Move<BoardType>, Box<dyn Error + Sync + Send>> {
        Err(String::from("I resign because im dumb").into())
    }
}

