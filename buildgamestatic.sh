#!/bin/bash


#pull the ccp-game from the server to be used as a dependancy for single_server
svn co https://github.com/guccialex/ccp-game.git/trunk/chesspoker_package


#delete the old wasm files
rm chesscheckersgame_static/wasmfiles/ -r
#and remake the empty directory
mkdir chesscheckersgame_static/wasmfiles

cd wasm_builder


#install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

#build the wasm package with the target of web
wasm-pack build --target web

#copy the package created into the frontend wasm file directory
cp pkg/wasm_builder.js ../chesscheckersgame_static/wasmfiles/
cp pkg/wasm_builder_bg.wasm ../chesscheckersgame_static/wasmfiles/