#!/bin/bash


#pull the ccp-game from the server to be used as a dependancy for single_server

#svn co https://github.com/guccialex/ccp-game.git/trunk/chesspoker_package



#install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh


#build the wasm package with the target of web
cd wasm_builder
wasm-pack build --target web --release
cd ..


#delete the old wasm files
rm static/chesscheckersgame_static/wasmfiles/ -r
#and remake the empty directory
mkdir static/chesscheckersgame_static/wasmfiles/


#copy the package created into the frontend wasm file directory
cp wasm_builder/pkg/wasm_builder.js static/chesscheckersgame_static/wasmfiles/
cp wasm_builder/pkg/wasm_builder_bg.wasm static/chesscheckersgame_static/wasmfiles/
