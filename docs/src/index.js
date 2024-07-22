import init, * as wasm from './decode_chess_wasm/decode_chess_wasm.js';
import {Chessboard, INPUT_EVENT_TYPE, MOVE_INPUT_MODE} from "./cm-chessboard/Chessboard.js"


let init_Promise = init();

async function init_wasm() {
    await init_Promise;
}

async function decompress(urlsafe_game_notation) {
    await init_wasm()
    let json_result_serialized = await wasm.decode_moves(urlsafe_game_notation);
    let json_result = JSON.parse(json_result_serialized);
    if (json_result.is_ok) {
        // log("Decompression result:" + json_result.value)
        let game = JSON.parse(json_result.value);

        log("vec_of_fen length: "+ game.vec_of_fen.length)
        log("vec_of_moves length: "+ game.vec_of_moves.length)
        log("moves: "+game.vec_of_moves)
    } else {
        log("Decompression error:" + json_result.value)
    }
    // return JSON.parse(fen_and_move_vecs);
}

const output = document.getElementById("output")

function log(text) {
    const log = document.createElement("div");
    log.innerText = text;
    output.prepend(log);
}

window.onload = function () {
    let gameModel = new GameModel();
    ko.applyBindings(gameModel);

    init_wasm().then(_ => {
        // gameModel.state(states.HUMAN_TURN);
        // "Y3vghpnyfWW7Q" -> "a2a4, h7h6, a4a5, b7b5, a5b6, h6h5, b6c7, h5h4, g2g3, h4g3, c7d8Q"
        // "TuCU2BS-tDL8_EA" -> "d2d3, g7g6, c1e3, f8g7, b1c3, g8f6, d1d2, e8h8, e1a1"
        decompress("Y3vghpnyfWW7Q")

        alert("decompressed");
    }, reason => {
        alert("Couldn't initialise wasm: " + reason);
    });
}

const states = {
    LOADING: "loading",
    REPLAY: "replay",
    GAME_ENDED: "game ended",
};
const moveTypes = {
    NORMAL: "normal",
    PAWN_PROMOTION: "pawn_promo",
    EN_PASSANT: "en_passant",
    SHORT_CASTLING: "short_castling",
    LONG_CASTLING: "long_castling",
};
const gameEvalTypes = {
    GAME_ENDED: "GameEnded",
    MOVE_TO_PLAY: "MoveToPlay",
    ERROR: "Err",
};

function BoardModel(gameModel) {
    let self = this;
    self.board = new Chessboard(
        document.getElementById("board"),
        {
            position: "start",
            moveInputMode: MOVE_INPUT_MODE.viewOnly,
            sprite: {url: "./assets/images/chessboard-sprite.svg"},
            style: {
                // cssClass: "default",
                showCoordinates: true, // show ranks and files
                showBorder: true, // display a border around the board
            }
        }
    );
    self.board.enableMoveInput(event => {
        log("enableMoveInput invoked")
        // switch (event.type) {
        //     case INPUT_EVENT_TYPE.moveStart:
        //         return gameModel.allowedMoveMap().has(event.square);
        //     case INPUT_EVENT_TYPE.moveDone:
        //         let moveOrNull = gameModel.allowedMoveMap().get(event.squareFrom).find(move=>move.to===event.squareTo);
        //         let move_accepted = moveOrNull != null;
        //         if(move_accepted) {
        //             setTimeout(()=>{
        //                 self.takeCareOfSpecialMoves(moveOrNull);
        //                 gameModel.informOfMove(moveOrNull)
        //             },0);
        //         }
        //         return move_accepted;
        //     case INPUT_EVENT_TYPE.moveCanceled:
        //         //log(`moveCanceled`)
        // }
    });
    self.takeCareOfSpecialMoves = function (move) {
        log("takeCareOfSpecialMoves invoked")
    //     if(move.type!==moveTypes.NORMAL) {
    //         let moves_plus_ongoing_move = [...gameModel.moveStrPlayed(), move.asStr];
    //         getFenResult(moves_plus_ongoing_move).then(fenResult => {
    //                 if (fenResult.is_ok) {
    //                     let fen = fenResult.value;
    //                     self.board.setPosition(fen);
    //                 } else {
    //                     log(fenResult.value);
    //                 }
    //             }, reason => {
    //                 log(`error when invoking getFenResult: ${reason}`);
    //             }
    //         )
    //     }
    }
}

function GameModel() {
    let self = this;
    self.evaluation = ko.observable("no evaluation");
    self.state = ko.observable(states.LOADING)
    // self.allowedMoveStrArray = ko.observableArray(_allowedMoveStrArrayClassic);
    // self.allowedMoveMap = ko.computed(()=>{
    //     return arrayOfMovesToMoveMap(self.allowedMoveStrArray());
    // });
    self.moveStrPlayed = ko.observableArray([]);
    // this.fullName = ko.computed(function() {
    //     return this.firstName() + " " + this.lastName();
    // }, this);
    self.boardModel = new BoardModel(self);
    // self.informOfMove = function (move) {
    //     let possibleMoves = [...self.allowedMoveStrArray()];
    //     self.allowedMoveStrArray([]);
    //     self.moveStrPlayed.push(move.asStr);
    //     self.state(states.ENGINE_TURN);
    //
    //     evaluateGame(
    //         [...self.moveStrPlayed()],
    //         possibleMoves,
    //         self.evaluation,
    //     ).then(
    //         (gameEval) => {
    //             if (gameEval.result_type === gameEvalTypes.ERROR) {
    //                 log(gameEval.msg);
    //             }
    //             if (gameEval.result_type === gameEvalTypes.GAME_ENDED) {
    //                 self.evaluation(gameEval.msg);
    //                 self.state(states.GAME_ENDED);
    //             }
    //             if (gameEval.result_type === gameEvalTypes.MOVE_TO_PLAY) {
    //                 self.moveStrPlayed.push(gameEval.move_to_play);
    //                 self.evaluation(gameEval.eval);
    //                 let fen = gameEval.fen;
    //                 self.boardModel.board.setPosition(fen);
    //
    //                 getAllowedMovesAsArray(self.moveStrPlayed()).then(
    //                     newAllowedMovesArray => {
    //                         if (newAllowedMovesArray.length === 0) {
    //                             log("no moves left")
    //                         }
    //                         self.allowedMoveStrArray(newAllowedMovesArray);
    //                     }, reason => {
    //                         alert(`couldn't compute allowed moves because of ${reason}`)
    //                     }
    //                 )
    //
    //                 self.state(states.HUMAN_TURN);
    //             }
    //         }
    //     );
    // };
}