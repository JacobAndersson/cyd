const cyd = require("cyd");
const axios = require("axios");

const TOKEN = process.env.TOKEN;
var fs = require("fs");
var { Writable } = require("stream");

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
      console.log(response.data);
    })
    .catch(err => {
      console.error(err);
    });
}

class boardStateStream extends Writable {
  _write(chunk, encoding, callback) {
    console.log('Board state', chunk.toString());
    if (chunk.toString().length == 1){
      callback();
      return;
    }
    const { id: gameId, state: gameState, status }= JSON.parse(chunk);
    if (status == 'aborted'){
      return callback();
    }
    const { moves } = gameState;
    makeMove(gameId, "a4a5");
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

    if (event.type === "gameStart") {
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
let move = cyd.find_move("", 6, 3);
console.log(move);
//main();
