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

PROJNAME=storage_size
wat2wasm -o $PROJNAME.wasm $PROJNAME.wat
wasm-prune --exports call,deploy $PROJNAME.wasm $PROJNAME-pruned.wasm

PROJNAME=call_flipper_runtime
wat2wasm -o $PROJNAME.wasm $PROJNAME.wat
wasm-prune --exports call,deploy $PROJNAME.wasm $PROJNAME-pruned.wasm

PROJNAME=storage_runtime_calls
wat2wasm -o $PROJNAME.wasm $PROJNAME.wat
wasm-prune --exports call,deploy $PROJNAME.wasm $PROJNAME-pruned.wasm

PROJNAME=storage_runtime_demo
wat2wasm -o $PROJNAME.wasm $PROJNAME.wat
wasm-prune --exports call,deploy $PROJNAME.wasm $PROJNAME-pruned.wasm