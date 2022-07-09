import { memory } from "../pkg/wasm_game_of_life_bg";
import { ElectricField, init_panic_hook, main } from "../pkg";
import { install, remove } from "./input";

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

const input_radio = document.getElementsByName("input_radio");

const render = () => {
  main(electricField);
  ctx.putImageData(new ImageData(new Uint8ClampedArray(memory.buffer, electricField.get_pointer(), width * height * 4), width, height), 0, 0);
}

const renderLoop = () => {
  render();
  console.log(radio);
  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);

let radio = 'install';

function radio_situation() {
  for (let i = 0; i < 2; i++){
    if (input_radio.item(i).checked){
      radio = input_radio.item(i).value;
      break;
    }
  }
}

input_radio.forEach(function(e) { e.addEventListener('click', radio_situation) });
//canvas.addEventListener('click', {canvasClientWidth: cwidth, canvasClientHeight: cheight, canvasWidth: width, canvasHeight: height, electricField: electricField, handleEvent: install});
canvas.addEventListener('click', {canvasClientWidth: cwidth, canvasClientHeight: cheight, canvasWidth: width, canvasHeight: height, electricField: electricField, handleEvent: remove});