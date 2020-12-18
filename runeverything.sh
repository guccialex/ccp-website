#!/bin/bash



#pull the ccp-game from the server to be used as a dependancy for single_server
svn co https://github.com/guccialex/ccp-game.git/trunk/chesspoker_package



#delete the old wasm files
rm chesscheckersgame_static/wasmfiles/ -r
#and remake the empty directory
mkdir chesscheckersgame_static/wasmfiles

cd wasm_builder

#build the wasm package with the target of web
wasm-pack build --target web

#copy the package created into the frontend wasm file directory
cp pkg/wasm_builder.js ../chesscheckersgame_static/wasmfiles/
cp pkg/wasm_builder_bg.wasm ../chesscheckersgame_static/wasmfiles/

cd ..



cd gamefinder_static


python3 -m http.server 8082 &


cd ..
cd chesscheckersgame_static


python3 -m http.server 8000





#run the nginx dockerfile that will copy the static files into it
#and serve them statically with nginx

#serve the gamefinder_static files and the chessscheckersgame files


#echo "running the server that serves the webpage to find a game"


#create a docker image to then deploy




