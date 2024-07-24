import init, * as wasm from './decode_chess_wasm/decode_chess_wasm.js';
import {Chessboard, MOVE_INPUT_MODE} from "./cm-chessboard/Chessboard.js"


let init_Promise = init();

async function init_wasm() {
    await init_Promise;
}

window.onload = function () {
    // The site displays a warning if WebAssembly isn't available in the user's browser.
    // But if this point is reached, WebAssembly is present and the warning can be disabled and the main content enabled.
    let noWasmWarning = document.getElementById("no-wasm-warning");
    let pageLoadingSpinner = document.getElementById("page-loading");
    let mainDiv = document.getElementById("main");

    const queryParams = new Proxy(new URLSearchParams(window.location.search), {
        get: (searchParams, prop) => searchParams.get(prop),
    });

    const defaultMatch = "TuCU2BS-tDL8_EA3nvfeW2P9GR"; // d2d3, g7g6, c1e3, f8g7, b1c3, g8f6, d1d2, e8h8, e1a1, h7h5, e3h6, h5h4, g2g4, h4g3, h6g7, g3h2, g7f8, h2g1R
    const queryMatch = queryParams.moves;
    const effectiveMatch = queryMatch ? queryMatch : defaultMatch;

    if (!queryMatch) {
        log("no query parameter 'moves' found. using default moves=" + defaultMatch);
    }

    init_wasm().then(_ => {
        decompress(effectiveMatch).then(gameData => {
            pageLoadingSpinner.remove();
            mainDiv.style = "display: block";

            let gameModel = new GameState(gameData);
            ko.applyBindings(gameModel);
        });
    }, reason => {
        pageLoadingSpinner.remove();
        noWasmWarning.style = "display: block; background-color: lightpink";
        console.log("init_wasm failed with: " + reason);
    });
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
    return {
        moves: [],
        positions: ["rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"],  // classic starting position
    };
}

const output = document.getElementById("output")

function log(text) {
    const log = document.createElement("div");
    log.innerText = text;
    output.prepend(log);
}

function UiModel(gameState) {
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
    self.setPosition = function (fen) {
        // log("new fen: "+ fen)
        self.board.setPosition(fen);
    }
    self.setProgress = function () {
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
        if (nextIndex < gameData.positions.length) {
            self._positionIndex(nextIndex);
        }
    };
    self.previousPosition = function () {
        let nextIndex = self._positionIndex() - 1;
        if (nextIndex >= 0) {
            self._positionIndex(nextIndex);
        }
    };
    // this.fullName = ko.computed(function() {
    //     return this.firstName() + " " + this.lastName();
    // }, this);

    let uiModel = new UiModel(self);
    uiModel.setPosition(gameData.positions[0]);
    const nrOfMoves = gameData.moves.length;
    uiModel.setProgress("moves played: 0/" + nrOfMoves);
    self._positionIndex.subscribe(function (newIndex) {
        uiModel.setPosition(gameData.positions[newIndex]);
        uiModel.setProgress("moves played: " + newIndex + "/" + nrOfMoves);
    });
}