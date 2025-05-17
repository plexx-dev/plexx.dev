import init, { greet } from './pkg/conway.js';

async function run() {
    await init(); // Initialize WASM module
    document.getElementById('greet-button').addEventListener('click', () => {
        alert(greet("Rust Developer"));
    });
}

run();