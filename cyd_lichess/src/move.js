const axios = require("axios");
const { TOKEN } = require("./env.js")

function streamMoves(id, handler) {
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

function postMove(gameId, move){
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


module.exports = {
  postMove, 
  streamMoves,
  getMove,
  spawnProcess,
}
