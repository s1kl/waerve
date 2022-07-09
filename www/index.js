import { memory } from "../pkg/wasm_game_of_life_bg";
import { ElectricField, init_panic_hook, main } from "../pkg";
import { get_mouse_coordinate } from "./input";

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
let radio = 'install';

const render = () => {
  main(electricField);
  ctx.putImageData(new ImageData(new Uint8ClampedArray(memory.buffer, electricField.get_pointer(), width * height * 4), width, height), 0, 0);
}

const renderLoop = () => {
  render();
  console.log(electricField.test2())
  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);

function radio_situation() {
  for (let i = 0; i < 3; i++){
    if (input_radio.item(i).checked){
      radio = input_radio.item(i).value;
      break;
    }
  }
}

function input(e) {
  const xy = get_mouse_coordinate(e, cwidth,  cheight, width, height)
  if (radio == 'install') {
    electricField.install_charge(1.0, xy[0], xy[1], width, height)
  } else if (radio == 'remove') {
    electricField.remove_charge(xy[0], xy[1], width, height)
  } else if (radio == 'fix') {
    electricField.can_move_charge(xy[0], xy[1])
  }
}

function mouseUp(e) {
  if (radio == 'fix') {
    electricField.cannot_move_charge()
  }
}

input_radio.forEach(function(e) { e.addEventListener('click', radio_situation) });
canvas.addEventListener('mousedown', input);
document.addEventListener('mouseup', mouseUp);