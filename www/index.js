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
let control = false;
const range = document.getElementById('range');
const rangev = document.getElementById('rangev');
let rangeq = 10;

const render = () => {
  main(electricField);
  ctx.putImageData(new ImageData(new Uint8ClampedArray(memory.buffer, electricField.get_pointer(), width * height * 4), width, height), 0, 0);
}

function animation(){
  let fps = 0;
  let frameCount = 0;
  let startTime;
  let endTime;
  startTime = new Date().getTime();
  function animationLoop(){
      frameCount ++;
      endTime = new Date().getTime();
      if(endTime - startTime >= 1000){
          fps = frameCount;
          frameCount = 0;
          startTime = new Date().getTime();
      }
      render();
      requestAnimationFrame(animationLoop);
      let animationFPS = document.getElementById("fps");
      animationFPS.innerHTML = fps;
  }
  animationLoop();
}
animation();

function radio_situation() {
  for (let i = 0; i < 4; i++){
    if (input_radio.item(i).checked){
      radio = input_radio.item(i).value;
      break;
    }
  }
}

function input(e) {
  const xy = get_mouse_coordinate(e, cwidth,  cheight, width, height)
  if (radio == 'install') {
    electricField.install_charge(rangeq/10, xy[0], xy[1], width, height)
  } else if (radio == 'remove') {
    electricField.remove_charge(xy[0], xy[1], width, height)
  } else if (radio == 'control') {
    control = true;
    electricField.control_charge(xy[0], xy[1])
  } else if (radio == 'fix') {
    electricField.fix_charge(xy[0], xy[1])
  }
}

function mouseMove(e) {
  if (radio == 'control' && control) {
    const xy = get_mouse_coordinate(e, cwidth,  cheight, width, height)
    electricField.mouse_charge(xy[0], xy[1])
  }
}

function mouseUp(e) {
  if (radio == 'control') {
    control = false;
    electricField.not_control_charge()
  }
}

function get_range(e) {
  rangeq = Number(range.value)
  if (rangeq <= 0) {
    rangeq-=1;
  }
  rangev.innerHTML = rangeq;
}

input_radio.forEach(function(e) { e.addEventListener('click', radio_situation) });
canvas.addEventListener('mousedown', input);
canvas.addEventListener('mousemove', mouseMove);//touch系にも対応しようね
document.addEventListener('mouseup', mouseUp);
range.addEventListener('input', get_range);

radio_situation();
get_range();