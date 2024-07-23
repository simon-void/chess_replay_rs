import init, * as wasm from './decode_chess_wasm/decode_chess_wasm.js';
import {Chessboard, INPUT_EVENT_TYPE, MOVE_INPUT_MODE} from "./cm-chessboard/Chessboard.js"


let init_Promise = init();

async function init_wasm() {
    await init_Promise;
}

async function decompress(urlsafe_game_notation) {
    await init_wasm()
    let gameData = GameData();
    let json_result_serialized = await wasm.decode_moves(urlsafe_game_notation);
    let json_result = JSON.parse(json_result_serialized);
    if (json_result.is_ok) {
        // log("Decompression result:" + json_result.value)
        let game = JSON.parse(json_result.value);
        gameData.moves = game.vec_of_moves;
        gameData.positions = game.vec_of_fen;
    } else {
        log("Decompression error:" + json_result.value)
    }
    return gameData;
}

function GameData() {
    let self = {
        moves: [],
        positions: ["rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"],  // classic starting position
    };
    return self;
}

const output = document.getElementById("output")

function log(text) {
    const log = document.createElement("div");
    log.innerText = text;
    output.prepend(log);
}

window.onload = function () {

    init_wasm().then(_ => {
        // gameModel.state(states.HUMAN_TURN);
        // "Y3vghpnyfWW7Q" -> "a2a4, h7h6, a4a5, b7b5, a5b6, h6h5, b6c7, h5h4, g2g3, h4g3, c7d8Q"
        // "TuCU2BS-tDL8_EA" -> "d2d3, g7g6, c1e3, f8g7, b1c3, g8f6, d1d2, e8h8, e1a1"
        let compressed_game = "TuCU2BS-tDL8_EA";
        decompress(compressed_game).then(gameData => {
            let gameModel = new GameState(gameData);
            ko.applyBindings(gameModel);
        });

        log("match in compressed notation: "+compressed_game);
    }, reason => {
        alert("Couldn't initialise wasm: " + reason);
    });
}

function UiModel(gameState) {
    let self = this;

    {
        // The site displays a warning if WebAssembly isn't available in the user's browser.
        // But if this point is reached, WebAssembly is present and the warning can be disabled and the main content enabled.
        document.getElementById("no-wasm-warning").style = "display: none";
        document.getElementById("main").style = "display: block";
    }

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
    self.setPosition = function (fen) {
        // log("new fen: "+ fen)
        self.board.setPosition(fen);
    }
    self.setProgress = function (fen) {
        let progressDiv = document.getElementById("progress");
        return function (msg) {
            progressDiv.innerText = msg;
        };
    }();

    document.getElementById("to_start_button").onclick = function() {gameState.startPosition();};
    document.getElementById("previous_button").onclick = function() {gameState.previousPosition();};
    document.getElementById("next_button").onclick = function() {gameState.nextPosition();};
    document.getElementById("to_end_button").onclick = function() {gameState.endPosition();};

    return self;
}

function GameState(gameData) {
    let self = this;
    self._positionIndex = ko.observable(0);
    self.startPosition = function () {
        self._positionIndex(0);
    };
    self.endPosition = function () {
        self._positionIndex(gameData.moves.length);
    };
    self.nextPosition = function () {
        let nextIndex = self._positionIndex() + 1;
        if(nextIndex<gameData.positions.length) {
            self._positionIndex(nextIndex);
        }
    };
    self.previousPosition = function () {
        let nextIndex = self._positionIndex() - 1;
        if(nextIndex>=0) {
            self._positionIndex(nextIndex);
        }
    };
    // this.fullName = ko.computed(function() {
    //     return this.firstName() + " " + this.lastName();
    // }, this);

    let uiModel = new UiModel(self);
    uiModel.setPosition(gameData.positions[0]);
    const nrOfMoves = gameData.moves.length;
    uiModel.setProgress("moves played: 0/"+nrOfMoves);
    self._positionIndex.subscribe(function(newIndex) {
        uiModel.setPosition(gameData.positions[newIndex]);
        uiModel.setProgress("moves played: "+newIndex+"/"+nrOfMoves);
    });
}