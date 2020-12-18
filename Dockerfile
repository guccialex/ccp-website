FROM rust

RUN cargo install wasm-pack

RUN echo "yo"

RUN ls

WORKDIR /home

RUN ls

COPY . .

RUN ls


CMD ./runeverything.sh
