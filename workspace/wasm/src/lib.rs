extern crate core;

use std::convert::{Into};
use CastlingType::{KingSide, QueenSide};
use chess_compress_urlsafe::a_move::{CastlingType, MoveData, MoveType};
use chess_compress_urlsafe::a_move::MoveType::PawnPromotion;
use chess_compress_urlsafe::decompress::decompress;
use chess_compress_urlsafe::FigureType;
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
    let moves_result = match decompress(base64_encoded) {
        Ok((positions_data, moves_data)) => {
            let vec_of_fen: Vec<String> = positions_data.into_iter().map(|it|it.fen).collect();
            let vec_of_moves: Vec<String> = moves_data.into_iter().map(|it| { to_move_notation(it) }).collect();

            serde_json::to_string(&Game { vec_of_fen, vec_of_moves }).map(|game_json|{
                JsonResult {
                    is_ok: true,
                    value: game_json,
                }
            }).unwrap_or_else(|serde_err|{
                JsonResult {
                    is_ok: false,
                    value: serde_err.to_string(),
                }
            })
        }
        Err(err) => {
            JsonResult {
                is_ok: false,
                value: err.to_string(),
            }
        }
    };
    let json = serde_json::to_string(&moves_result).unwrap_or_else(|_| "{\"is_ok\": false, \"value\": \"Serialization failed\"}".to_string());
    JsValue::from_str(json.as_str())
}

fn to_move_notation(move_data: MoveData) -> String {
    if let MoveType::Castling {castling_type, .. } = move_data.move_type {
        return match castling_type {
            KingSide => "O-O",
            QueenSide => "O-O-O",
        }.to_string();
    };

    let mut move_str = String::with_capacity(6);
    if move_data.figure_moved != FigureType::Pawn {
        move_str.push(move_data.figure_moved.as_encoded());
    };

    let from_to = move_data.given_from_to;
    if move_data.did_catch_figure() {
        move_str.push_str(format!("{}x{}", from_to.from, from_to.to).as_str());
    } else {
        move_str.push_str(from_to.to_string().as_str());
    }

    if let PawnPromotion{promoted_to} = move_data.move_type {
        move_str.push('=');
        move_str.push(promoted_to.as_encoded());
    };

    move_str
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
