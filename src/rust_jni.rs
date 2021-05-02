// This is the interface to the JVM that we'll call the majority of our
// methods on.

use crate::algorithm;
use crate::algorithm::Game;
use crate::algorithm::TimeFormat;
use crate::algorithms;
use crate::chess;
use crate::contrasting_chess;

use chrono::Duration;

use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jint};
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_com_troy_chess_Natives_init_1rust(_env: JNIEnv, _class: JClass) {}

/// Called from java to start a chess game of the given type with 2 algorithms given
/// game_type must either be 0 for regular chess or 1 for contrasting chess
#[no_mangle]
pub extern "system" fn Java_com_troy_chess_Natives_start_1game(
    bad_env: JNIEnv,
    bad_natives_class: JClass,
    ja_name: JString,
    jb_name: JString,
    game_type: jint,
    game_id: jint,
) -> jboolean {

    // SAFETY:
    //  See HumanJavaFXAlgorithm struct for more details
    let env: JNIEnv<'static> = unsafe { std::mem::transmute(bad_env) };
    let natives_class: JClass<'static> = unsafe { std::mem::transmute(bad_natives_class) };

    let a_name: String = env
        .get_string(ja_name)
        .expect("Failed to get the a algorithm")
        .into();
    let b_name: String = env
        .get_string(jb_name)
        .expect("Failed to get the b algorithm")
        .into();

    println!(
        "Got game info: {} vs, {}, Type: {}, id: {}",
        a_name, b_name, game_type, game_id
    );

    match game_type {
        0 => {
            let a: Box<dyn algorithm::Algorithm<chess::ChessBoard>> = match a_name.as_str() {
                "human" => Box::new(algorithms::HumanJavaFXAlgorithm::new(env, natives_class)),
                "random_ai" => Box::new(algorithms::RandomAlgorithm::new()),
                _ => return false.into(),
            };
            let b: Box<dyn algorithm::Algorithm<chess::ChessBoard>> = match b_name.as_str() {
                "human" => Box::new(algorithms::HumanJavaFXAlgorithm::new(env, natives_class)),
                "random_ai" => Box::new(algorithms::RandomAlgorithm::new()),
                _ => return false.into(),
            };

            //Not shared with the algos
            let mut game: Game<chess::ChessBoard, 2> = Game::new(
                vec![a, b],
                TimeFormat::Increment {
                    initial: Duration::minutes(5),
                    increment: Duration::seconds(0),
                },
            );

            while game.one_move() {
                println!("Got one move");
            }
            match game.state {
                algorithm::GameState::Finished(state) => match state {
                    algorithm::GameEndState::Draw(draw) => {
                        println!("Game {} ended in a draw: {}", game_id, draw);
                    }
                    algorithm::GameEndState::Decisive { winner, kind } => {
                        println!("Game {} ended with a win for {} - {}", game_id, winner, kind);
                    }
                    algorithm::GameEndState::Aborted => {
                        println!("Game {} was aborted", game_id);
                    }
                },
                _ => unreachable!(),
            }
        }
        1 => {
            println!("Contrasting chess not available yet...");
            return false.into();
        }
        _ => {
            return false.into();
        }
    }

    return true.into();
}

