use wasm_bindgen::prelude::*;
// Chargeの配列はwasmとの兼ね合いで無理
// pub x: Vec<usize>みたいなことも出来ない
// electric_fieldの中にchargeを入れてみようかな

#[wasm_bindgen]
pub struct Charge {
    pub x: usize,
    pub y: usize,
    vx: f64,
    vy: f64,
    ax: f64,
    ay: f64,
    pub q: f64,
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