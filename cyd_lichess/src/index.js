const axios = require('axios');
const { boardStateStream, game } = require('./game.js');
const { TOKEN } = require('./env.js');

async function main() {
  const moveStream = new boardStateStream();
  const eventHandler = new game(moveStream);

  axios
    .get('https://lichess.org/api/stream/event', {
      headers: {
        Authorization: `Bearer ${TOKEN}`,
      },
      responseType: 'stream',
    })
    .then((response) => {
      response.data.pipe(eventHandler);
    });
}
main();
