const axios = require("axios");
const { boardStateStream, game, cyd} = require("./game.js");

const { TOKEN } = require("./env.js");

async function sleep(msec) {
  return new Promise(resolve => setTimeout(resolve, msec));
}


async function main() {
  const moveStream = new boardStateStream();
  const eventHandler = new game(moveStream);


  /*
  const c = new cyd("", 2,  1); 
  await sleep(200);

  await c.makeMove("e2e4")
  let mv = await c.getMove();
  console.log("RETURN", mv);

  await c.makeMove("a2a4")
  let mv2 = await c.getMove();
  console.log("RETURN", mv2);
  */

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
