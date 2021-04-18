// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;

use jni::objects::{JClass, JString};

#[no_mangle]
pub extern "system" fn Java_com_troy_chess_Natives_init_1rust(_env: JNIEnv, _class: JClass) {}


#[no_mangle]
pub extern "system" fn Java_com_troy_chess_Natives_start_game(env: JNIEnv, _class: JClass, ja_name: JString, jb_name: JString, game_type_java: JString) {
    let a_name: String = env.get_string(ja_name).expect("Failed to get player a's name!").into();
    let b_name: String = env.get_string(jb_name).expect("Failed to get player b's name!").into();
    let game_type: String = env.get_string(game_type_java).expect("Failed to get the game type").into();

    println!("Got game info: {} vs, {}. Type: {}", a_name, b_name, game_type);

}

