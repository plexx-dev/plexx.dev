import init, { Universe } from './pkg/snake.js';

async function run() {
    let rustWasm = await init(); // Initialize WASM module

    const CELL_SIZE = 30;
    const GRID_COLOR = "#333333";
    const DEAD_COLOR = "#000000";
    const ALIVE_COLOR = "#FFFFFF";
    const ALIVE_HEAD_COLOR = "#00FF00";
    const APPLE_COLOR = "#FF0000"

    // These must match `Cell::Alive` and `Cell::Dead` in `src/lib.rs`.
    const DEAD = 0;
    const ALIVE = 1;
    const APPLE = 2;

    const universe = Universe.new();
    const width = universe.width();
    const height = universe.height();

    universe.spawn_snake();
    universe.spawn_apple();

    // Initialize the canvas with room for all of our cells and a 1px border
    // around each of them.
    const canvas = document.getElementById("snake-canvas");
    canvas.height = (CELL_SIZE + 1) * height + 1;
    canvas.width = (CELL_SIZE + 1) * width + 1;

    const ctx = canvas.getContext('2d');

    let animationId = null;

    const frameDuration = 1000 / universe.get_speed(); // in milliseconds
    let lastTime = performance.now();

    let lastScore = -1;
    
    const renderLoop = () => {
        const now = performance.now();
        const elapsed = now - lastTime;

        if (elapsed >= frameDuration) {
            lastTime = now;
            universe.tick();

            drawCells();
            drawGrid();

            let currentScore = universe.get_score();
            if (universe.get_score() != lastScore) {
                document.getElementById('score').innerText = `Score: ${currentScore}`;
                lastScore = currentScore;
            }
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

        //SNAKE HEAD
        ctx.fillStyle = ALIVE_HEAD_COLOR;
        let head_pos = universe.get_snake_head();
        let row = Math.floor(head_pos / width);
        let col = head_pos % width

        ctx.fillRect(
            col * (CELL_SIZE + 1) + 1,
            row * (CELL_SIZE + 1) + 1,
            CELL_SIZE,
            CELL_SIZE
        );

        // Alive cells.
        ctx.fillStyle = ALIVE_COLOR;
        for (let row = 0; row < height; row++) {
            for (let col = 0; col < width; col++) {
                const idx = getIndex(row, col);
                if (cells[idx] !== ALIVE || idx == head_pos) {
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

        // Apple cells.
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

    const start_text = document.getElementById("start-text");

    document.addEventListener('keydown', (event) => {
        let direction = -1;

        if (universe.get_is_paused()) {
            switch (event.key.toLowerCase()) {
                case 'arrowup':
                case 'w':
                case 'arrowright':
                case 'd':
                case 'arrowdown':
                case 's':
                case 'arrowleft':
                case 'a':
                case ' ':
                    universe.start_stop_toggle();
                    start_text.classList.toggle('hidden');
                    break;
            }

            return;
        } else {
            switch (event.key.toLowerCase()) {
                case ' ':
                    universe.start_stop_toggle();
                    start_text.classList.toggle('hidden');
                    break;
            }
        }

        switch (event.key.toLowerCase()) {
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

    const play_pause = document.getElementById("start");

    play_pause.addEventListener("click", event => {
        universe.start_stop_toggle();
        start_text.classList.toggle('hidden');
    })

    renderLoop();
}

run();