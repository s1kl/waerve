// mod electric_field;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Charge{
    x: usize,
    y: usize,
    vx: f64,
    vy: f64,
    ax: f64,
    ay: f64,
    q: f64,
}


#[wasm_bindgen]
impl Charge{
    pub fn new() -> Charge{
        let x: usize = 0;
        let y: usize = 0;
        let q: f64 = 1.0;
        Charge{
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            ax: 0.0,
            ay: 0.0,
            q,
        }
    }
}