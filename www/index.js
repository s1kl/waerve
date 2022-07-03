import { memory } from "../pkg/wasm_game_of_life_bg";
import { ElectricField, Charge, init_panic_hook, main } from "../pkg";

init_panic_hook();

const electricField = ElectricField.new();
const width = electricField.width();
const height = electricField.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.width = width;
canvas.height = height;

const ctx = canvas.getContext('2d');

const render = () => {
  // electricField.render();
  main(electricField);
  const pointer = electricField.get_pointer();
  const imageData = new ImageData(new Uint8ClampedArray(memory.buffer, pointer, width * height * 4), width, height);
  ctx.putImageData(imageData, 0, 0);
  console.log(electricField.charge_ax0());
  console.log(electricField.charge_ax1());
}

const renderLoop = () => {
  render();
  requestAnimationFrame(renderLoop);
};

// render();
requestAnimationFrame(renderLoop);
