const axios = require("axios");
const { spawn } = require('child_process');

const DEBUG = process.env.DEBUG || 0;
const TOKEN = process.env.TOKEN;
const DEPTH = process.env.DEPTH || 5;
const THREADS = 1;

var fs = require("fs");
var { Writable } = require("stream");

let GAME_ID = "";
let IS_WHITE = true;

function getMove(moves, depth, threads){
  let process = spawn("./cyd", ["--moves", moves, "--depth",  depth, "--num-threads",  threads]);
  
  return new Promise((resolve, reject) => {
    process.stdout.on("data", (data) => {
      let split = data.toString().split(",");
      let move = split[0];
      let score = split[1];
      resolve({move, score});
    });

    process.stdout.on("error", (error) => {
      console.error(error);
      reject(error);
    });

    process.on("error", (error) => {
      reject(error);
    })

    process.on("close", (code) => {
      console.log("Search closed with", code);
    })
  })
}

function getMoves(id, handler) {
  axios
    .get(`https://lichess.org/api/bot/game/stream/${id}`, {
      headers: {
        Authorization: `Bearer ${TOKEN}`,
      },
      responseType: "stream",
    })
    .then((response) => {
      response.data.pipe(handler);
    })
    .catch(err => {
      console.error(err);
    });
}

function makeMove(gameId, move){
  axios
    .post(`https://lichess.org/api/bot/game/${gameId}/move/${move}`,{}, {
      headers: {
        Authorization: `Bearer ${TOKEN}`,
      }
    })
    .then((response) => {
      console.log("Succesfully made move");
    })
    .catch(err => {
      console.error(err);
    });
}

class boardStateStream extends Writable {
  
  async _write(chunk, encoding, callback) {
    if (chunk.toString().length == 1){
      callback();
      return;
    }

    let data = JSON.parse(chunk.toString());
    if (DEBUG){
      console.log('Board state', chunk.toString());
    }

    const { type, id: gameId, state: gameState, status, white, black, winner } = data;
    if (type == "gameFull"){
      GAME_ID = gameId;
      if (white.name == "c2d2"){
        IS_WHITE = true;
      } else {
        IS_WHITE = false;
      }
    }

    if (winner) {
      return callback();
    }

    if (status == 'aborted'){
      return callback();
    }
    
    if (!gameState && type == "gamestate"){
      return callback();
    }

    let moves = gameState?.moves || data.moves;
    let numMoves = moves?.split(" ")?.length || 0;

    if (numMoves % 2 == 0 && IS_WHITE || numMoves % 2 == 1 && !IS_WHITE) {
      const { move, score } = await getMove(moves, DEPTH, THREADS)
      console.log(`MOVE: ${move}, SCORE: ${score}`);
      makeMove(GAME_ID, move);
    }

    callback();
  }
}

class game extends Writable {
  constructor(handler) {
    super();
    this.handler = handler;
  }

  _write(chunk, encoding, callback) {
    if (!chunk || chunk.toString().length == 1){
      callback();
      return;
    }

    console.log('Game state');
    let event = JSON.parse(chunk);

    if (event.type === "gameStart" && GAME_ID == "") {
      getMoves(event.game.id, this.handler);
    }
    callback();
  }
}

function main() {
  const moveStream = new boardStateStream();
  const eventHandler = new game(moveStream);

  axios
    .get("https://lichess.org/api/stream/event", {
      headers: {
        Authorization: `Bearer ${TOKEN}`,
      },
      responseType: "stream",
    })
    .then((response) => {
      response.data.pipe(eventHandler);
    });
}
main();
