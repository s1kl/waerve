import { memory } from "../pkg/wasm_game_of_life_bg";

init_panic_hook();

/*
import { Universe } from "../pkg";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.width = width;
canvas.height = height;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
  universe.tick();
  renderCells();
  requestAnimationFrame(renderLoop);
};

const renderCells = () => {
  const cellsPtr = universe.cells();
  const imageData = new ImageData(new Uint8ClampedArray(memory.buffer, cellsPtr, width * height * 4), width, height);
  ctx.putImageData(imageData, 0, 0);
  console.log(imageData.data);
}

renderCells();
requestAnimationFrame(renderLoop);
*/

import { ElectricField, init_panic_hook } from "../pkg";

const electricField = ElectricField.new();
const width = electricField.width();
const height = electricField.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.width = width;
canvas.height = height;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
  electricField.render();
  renderCells();
  requestAnimationFrame(renderLoop);
};

const renderCells = () => {
  const cellsPtr = electricField.electric_field_render();
  const imageData = new ImageData(new Uint8ClampedArray(memory.buffer, cellsPtr, width * height * 4), width, height);
  ctx.putImageData(imageData, 0, 0);
  console.log(imageData.data);
}

renderCells();
requestAnimationFrame(renderLoop);
