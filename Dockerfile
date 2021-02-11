FROM rust

WORKDIR /home

COPY . .

RUN rustup default nightly

#install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh



#build the wasm package with the target of web
RUN wasm-pack build wasm_builder --target web --release


#delete the old wasm files
RUN rm static/chesscheckersgame_static/wasmfiles/ -rf
#and remake the empty directory
RUN mkdir static/chesscheckersgame_static/wasmfiles/
#copy the package created into the frontend wasm file directory
RUN cp wasm_builder/pkg/wasm_builder.js static/chesscheckersgame_static/wasmfiles/
RUN cp wasm_builder/pkg/wasm_builder_bg.wasm static/chesscheckersgame_static/wasmfiles/



RUN cargo update
RUN cargo build --release


#ROCKET_ENV=prod

CMD cargo run --release