ChessReplayRS - WiP (not functional yet)
=================

A stateless website to replay chess games provided as query param [here](https://TODO).
The chess logic necessary to replay the game is implemented in Rust and compiled to WebAssembly.



## Build Wasm Module

by executing the bash scripts `wasm-pack-and-copy-over_dev` or `wasm-pack-and-copy-over_release`.
This will update the voidchess_engine_wasm-module in the docs/engine folder.
