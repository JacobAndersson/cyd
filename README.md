# Cyd

Chess bot written in rust.

The bot comes in two parts, the actual bot which plays chess and the part that talks with lichess. To use the bot, you need to compile the bot using wasm and the run `npm link` in the `/pkg` directory. Then also link in the `cyd_lichess` directory. After that just run `API_TOKEN=TOKEN npm start`
