use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ElectricField{
    width: usize,
    height: usize,
    electric_field_template_x: Vec<f64>,
    electric_field_template_y: Vec<f64>,
    electric_field_render: Vec<u8>,
}

impl ElectricField{
    fn get_index(&self, y: usize, x: usize) -> usize {
        (y * self.width + x) as usize
    }

    fn get_index_double(&self, y: usize, x: usize) -> usize {
        (y * (self.width * 2 + 1) + x) as usize
    }

    fn compression_f64_u8(&self, n: f64) -> u8 {
        //if n < 0.0 {
        //    return (n / 898755200.0) as u8;
        //} else {
            return (n / 8987552000.0) as u8;
        //}
    }
}

#[wasm_bindgen]
impl ElectricField{
    pub fn render(&mut self) {
        let mut next = self.electric_field_render.clone();
        let h = self.height;
        let w = self.width;
        let efx = &self.electric_field_template_x;
        for j in 0..h {
            for i in 0..w {
                let idx = self.get_index_double(j + 128, i + 128);
                let idx4 = 4 * self.get_index(j, i);
                let efx_idx = self.compression_f64_u8(efx[idx]);
                next[idx4] = efx_idx;
                next[idx4+1] = efx_idx;
                next[idx4+2] = efx_idx;
                
            }
        }
        self.electric_field_render = next;
    }

    pub fn new() -> ElectricField{
        let width: usize = 513;
        let height: usize = 513;
        let width_double = width * 2 + 1;
        let height_double = height * 2 + 1;
        let mut electric_field_template_x: Vec<f64> = Vec::new();
        let mut electric_field_template_y: Vec<f64> = Vec::new();
        for j in 0..height_double{
            for i in 0..width_double{
                let y = j - height;
                let x = i - width;
                let r: f64 = ((x*x + y*y) as f64).sqrt();
                let r_three = r * r * r;
                let e_norm = 8987552000.0 / r_three;
                let e_x = e_norm * x as f64;
                let e_y = e_norm * y as f64;
                electric_field_template_x.push(e_x);
                electric_field_template_y.push(e_y);
            }
        }
        let n:usize = width * height * 4;
        let electric_field_render: Vec<u8> = vec![255; n];
    
        ElectricField{
            width,
            height,
            electric_field_template_x,
            electric_field_template_y,
            electric_field_render,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn electric_field_render(&self) -> *const u8 {
        self.electric_field_render.as_ptr()
    }
}