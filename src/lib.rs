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
    electric_field.calc_velocity();
    electric_field.calc_position();
}

/*
置く
↓
電界映す←--------
↓               |
力(加速度)計算   |
↓               |	
速度計算         |
↓               |
位置計算---------
*/