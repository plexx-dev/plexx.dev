import init, { Universe } from './pkg/snake.js';

async function run() {
    let rustWasm = await init(); // Initialize WASM module

    const CELL_SIZE = 30;
    const GRID_COLOR = "#333333";
    const DEAD_COLOR = "#000000";
    const ALIVE_COLOR = "#FFFFFF";
    const APPLE_COLOR = "#FF0000"

    // These must match `Cell::Alive` and `Cell::Dead` in `src/lib.rs`.
    const DEAD = 0;
    const ALIVE = 1;
    const APPLE = 2;

    const universe = Universe.new();
    const width = universe.width();
    const height = universe.height();

    universe.spawn_snake();

    // Initialize the canvas with room for all of our cells and a 1px border
    // around each of them.
    const canvas = document.getElementById("snake-canvas");
    canvas.height = (CELL_SIZE + 1) * height + 1;
    canvas.width = (CELL_SIZE + 1) * width + 1;

    const ctx = canvas.getContext('2d');

    let animationId = null;

    const frameDuration = 1000 / universe.get_speed(); // in milliseconds
    let lastTime = performance.now();

    const renderLoop = () => {
        const now = performance.now();
        const elapsed = now - lastTime;

        if (elapsed >= frameDuration) {
            lastTime = now;
            universe.tick();

            drawCells();
            drawGrid();

            document.getElementById('score').innerText = `Score: ${universe.get_score()}`;
        }

        animationId = requestAnimationFrame(renderLoop);
    };

    const drawGrid = () => {
        ctx.beginPath();
        ctx.lineWidth = 1 / window.devicePixelRatio;
        ctx.strokeStyle = GRID_COLOR;

        // Vertical lines.
        for (let i = 0; i <= width; i++) {
            ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
            ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
        }

        // Horizontal lines.
        for (let j = 0; j <= height; j++) {
            ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
            ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
        }

        ctx.stroke();
    };

    const getIndex = (row, column) => {
        return row * width + column;
    };

    const drawCells = () => {
        const cellsPtr = universe.cells();
        const cells = new Uint8Array(rustWasm.memory.buffer, cellsPtr, width * height);

        ctx.beginPath();

        // Alive cells.
        ctx.fillStyle = ALIVE_COLOR;
        for (let row = 0; row < height; row++) {
            for (let col = 0; col < width; col++) {
                const idx = getIndex(row, col);
                if (cells[idx] !== ALIVE) {
                    continue;
                }

                ctx.fillRect(
                    col * (CELL_SIZE + 1) + 1,
                    row * (CELL_SIZE + 1) + 1,
                    CELL_SIZE,
                    CELL_SIZE
                );
            }
        }

        // Dead cells.
        ctx.fillStyle = DEAD_COLOR;
        for (let row = 0; row < height; row++) {
            for (let col = 0; col < width; col++) {
                const idx = getIndex(row, col);
                if (cells[idx] !== DEAD) {
                    continue;
                }

                ctx.fillRect(
                    col * (CELL_SIZE + 1) + 1,
                    row * (CELL_SIZE + 1) + 1,
                    CELL_SIZE,
                    CELL_SIZE
                );
            }
        }

        // Dead cells.
        ctx.fillStyle = APPLE_COLOR;
        for (let row = 0; row < height; row++) {
            for (let col = 0; col < width; col++) {
                const idx = getIndex(row, col);
                if (cells[idx] !== APPLE) {
                    continue;
                }

                ctx.fillRect(
                    col * (CELL_SIZE + 1) + 1,
                    row * (CELL_SIZE + 1) + 1,
                    CELL_SIZE,
                    CELL_SIZE
                );
            }
        }


        ctx.stroke();
    };

    document.addEventListener('keydown', (event) => {
        let direction = -1;
        
        switch(event.key.toLowerCase()) {
            case 'arrowup':
            case 'w':
                universe.change_direction(0);
                break;
            case 'arrowright':
            case 'd':
                universe.change_direction(1);
                break;
            case 'arrowdown':
            case 's':
                universe.change_direction(2);
                break;
            case 'arrowleft':
            case 'a':
                universe.change_direction(3);
                break;
        }
        
        if (direction !== -1) {
            //event.preventDefault(); // Prevent default browser actions (like scrolling)
            //callback(direction);
        }
    });

    const playText = document.getElementById("start");

    playText.addEventListener("click", event => {
        universe.tick();
    })

    const isPaused = () => {
        return animationId === null;
    };

    const play = () => {
        renderLoop();
    };

    const pause = () => {
        cancelAnimationFrame(animationId);
        animationId = null;
    };

    play();
}

run();