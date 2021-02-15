#!/bin/bash

#build the wasm package with the target of web
wasm-pack build wasm_builder --target web --release


#delete the old wasm files
rm static/chesscheckersgame_static/wasmfiles/ -rf
#and remake the empty directory
mkdir static/chesscheckersgame_static/wasmfiles/
#copy the package created into the frontend wasm file directory
cp wasm_builder/pkg/wasm_builder.js static/chesscheckersgame_static/wasmfiles/
cp wasm_builder/pkg/wasm_builder_bg.wasm static/chesscheckersgame_static/wasmfiles/