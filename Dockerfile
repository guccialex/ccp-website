FROM rust

RUN cargo install wasm-pack

WORKDIR /home

COPY . .

EXPOSE 8080
EXPOSE 8082

CMD ./runeverything.sh
