import { Universe } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

//
// Render settings
//

const DELAY_MS = 1000;

const CELL_SIZE = 16; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

//
// Object declarations
//

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;
const ctx = canvas.getContext("2d");

//
// Helper functions
//

const getIndex = (row, col) => {
  return row * width + col;
};

const bitIsSet = (n, arr) => {
  const byte = Math.floor(n / 8);
  const mask = 1 << n % 8;
  return (arr[byte] & mask) === mask;
};

//
// Render functions
//

const renderGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines
  for (let i = 0; i <= width; i++) {
    const x = i * (CELL_SIZE + 1) + 1;
    const topY = 0;
    const bottomY = canvas.height;
    ctx.moveTo(x, topY);
    ctx.lineTo(x, bottomY);
  }

  // Horizontal lines
  for (let j = 0; j <= height; j++) {
    const y = j * (CELL_SIZE + 1) + 1;
    const leftX = 0;
    const rightX = canvas.width;
    ctx.moveTo(leftX, y);
    ctx.lineTo(rightX, y);
  }

  ctx.stroke();
};

const renderCells = () => {
  const cells = new Uint8Array(
    memory.buffer,
    universe.cells(),
    (width * height) / 8
  );
  ctx.beginPath();
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = bitIsSet(idx, cells) ? ALIVE_COLOR : DEAD_COLOR;

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

//
// Randomize button
//

const randomizeButton = document.getElementById("randomize");
randomizeButton.addEventListener("click", (_) => {
  universe.randomize();
  render();
});

//
// Clear button
//

const clearButton = document.getElementById("clear");
clearButton.addEventListener("click", (_) => {
  universe.clear();
  render();
});

//
// Render loop
//

let previousTimeStamp;
let animationId;

const isPaused = () => {
  return animationId === null;
};

const render = () => {
  renderGrid();
  renderCells();
};

const step = (timeStamp) => {
  if (isPaused()) {
    return;
  }

  if (previousTimeStamp === undefined) {
    previousTimeStamp = timeStamp;
  }

  const elapsed = timeStamp - previousTimeStamp;
  if (elapsed > DELAY_MS) {
    previousTimeStamp = timeStamp;
    universe.tick();
    render();
    animationId = requestAnimationFrame(step);
  }

  animationId = requestAnimationFrame(step);
};

//
// Play/pause
//

const playPauseButton = document.getElementById("play-pause");

const play = () => {
  playPauseButton.textContent = "Pause";
  animationId = requestAnimationFrame(step);
};

const pause = () => {
  playPauseButton.textContent = "Play";
  cancelAnimationFrame(animationId);
  animationId = null;
};

playPauseButton.addEventListener("click", (_) => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

//
// Cell toggleability
//

canvas.addEventListener("click", (event) => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

  if (event.altKey) {
    // Insert a glider:
    universe.set_cell(row, col, true);
    universe.set_cell(row, col + 1, true);
    universe.set_cell(row + 1, col, true);
    universe.set_cell(row + height - 1, col + 1, true);
    universe.set_cell(row + height - 1, col + width - 1, true);
  } else {
    // Just toggle the cell normally:
    universe.toggle_cell(row, col);
  }

  render();
});

//
// Start simulation
//

render();
play();
