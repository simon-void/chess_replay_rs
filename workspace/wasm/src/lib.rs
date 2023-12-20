extern crate core;

use std::convert::{Into};

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;

use logic_core::*;
use logic_core::base::Move;
use logic_core::game::{GameState, MoveStats};

pub use crate::figure::functions::allowed::get_allowed_moves;
pub use crate::game::Game;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();


    // Your code goes here!
    console::log_1(&JsValue::from_str("wasm init"));

    Ok(())
}


#[wasm_bindgen]
pub fn get_greeting_for(name: &str) -> JsValue {
    let greeting = format!("Hello, {}", name);
    JsValue::from_str(greeting.as_str())
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonResult {
    is_ok: bool,
    value: String,
}

#[wasm_bindgen]
pub fn decode_moves(base64_encoded: &str) -> JsValue {
    let moves_result = match decode_moves_base64(base64_encoded) {
        Ok(moves) => {
            JsonResult {
                is_ok: true,
                value: moves,
            }
        }
        Err(err) => {
            JsonResult {
                is_ok: false,
                value: err,
            }
        }
    };
    let json = serde_json::to_string(&moves_result).unwrap();
    JsValue::from_str(json.as_str())
}

fn decode_moves_base64(base64_encoded: &str) -> Result<Vec<MoveStats>, String> {
    let mut encoded_chars= base64_encoded.chars();
    let mut move_stats: Vec<MoveStats> = Vec::with_capacity(base64_encoded.len() * 2 + 4);
    let mut game_state = GameState::classic();

    loop {
        let next_move_str = {
            let from_pos_enc: char = match encoded_chars.next() {
                None => { break; }
                Some(pos) => { pos }
            };
            let to_pos_enc: char = match encoded_chars.next() {
                None => { return Err(format!("encoded from-position without to-position")); }
                Some(pos) => { pos }
            };
            let mut move_to_fill: String = String::with_capacity(5);
            decode_base64_char(from_pos_enc, &mut move_to_fill)?;
            move_to_fill.push('-');
            decode_base64_char(to_pos_enc, &mut move_to_fill)?;
            move_to_fill
        };

        let mut next_move = next_move_str.parse::<Move>().unwrap()?;
        if game_state.looks_like_pawn_promotion_move(next_move) {
            if let Some(promotion_char) = encoded_chars.next() {
                next_move_str[2] = promotion_char;
                next_move = next_move_str.parse::<Move>().unwrap()?;
            }
        }
        let new_game_and_stats = game_state.do_move(next_move);
        game_state = new_game_and_stats.0;
        move_stats.push(new_game_and_stats.1);
    }

    Ok(move_stats)
}

fn decode_base64_char(character: char, move_to_fill: &mut String) -> Result<(), String> {
    // 0 A            17 R            34 i            51 z
    // 1 B            18 S            35 j            52 0
    // 2 C            19 T            36 k            53 1
    // 3 D            20 U            37 l            54 2
    // 4 E            21 V            38 m            55 3
    // 5 F            22 W            39 n            56 4
    // 6 G            23 X            40 o            57 5
    // 7 H            24 Y            41 p            58 6
    // 8 I            25 Z            42 q            59 7
    // 9 J            26 a            43 r            60 8
    //10 K            27 b            44 s            61 9
    //11 L            28 c            45 t            62 - (minus)
    //12 M            29 d            46 u            63 _ (underline)
    //13 N            30 e            47 v
    //14 O            31 f            48 w
    //15 P            32 g            49 x
    //16 Q            33 h            50 y         (pad) =

    let decoded: u8 = match character {
        'A' => { 0 }
        'B' => { 1 }
        'C' => { 2 }
        'D' => { 3 }
        'E' => { 4 }
        'F' => { 5 }
        'G' => { 6 }
        'H' => { 7 }
        'I' => { 8 }
        'J' => { 9 }
        'K' => { 10 }
        'L' => { 11 }
        'M' => { 12 }
        'N' => { 13 }
        'O' => { 14 }
        'P' => { 15 }
        'Q' => { 16 }
        'R' => { 17 }
        'S' => { 18 }
        'T' => { 19 }
        'U' => { 20 }
        'V' => { 21 }
        'W' => { 22 }
        'X' => { 23 }
        'Y' => { 24 }
        'Z' => { 25 }
        'a' => { 26 }
        'b' => { 27 }
        'c' => { 28 }
        'd' => { 29 }
        'e' => { 30 }
        'f' => { 31 }
        'g' => { 32 }
        'h' => { 33 }
        'i' => { 34 }
        'j' => { 35 }
        'k' => { 36 }
        'l' => { 37 }
        'm' => { 38 }
        'n' => { 39 }
        'o' => { 40 }
        'p' => { 41 }
        'q' => { 42 }
        'r' => { 43 }
        's' => { 44 }
        't' => { 45 }
        'u' => { 46 }
        'v' => { 47 }
        'w' => { 48 }
        'x' => { 49 }
        'y' => { 50 }
        'z' => { 51 }
        '0' => { 52 }
        '1' => { 53 }
        '2' => { 54 }
        '3' => { 55 }
        '4' => { 56 }
        '5' => { 57 }
        '6' => { 58 }
        '7' => { 59 }
        '8' => { 60 }
        '9' => { 61 }
        '-' => { 62 }
        '_' => { 63 }
        _ => { return Err(format!("invalid char: {}", character)); }
    };
    let column_index = decoded % 8;
    let row_index = decoded / 8;
    let column = (column_index + 97) as char;
    let row = (row_index + 49) as char;
    move_to_fill.push(column);
    move_to_fill.push(row);
    Ok(())
}

#[wasm_bindgen]
pub fn get_fen(game_config: &str) -> JsValue {
    let fen_result = match game_config.parse::<Game>() {
        Ok(game) => {
            let fen = game.get_fen();
            JsonResult {
                is_ok: true,
                value: fen,
            }
        }
        Err(err) => {
            let error_msg = format!("{:?}: {}", err.kind, err.msg);
            JsonResult {
                is_ok: false,
                value: error_msg,
            }
        }
    };
    let json = serde_json::to_string(&fen_result).unwrap();
    JsValue::from_str(json.as_str())
}

// // fn get_result_json(is_ok: bool, value: String) -> String {
// //     format!("{}{}{}{}{}", "{\"isOk\":", is_ok, ", \"value\":\"", value, "\"}")
// // }


// #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// pub enum SerializableDrawReason {
//     StaleMate,
//     InsufficientMaterial,
//     ThreeTimesRepetition,
//     NoChangeIn50Moves,
// }
//
// impl From<DrawReason> for SerializableDrawReason {
//     fn from(item: DrawReason) -> Self {
//         match item {
//             DrawReason::StaleMate => SerializableDrawReason::StaleMate,
//             DrawReason::InsufficientMaterial => SerializableDrawReason::InsufficientMaterial,
//             DrawReason::ThreeTimesRepetition => SerializableDrawReason::ThreeTimesRepetition,
//             DrawReason::NoChangeIn50Moves => SerializableDrawReason::NoChangeIn50Moves,
//         }
//     }
// }
//
// impl From<SerializableDrawReason> for DrawReason {
//     fn from(reason: SerializableDrawReason) -> Self {
//         match reason {
//             SerializableDrawReason::StaleMate => DrawReason::StaleMate,
//             SerializableDrawReason::InsufficientMaterial => DrawReason::InsufficientMaterial,
//             SerializableDrawReason::ThreeTimesRepetition => DrawReason::ThreeTimesRepetition,
//             SerializableDrawReason::NoChangeIn50Moves => DrawReason::NoChangeIn50Moves,
//         }
//     }
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub enum SerializableMoveEvaluation {
//     EngineCheckMatesIn(u8),
//     Numeric(f32),
//     Draw(SerializableDrawReason),
//     EngineGetsCheckMatedIn(u8, f32),
// }
//
// impl From<MoveEvaluation> for SerializableMoveEvaluation {
//     fn from(item: MoveEvaluation) -> Self {
//         match item {
//             MoveEvaluation::EngineCheckMatesIn(count) => SerializableMoveEvaluation::EngineCheckMatesIn(count),
//             MoveEvaluation::Numeric(value) => SerializableMoveEvaluation::Numeric(value),
//             MoveEvaluation::Draw(reason) => SerializableMoveEvaluation::Draw(reason.into()),
//             MoveEvaluation::EngineGetsCheckMatedIn(count, value) => SerializableMoveEvaluation::EngineGetsCheckMatedIn(count, value),
//         }
//     }
// }
//
// impl From<SerializableMoveEvaluation> for MoveEvaluation {
//     fn from(evaluation: SerializableMoveEvaluation) -> Self {
//         match evaluation {
//             SerializableMoveEvaluation::EngineCheckMatesIn(count) => MoveEvaluation::EngineCheckMatesIn(count),
//             SerializableMoveEvaluation::Numeric(value) => MoveEvaluation::Numeric(value),
//             SerializableMoveEvaluation::Draw(reason) => MoveEvaluation::Draw(reason.into()),
//             SerializableMoveEvaluation::EngineGetsCheckMatedIn(count, value) => MoveEvaluation::EngineGetsCheckMatedIn(count, value),
//         }
//     }
// }
//
//
//
//------------------------------Tests------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest(
    encoded, expected_decoded,
    case("IY-tYgxhgp2upx92", "a2-a4,g8-f6,a4-a5,b7-b5,a5-b6,g7-g6,b6-b7,f8-g7"),
    case("IY-tYgxhgp2upx92x4Q8_", "a2-a4,g8-f6,a4-a5,b7-b5,a5-b6,g7-g6,b6-b7,f8-g7,b7Qa8,e8-h8"),
    ::trace //This leads to the arguments being printed in front of the test result.
    )]
    fn test_decode_moves_base64(encoded: &str, expected_decoded: &str) {
        let actual_decoded = match decode_moves_base64(encoded) {
            Ok(moves) => {moves}
            Err(err) => {panic!("{}", err)}
        };
        assert_eq!(
            actual_decoded,
            expected_decoded,
        );
    }

//     #[test]
//     #[allow(non_snake_case)]
//     fn test_serialize_deserialize_GameEvaluationResultMoveToPlay() {
//         let move_to_play_result = GameEvaluationResultMoveToPlay {
//             result_type: "MoveToPlay".to_string(),
//             move_to_play: "b7-b6".to_string(),
//             eval: "{\"Numeric\":-1.0050015}".to_string(),
//             fen: "rnbqkbnr/p1pppppp/1p6/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2".to_string()
//         };
//         let serialized = serde_json::to_string(&move_to_play_result).unwrap();
//         let deserialized: GameEvaluationResultMoveToPlay = serde_json::from_str(serialized.as_str()).unwrap();
//         assert_eq!(
//             deserialized,
//             move_to_play_result,
//         );
//     }
//
//     #[test]
//     #[allow(non_snake_case)]
//     fn test_deserialize_GameEvaluationResultMoveToPlay() {
//         let chosen_move = "a2-a4".parse::<Move>().unwrap();
//         let eval = MoveEvaluation::Numeric(5.5);
//         let fen = "rnbqkbnr/p1pppppp/1p6/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2".to_string();
//         let json = move_to_play_to_json(chosen_move, eval, fen);
//         let deserialized: GameEvaluationResultMoveToPlay = serde_json::from_str(json.as_str()).unwrap();
//         assert_eq!(
//             deserialized.move_to_play,
//             "a2-a4",
//         );
//         assert_eq!(
//             deserialized.fen,
//             "rnbqkbnr/p1pppppp/1p6/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2".to_string(),
//         );
//         let deserialized_move_eval: MoveEvaluation = serde_json::from_str::<SerializableMoveEvaluation>(deserialized.eval.as_str()).unwrap().into();
//         assert_eq!(
//             deserialized_move_eval,
//             MoveEvaluation::Numeric(5.5),
//         );
//     }
}
