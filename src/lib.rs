mod electric_field;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn init() {
    let mut electric_field = electric_field::ElectricField::new();
    electric_field.surpose_electric_field();
}

#[wasm_bindgen]
pub fn main(electric_field: &mut electric_field::ElectricField) {
    electric_field.surpose_electric_field();
    electric_field.polar_conversion();
    electric_field.render();
}