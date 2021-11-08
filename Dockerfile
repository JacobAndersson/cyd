FROM node:17

RUN DEBIAN_FRONTEND=nointeractive curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y && exec bash && $HOME/.cargo/env

ENV PATH="/root/.cargo/bin:${PATH}"

ADD cyd ./cyd
ADD cyd_lichess ./cyd_lichess

WORKDIR /cyd
RUN ./build.sh && mv /cyd/target/release/cyd /cyd_lichess/

WORKDIR /cyd_lichess
RUN npm ci .
CMD npm start
