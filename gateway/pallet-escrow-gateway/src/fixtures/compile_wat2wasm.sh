#!/bin/bash

set -e

PROJNAME=return_from_start_fn
wat2wasm -o $PROJNAME.wasm $PROJNAME.wat
wasm-prune --exports call,deploy $PROJNAME.wasm $PROJNAME-pruned.wasm

PROJNAME=transfer_return_code
wat2wasm -o $PROJNAME.wasm $PROJNAME.wat
wasm-prune --exports call,deploy $PROJNAME.wasm $PROJNAME-pruned.wasm

PROJNAME=transfer_big_amount_return_code
wat2wasm -o $PROJNAME.wasm $PROJNAME.wat
wasm-prune --exports call,deploy $PROJNAME.wasm $PROJNAME-pruned.wasm