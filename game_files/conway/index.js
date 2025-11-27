import init, { Universe } from './pkg/conway.js';

async function run() {
    let rustWasm = await init(); // Initialize WASM module

    const CELL_SIZE = 5;
    const GRID_COLOR = "#333333";
    const DEAD_COLOR = "#000000";
    const ALIVE_COLOR = "#FFFFFF";

    // These must match `Cell::Alive` and `Cell::Dead` in `src/lib.rs`.
    const DEAD = 0;
    const ALIVE = 1;

    const universe = Universe.new();
    const width = universe.width();
    const height = universe.height();

    // Initialize the canvas with room for all of our cells and a 1px border
    // around each of them.
    const canvas = document.getElementById("conway-canvas");
    canvas.height = (CELL_SIZE + 1) * height + 1;
    canvas.width = (CELL_SIZE + 1) * width + 1;

    const ctx = canvas.getContext('2d');

    let animationId = null;

    const renderLoop = () => {
        fps.render();

        universe.tick();

        drawCells();
        drawGrid();

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


        ctx.stroke();
    };

    const clearButton = document.getElementById("clear");

    clearButton.addEventListener("click", event => {
        universe.clear();

        drawCells();
        drawGrid();
    })

    const playPauseButton = document.getElementById("play-pause");

    const isPaused = () => {
        return animationId === null;
    };

    const play = () => {
        playPauseButton.textContent = "⏸";
        renderLoop();
    };

    const pause = () => {
        playPauseButton.textContent = "▶";
        cancelAnimationFrame(animationId);
        animationId = null;
    };

    playPauseButton.addEventListener("click", event => {
        if (isPaused()) {
            play();
        } else {
            pause();
        }
    });

    canvas.addEventListener("mousedown", event => {
        const boundingRect = canvas.getBoundingClientRect();

        const scaleX = canvas.width / boundingRect.width;
        const scaleY = canvas.height / boundingRect.height;

        const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
        const canvasTop = (event.clientY - boundingRect.top) * scaleY;

        const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
        const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

        if (event.button == 0) { // left click for mouse
            universe.add_glider(row, col)
        } else if (event.button == 1) { // wheel click for mouse
            universe.toggle_cell(row, col);
        } else if (event.button == 2) {   // right click for mouse
            //TODO
        }

        

        drawCells();
        drawGrid();

    });

    const fps = new class {
        constructor() {
            this.fps = document.getElementById("fps");
            this.frames = [];
            this.lastFrameTimeStamp = performance.now();
        }

        render() {
            // Convert the delta time since the last frame render into a measure
            // of frames per second.
            const now = performance.now();
            const delta = now - this.lastFrameTimeStamp;
            this.lastFrameTimeStamp = now;
            const fps = 1 / delta * 1000;

            // Save only the latest 100 timings.
            this.frames.push(fps);
            if (this.frames.length > 100) {
                this.frames.shift();
            }

            // Find the max, min, and mean of our 100 latest timings.
            let min = Infinity;
            let max = -Infinity;
            let sum = 0;
            for (let i = 0; i < this.frames.length; i++) {
                sum += this.frames[i];
                min = Math.min(this.frames[i], min);
                max = Math.max(this.frames[i], max);
            }
            let mean = sum / this.frames.length;

            // Render the statistics.
            this.fps.textContent = `
            Frames per Second:
                    latest = ${Math.round(fps)}
            avg of last 100 = ${Math.round(mean)}
            min of last 100 = ${Math.round(min)}
            max of last 100 = ${Math.round(max)}
            `.trim();
        }
    };


    play();
}

run();