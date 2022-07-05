import { memory } from "../pkg/wasm_game_of_life_bg";
import { ElectricField, init_panic_hook, main } from "../pkg";

init_panic_hook();

const electricField = ElectricField.new();
const width = electricField.width();
const height = electricField.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.width = width;
canvas.height = height;

const ctx = canvas.getContext('2d');

const render = () => {
  main(electricField);
  const pointer = electricField.get_pointer();
  const imageData = new ImageData(new Uint8ClampedArray(memory.buffer, pointer, width * height * 4), width, height);
  ctx.putImageData(imageData, 0, 0);
}

const renderLoop = () => {
  render();
  // console.log("( x0 , y0 ) = (",electricField.cx0(),",", electricField.cy0(),")");
  // console.log("( vx0 , vy0 ) = (",electricField.cvx0(),",", electricField.cvy0(),")");
  // console.log("( ax0 , ay0 ) = (",electricField.cax0(),",", electricField.cay0(),")");
  // console.log("( x1 , y1 ) = (",electricField.cx1(),",", electricField.cy1(),")");
  // console.log("( vx1 , vy1 ) = (",electricField.cvx1(),",", electricField.cvy1(),")");
  // console.log("( ax1 , ay1 ) = (",electricField.cax1(),",", electricField.cay1(),")");
  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
