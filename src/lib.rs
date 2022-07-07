mod electric_field;
mod charge;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn main(electric_field: &mut electric_field::ElectricField) {
    electric_field.surpose_electric_field();
    electric_field.polar_conversion();
    electric_field.render();
    electric_field.calc_accuration();
    electric_field.calc_v_p();
    electric_field.colision();
}

// 次はマウスで電荷を置けるようにする