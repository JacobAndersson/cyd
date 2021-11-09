const { Writable } = require('stream');
const { spawn } = require('child_process');

const { streamMoves, postMove } = require('./move.js');
const { THREADS, DEBUG, DEPTH } = require('./env.js');

let GAME_ID = '';
let IS_WHITE = true;

class boardStateStream extends Writable {
  constructor() {
    super();
    this.process = null;
  }

  async _write(chunk, encoding, callback) {
    if (chunk.toString().length == 1) {
      callback();
      return;
    }

    let data = JSON.parse(chunk.toString());
    if (DEBUG) {
      console.log('Board state', chunk.toString());
    }

    const {
      type,
      id: gameId,
      state: gameState,
      status,
      white,
      winner,
    } = data;

    if (type == 'gameFull') {
      GAME_ID = gameId;
      if (white.name == 'c2d2') {
        IS_WHITE = true;
      } else {
        IS_WHITE = false;
      }
    }

    if (winner || status == 'aborted' || (!gameState && type == 'gamestate')) {
      this.game = null;
      return callback();
    }

    let moves = gameState?.moves || data.moves;
    let numMoves = moves?.split(' ')?.length || 0;

    if ((numMoves % 2 == 0 && IS_WHITE) || (numMoves % 2 == 1 && !IS_WHITE)) {
      if (!this.game) {
        this.game = new cyd(moves, DEPTH, THREADS);
      }

      this.game.makeMove(moves);
      const { move, score } = await this.game.getMove();
      console.log(`MOVE: ${move}, SCORE: ${score}`);
      postMove(GAME_ID, move);
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
    if (!chunk || chunk.toString().length == 1) {
      callback();
      return;
    }

    console.log('Game state');
    let event = JSON.parse(chunk);

    if (event.type === 'gameStart' && GAME_ID == '') {
      streamMoves(event.game.id, this.handler);
    }
    callback();
  }
}

class cyd {
  constructor(moves, depth, threads) {
    this.lastOutput;

    this.ready = false;
    this.process = spawn('./cyd', [
      '--depth',
      depth,
      '--num-threads',
      threads,
      '--alive',
    ]);

    this.process.stdout.on('error', (error) => {
      console.error(error);
    });

    this.process.on('close', (code) => {
      console.log('Search closed with', code);
    });

    this.process.stdout.on('data', (data) => {
      if (data.includes('move')) {
        let split = data
          .toString()
          .replace('move', '')
          .trim()
          .toString()
          .split(',');
        let move = split[0];
        let score = split[1];

        this.lastOutput = { move, score };
      }
    });
  }

  makeMove(moves) {
    if (moves) {
      let lastMove = moves.split(' ').pop() || 'con';
      this.process.stdin.write(lastMove + '\n');
    } else {
      this.process.stdin.write('con\n'); 
    }
  }

  async sleep(msec) {
    return new Promise((resolve) => setTimeout(resolve, msec));
  }

  async getMove() {
    while (!this.lastOutput) {
      await this.sleep(10);
    }
    let mv = this.lastOutput;
    this.lastOutput = null;
    return mv;
  }
}

module.exports = {
  game,
  boardStateStream,
  cyd,
};
