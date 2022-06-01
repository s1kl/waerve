use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ElectricField{
    width: u16,
    height: u16,
    electric_field_template_x: Vec<f64>,
    electric_field_template_y: Vec<f64>,
    electric_field_render: Vec<u8>,
}

impl ElectricField{
    fn get_index(&self, y: u16, x: u16) -> usize {
        (y * self.width + x) as usize
    }

    fn compression_f64_u8(&self, n: f64) -> u8 {
        if n > 89875520.0 {
            return 255;
        }
        else {
            return 0;
        }
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
                let idx = self.get_index(j, i);
                let idx4 = 4 * idx;
                let efx_idx = self.compression_f64_u8(efx[idx]);
                next[idx4] = efx_idx;
                next[idx4+1] = efx_idx;
                next[idx4+2] = efx_idx;
                
            }
        }
        self.electric_field_render = next;
    }

    pub fn new() -> ElectricField{
        let width = 513 as u16;
        let height = 513 as u16;
        let width_double = width * 2 + 1;
        let height_double = height * 2 + 1;
        let mut electric_field_template_x: Vec<f64> = Vec::new();
        let mut electric_field_template_y: Vec<f64> = Vec::new();
        for j in 0..height_double{
            for i in 0..width_double{
                let y = j - height;
                let x = i - width;
                let r = ((x*x + y*y) as f64).sqrt();
                let r_three = r * r * r as f64;
                let e_norm = (8987552000.0 as f64) / r_three;
                let e_x = e_norm * x as f64;
                let e_y = e_norm * y as f64;
                electric_field_template_x.push(e_x as f64);
                electric_field_template_y.push(e_y as f64);
            }
        }
        let n:usize = (width * height * 4) as usize;
        let electric_field_render: Vec<u8> = vec![255; n];
    
        ElectricField{
            width,
            height,
            electric_field_template_x,
            electric_field_template_y,
            electric_field_render,
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn electric_field_render(&self) -> *const u8 {
        self.electric_field_render.as_ptr()
    }
}