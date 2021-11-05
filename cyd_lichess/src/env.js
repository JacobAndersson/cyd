const envalid = require('envalid');

const { str, num } = envalid;

module.exports = envalid.cleanEnv(process.env, {
  DEBUG: num({ default: 0 }),
  TOKEN: str({}),
  DEPTH: num({ default: 5 }),
  THREADS: num({ default: 1 }),
});
