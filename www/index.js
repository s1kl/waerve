import { memory } from "../pkg/wasm_game_of_life_bg";
import { ElectricField, init_panic_hook, main } from "../pkg";
import { install } from "./input";

init_panic_hook();

const electricField = ElectricField.new();
const width = electricField.width();
const height = electricField.height();

const canvas = document.getElementById("canvas");
canvas.width = width;
canvas.height = height;
const cwidth = canvas.clientWidth;
const cheight = canvas.clientHeight;
const ctx = canvas.getContext('2d');

const render = () => {
  main(electricField);
  ctx.putImageData(new ImageData(new Uint8ClampedArray(memory.buffer, electricField.get_pointer(), width * height * 4), width, height), 0, 0);
}

const renderLoop = () => {
  render();
  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);

canvas.addEventListener('click', {canvasClientWidth: cwidth, canvasClientHeight: cheight, canvasWidth: width, canvasHeight: height, electricField: electricField, handleEvent: install});