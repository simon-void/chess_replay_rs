extern crate core;

use std::convert::{Into};
use chess_compress_urlsafe::compression::decompress::decompress;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;

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

#[derive(Serialize, Deserialize, Debug)]
struct Game {
    vec_of_fen: Vec<String>,
    vec_of_moves: Vec<String>,
}

#[wasm_bindgen]
pub fn decode_moves(base64_encoded: &str) -> JsValue {
    // pub fn decompress(base64_encoded_match: &str) -> Result<(Vec<PositionData>, Vec<MoveData>), ChessError>
    let moves_result = match decompress(base64_encoded) {
        Ok(moves) => {
            // let move_stats_json = serde_json::to_string(&moves).unwrap();
            JsonResult {
                is_ok: true,
                value: move_stats_json,
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

//------------------------------Tests------------------------

#[cfg(test)]
mod tests {
    // use super::*;
    // use rstest::*;
    // use chess::game::concat_main_moves;
    //
    // #[rstest(
    //     encoded, expected_decoded,
    //     case("Y -t g xh p u x 2 4Q 8_", "a2a4,g8f6,a4a5,b7b5,a5b6,g7g6,b6b7,f8g7,b7a8Q,e8h8"), // IY -t Yg xh gp 2u px 92 x4Q 8_
    //     ::trace //This leads to the arguments being printed in front of the test result.
    // )]
    // fn test_decode_moves_base64(encoded: &str, expected_decoded: &str) {
    //     let actual_move_stats = match decode_moves_base64(encoded) {
    //         Ok(moves) => { moves }
    //         Err(err) => { panic!("{err}") }
    //     };
    //     assert_eq!(
    //         concat_main_moves(actual_move_stats),
    //         expected_decoded,
    //     );
    // }
    //
    // #[test]
    // #[allow(non_snake_case)]
    // fn test_serialize_deserialize_GameEvaluationResultMoveToPlay() {
    //     let move_to_play_result = GameEvaluationResultMoveToPlay {
    //         result_type: "MoveToPlay".to_string(),
    //         move_to_play: "b7-b6".to_string(),
    //         eval: "{\"Numeric\":-1.0050015}".to_string(),
    //         fen: "rnbqkbnr/p1pppppp/1p6/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2".to_string()
    //     };
    //     let serialized = serde_json::to_string(&move_to_play_result).unwrap();
    //     let deserialized: GameEvaluationResultMoveToPlay = serde_json::from_str(serialized.as_str()).unwrap();
    //     assert_eq!(
    //         deserialized,
    //         move_to_play_result,
    //     );
    // }
    //
    // #[test]
    // #[allow(non_snake_case)]
    // fn test_deserialize_GameEvaluationResultMoveToPlay() {
    //     let chosen_move = "a2-a4".parse::<Move>().unwrap();
    //     let eval = MoveEvaluation::Numeric(5.5);
    //     let fen = "rnbqkbnr/p1pppppp/1p6/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2".to_string();
    //     let json = move_to_play_to_json(chosen_move, eval, fen);
    //     let deserialized: GameEvaluationResultMoveToPlay = serde_json::from_str(json.as_str()).unwrap();
    //     assert_eq!(
    //         deserialized.move_to_play,
    //         "a2-a4",
    //     );
    //     assert_eq!(
    //         deserialized.fen,
    //         "rnbqkbnr/p1pppppp/1p6/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2".to_string(),
    //     );
    //     let deserialized_move_eval: MoveEvaluation = serde_json::from_str::<SerializableMoveEvaluation>(deserialized.eval.as_str()).unwrap().into();
    //     assert_eq!(
    //         deserialized_move_eval,
    //         MoveEvaluation::Numeric(5.5),
    //     );
    // }
}
