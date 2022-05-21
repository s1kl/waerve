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

  renderCells();
	// console.log(ctx.getImageData(0,0,canvas.width,canvas.height))

  requestAnimationFrame(renderLoop);
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8ClampedArray(memory.buffer, cellsPtr, width * height * 4);
  const imageData = ctx.getImageData(0,0,width,height);
  const img = imageData.data;
  for (let i = 0; i < width*height*4; i++) {
    img[i]=cells[i]
  } 
	ctx.putImageData(imageData,0,0)
  console.log(img, cells)
};

const renderCells = () => {
  const cellsPtr = universe.cells();
  const imageData = new ImageData(new Uint8ClampedArray(memory.buffer, cellsPtr, width * height * 4), width, height);
  ctx.putImageData(imageData, 0, 0);
}


renderCells();
requestAnimationFrame(renderLoop);