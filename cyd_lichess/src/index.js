const cyd = require("cyd");
const axios = require("axios");

const TOKEN = process.env.TOKEN;
var fs = require("fs");
var { Writable } = require("stream");

function getMoves(id, handler) {
  axios
    .get("https://lichess.org/api/stream/event", {
      headers: {
        Authorization: `Bearer ${TOKEN}`,
      },
      responseType: "stream",
    })
    .then((response) => {
      response.data.pipe(handler);
    });
}

class boardStateStream extends Writable {
  _write(chunk, encoding, callback) {
    console.log(JSON.parse(chunk));
  }
}

class game extends Writable {
  constructor(options, handler) {
    super(options);
    console.log(options);
    console.log(handler);
  }

  _write(chunk, encoding, callback) {
    console.log("HERE");
    console.log(chunk.toString());
    let event = JSON.parse(chunk);

    if (event.type === "gameStart") {
      getMoves(event.game.id, this.handler);
    }
  }
}

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
