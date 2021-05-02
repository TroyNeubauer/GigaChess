use crate::algorithm;
use crate::chess_like;

use std::convert::TryFrom;
use std::error;

use jni::JNIEnv;

use jni::objects::JClass;

#[derive(Debug, new)]
pub struct RandomAlgorithm {}

impl<BoardType: chess_like::GenericBoard> algorithm::Algorithm<BoardType> for RandomAlgorithm {
    fn next_move(
        &self,
        input: algorithm::AlgorithmInput<BoardType>,
    ) -> Result<chess_like::Move<BoardType>, Box<dyn error::Error + Sync + Send>> {
        Err(String::from("I resign because there are no good moves").into())
    }
}

#[derive(Debug, new)]
pub struct DumbAlgorithm {}

impl<BoardType: crate::chess_like::GenericBoard> algorithm::Algorithm<BoardType> for DumbAlgorithm {
    fn next_move(
        &self,
        input: algorithm::AlgorithmInput<BoardType>,
    ) -> Result<crate::chess_like::Move<BoardType>, Box<dyn error::Error + Sync + Send>> {
        Err(String::from("I resign because im dumb").into())
    }
}

#[derive(new)]
pub struct HumanJavaFXAlgorithm {
    // SAFETY:
    //  A JNIEnv pointer is valid within a thread. Since JNIEnv is not Send this is ok
    //  The argument for JClass is the same.
    //  We don't store any algorithm for longer than a call into start_game anyway
    //  Also this avoids having to add lifetime parameters to every algorithm
    env: JNIEnv<'static>,
    natives_class: JClass<'static>,
}

impl<BoardType: crate::chess_like::GenericBoard> algorithm::Algorithm<BoardType>
    for HumanJavaFXAlgorithm
{
    fn next_move(
        &self,
        input: algorithm::AlgorithmInput<BoardType>,
    ) -> Result<crate::chess_like::Move<BoardType>, Box<dyn error::Error + Sync + Send>> {
        loop {
            //public static long get_human_move(int side);
            let j_long = self
                .env
                .call_static_method(
                    self.natives_class,
                    "get_human_move",
                    "(I)J",
                    &[jni::objects::JValue::Int(0)],
                )
                .expect("Failed to call get_human_move")
                .j()
                .ok()
                .unwrap();

            let src_square_raw = ((j_long >> 32) & 0xFFFFFFFFi64) as usize;
            let dest_square_raw = (j_long & 0xFFFFFFFFi64) as usize;
            let src_square: BoardType::StorageType =
                BoardType::StorageType::try_from(src_square_raw)
                    .ok()
                    .unwrap();
            let dest_square: BoardType::StorageType =
                BoardType::StorageType::try_from(dest_square_raw)
                    .ok()
                    .unwrap();

            let m: chess_like::Move<BoardType> = chess_like::Move {
                src: src_square,
                dest: dest_square,
            };
            if input.board.is_move_legal(input.color, m) {
                println!("Got good move {}", m);
                return Ok(m);
            }
            println!("Move {} is bad", m);
        }
    }
}

impl std::fmt::Debug for HumanJavaFXAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("")
    }
}
