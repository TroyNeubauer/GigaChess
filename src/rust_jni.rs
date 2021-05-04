// This is the interface to the JVM that we'll call the majority of our
// methods on.

use crate::{algorithm, algorithms, chess, chess_like, contrasting_chess};

use crate::chess_like::GenericBoard;

use chrono::Duration;
use std::convert::From;
use std::convert::TryInto;

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
            let mut game: algorithm::Game<chess::ChessBoard, 2> = algorithm::Game::new(
                vec![a, b],
                algorithm::TimeFormat::Increment {
                    initial: Duration::minutes(5),
                    increment: Duration::seconds(0),
                },
            );

            if false {
                game.board.clear();
                game.board.set(
                    chess::ChessBoard::to_storage(chess::ChessFile::E, chess::ChessRank::R3),
                    chess_like::RawSquare::new(
                        chess::ChessPiece::King,
                        chess_like::DefaultColorScheme::While,
                    ),
                );

                game.board.set(
                    chess::ChessBoard::to_storage(chess::ChessFile::D, chess::ChessRank::R5),
                    chess_like::RawSquare::new(
                        chess::ChessPiece::King,
                        chess_like::DefaultColorScheme::Black,
                    ),
                );
                game.board.set(
                    chess::ChessBoard::to_storage(chess::ChessFile::A, chess::ChessRank::R1),
                    chess_like::RawSquare::new(
                        chess::ChessPiece::Rook,
                        chess_like::DefaultColorScheme::While,
                    ),
                );
                game.board.set(
                    chess::ChessBoard::to_storage(chess::ChessFile::H, chess::ChessRank::R8),
                    chess_like::RawSquare::new(
                        chess::ChessPiece::Rook,
                        chess_like::DefaultColorScheme::Black,
                    ),
                );
            }

            let still_ok = env
                .call_static_method(
                    natives_class,
                    "set_board_size",
                    "(II)Z",
                    &[
                        jni::objects::JValue::Int(game_id),
                        jni::objects::JValue::Int(chess::ChessBoard::side_len().into()),
                    ],
                )
                .expect("Failed to call set_board_size")
                .z()
                .ok()
                .unwrap();
            if !still_ok {
                return false.into();
            }

            for square in game.board.raw_square_iter() {
                let piece = game.board.get(square);

                //public static boolean set_square(int gameID, int square, int pieceKind, int color) {
                let still_ok = env
                    .call_static_method(
                        natives_class,
                        "set_square",
                        "(IIII)Z",
                        &[
                            jni::objects::JValue::Int(game_id),
                            jni::objects::JValue::Int(square as i32),
                            jni::objects::JValue::Int(to_java_piece_type_chess(piece)),
                            jni::objects::JValue::Int(to_java_color::<chess::ChessBoard>(piece)),
                        ],
                    )
                    .expect("Failed to call set_square")
                    .z()
                    .ok()
                    .unwrap();
                if !still_ok {
                    return false.into();
                }
            }

            loop {
                match game.one_move() {
                    Some(m) => {
                        let src: isize = m.src.into();
                        let dest: isize = m.dest.into();

                        let still_ok = env
                            .call_static_method(
                                natives_class,
                                "display_move",
                                "(III)Z",
                                &[
                                    jni::objects::JValue::Int(game_id),
                                    jni::objects::JValue::Int(src.try_into().unwrap()),
                                    jni::objects::JValue::Int(dest.try_into().unwrap()),
                                ],
                            )
                            .expect("Failed to call set_square")
                            .z()
                            .ok()
                            .unwrap();
                        if !still_ok {
                            return false.into();
                        }
                    }
                    None => break,
                }
            }

            match game.state {
                algorithm::GameState::Finished(state) => match state {
                    algorithm::GameEndState::Draw(draw) => {
                        println!("Game {} ended in a draw: {}", game_id, draw);
                    }
                    algorithm::GameEndState::Decisive { winner, kind } => {
                        println!(
                            "Game {} ended with a win for {} - {}",
                            game_id, winner, kind
                        );
                    }
                    algorithm::GameEndState::Aborted => {
                        println!("Game {} was aborted", game_id);
                    }
                },
                _ => unreachable!(),
            }
        }
        1 => {
            let a: Box<dyn algorithm::Algorithm<contrasting_chess::ContrastingChessBoard>> = match a_name.as_str() {
                "human" => Box::new(algorithms::HumanJavaFXAlgorithm::new(env, natives_class)),
                "random_ai" => Box::new(algorithms::RandomAlgorithm::new()),
                _ => return false.into(),
            };
            let b: Box<dyn algorithm::Algorithm<contrasting_chess::ContrastingChessBoard>> = match b_name.as_str() {
                "human" => Box::new(algorithms::HumanJavaFXAlgorithm::new(env, natives_class)),
                "random_ai" => Box::new(algorithms::RandomAlgorithm::new()),
                _ => return false.into(),
            };

            //Not shared with the algos
            let mut game: algorithm::Game<contrasting_chess::ContrastingChessBoard, 2> = algorithm::Game::new(
                vec![a, b],
                algorithm::TimeFormat::Increment {
                    initial: Duration::minutes(5),
                    increment: Duration::seconds(0),
                },
            );

            let still_ok = env
                .call_static_method(
                    natives_class,
                    "set_board_size",
                    "(II)Z",
                    &[
                        jni::objects::JValue::Int(game_id),
                        jni::objects::JValue::Int(contrasting_chess::ContrastingChessBoard::side_len().into()),
                    ],
                )
                .expect("Failed to call set_board_size")
                .z()
                .ok()
                .unwrap();
            if !still_ok {
                return false.into();
            }

            for square in game.board.raw_square_iter() {
                let piece = game.board.get(square);

                //public static boolean set_square(int gameID, int square, int pieceKind, int color) {
                let still_ok = env
                    .call_static_method(
                        natives_class,
                        "set_square",
                        "(IIII)Z",
                        &[
                            jni::objects::JValue::Int(game_id),
                            jni::objects::JValue::Int(square as i32),
                            jni::objects::JValue::Int(to_java_piece_type_contrasting_chess(piece)),
                            jni::objects::JValue::Int(to_java_color::<contrasting_chess::ContrastingChessBoard>(piece)),
                        ],
                    )
                    .expect("Failed to call set_square")
                    .z()
                    .ok()
                    .unwrap();
                if !still_ok {
                    return false.into();
                }
            }

            loop {
                match game.one_move() {
                    Some(m) => {
                        let src: isize = m.src.into();
                        let dest: isize = m.dest.into();

                        let still_ok = env
                            .call_static_method(
                                natives_class,
                                "display_move",
                                "(III)Z",
                                &[
                                    jni::objects::JValue::Int(game_id),
                                    jni::objects::JValue::Int(src.try_into().unwrap()),
                                    jni::objects::JValue::Int(dest.try_into().unwrap()),
                                ],
                            )
                            .expect("Failed to call set_square")
                            .z()
                            .ok()
                            .unwrap();
                        if !still_ok {
                            return false.into();
                        }
                    }
                    None => break,
                }
            }

            match game.state {
                algorithm::GameState::Finished(state) => match state {
                    algorithm::GameEndState::Draw(draw) => {
                        println!("Game {} ended in a draw: {}", game_id, draw);
                    }
                    algorithm::GameEndState::Decisive { winner, kind } => {
                        println!(
                            "Game {} ended with a win for {} - {}",
                            game_id, winner, kind
                        );
                    }
                    algorithm::GameEndState::Aborted => {
                        println!("Game {} was aborted", game_id);
                    }
                },
                _ => unreachable!(),
            }

            println!("Contrasting chess not available yet...");
            return false.into();
        }
        _ => {
            return false.into();
        }
    }

    return true.into();
}

// private static final String[] IMAGE_NAMES = new String[] { "", "king", "queen", "rook",
// "bishop", "night", "pawn", "donkey", "elephant", "moose" };

fn to_java_piece_type_chess(
    piece: &chess_like::RawSquare<chess::ChessPiece, chess_like::DefaultColorScheme>,
) -> jint {
    match piece.0 {
        Some(raw_square) => match raw_square.piece {
            chess::ChessPiece::King => 1,
            chess::ChessPiece::Queen => 2,
            chess::ChessPiece::Rook => 3,
            chess::ChessPiece::Bishop => 4,
            chess::ChessPiece::Knight => 5,
            chess::ChessPiece::Pawn => 6,
        },
        //Empty square
        None => 0,
    }
}


fn to_java_piece_type_contrasting_chess(
    piece: &chess_like::RawSquare<contrasting_chess::ContrastingChessPiece, chess_like::DefaultColorScheme>,
) -> jint {
    match piece.0 {
        Some(raw_square) => match raw_square.piece {
King,
    Elephant,
    Bear,
    Horse,
    Dragon,
    Moose,
    Rodent,

            chess::ChessPiece::King => 1,
            chess::ChessPiece::Queen => 2,
            chess::ChessPiece::Rook => 3,
            chess::ChessPiece::Bishop => 4,
            chess::ChessPiece::Knight => 5,
            chess::ChessPiece::Pawn => 6,
        },
        //Empty square
        None => 0,
    }
}



fn to_java_color<BoardType: chess_like::GenericBoard>(
    piece: &chess_like::RawSquare<BoardType::PieceType, BoardType::ColorType>,
) -> jint
where
    <BoardType as chess_like::GenericBoard>::ColorType: chess_like::GenericColor,
{
    match piece.0 {
        Some(piece) => {
            let color: usize = piece.color.try_into().unwrap();
            color.try_into().unwrap()
        }
        None => 0,
    }
}
