import { Universe, Cell } from "wasm-game-of-life";
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

  drawCells();
	// console.log(ctx.getImageData(0,0,canvas.width,canvas.height))

  requestAnimationFrame(renderLoop);
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  const imageData = ctx.getImageData(0,0,width,height);
  const img = imageData.data;
  for (let i = 0; i < width*height*4; i++) {
    img[i]=cells[i/4 | 0]*255
  } 
	ctx.putImageData(imageData,0,0)
};


drawCells();
requestAnimationFrame(renderLoop);