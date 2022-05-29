import { Universe } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

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
	// console.log(ctx.getImageData(0,0,canvas.width,canvas.height))

  requestAnimationFrame(renderLoop);
};

const renderCells = () => {
  const cellsPtr = universe.cells();
  const imageData = new ImageData(new Uint8ClampedArray(memory.buffer, cellsPtr, width * height * 4), width, height);
  ctx.putImageData(imageData, 0, 0);
  console.log(imageData.data[0],imageData.data[2200]);
}


renderCells();
requestAnimationFrame(renderLoop);