FROM rust

RUN cargo install wasm-pack

WORKDIR /home

COPY . .

CMD ./runeverything.sh
