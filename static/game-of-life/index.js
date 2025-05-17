// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
//import greeter from "/static/game-of-life/pkg/wasm_game_of_life.js";
//
//greeter("moin");

import { greeter } from '/static/game-of-life/pkg/wasm_game_of_life.js3'

let greeting = greeter('Grafbase')

document.getElementById('container').innerText = greeting