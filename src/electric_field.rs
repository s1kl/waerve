use wasm_bindgen::prelude::*;
use super::charge::Charge;

#[wasm_bindgen]
pub struct ElectricField{
    width: usize,
    height: usize,
    electric_field_template_x: Vec<f64>,
    electric_field_template_y: Vec<f64>,
    electric_field_x: Vec<f64>,
    electric_field_y: Vec<f64>,
    electric_field_r: Vec<f64>,
    electric_field_render: Vec<u8>,
    charge: Vec<Charge>,
    charge_nummber: u8,
}

impl ElectricField{
    fn get_index(&self, y: usize, x: usize) -> usize {
        (y * self.width + x) as usize
    }

    fn get_index_double(&self, y: usize, x: usize) -> usize {
        (y * (self.width * 2 + 1) + x) as usize
    }

    fn compression_f64_u8(&self, n: f64) -> u8 {
        if (n / 90.0) as u8 > 254{
            return 255 as u8;
        }
        return (n / 90.0) as u8;
    }

    pub fn surpose_electric_field(&mut self) {
        let mut next_x = self.electric_field_x.clone();
        let mut next_y = self.electric_field_y.clone();
        let h = self.height;
        let w = self.width;
        let efx = &self.electric_field_template_x;
        let efy = &self.electric_field_template_y;
        let qnum = self.charge_nummber;
        for k in 0..qnum {
            for j in 0..h {
                for i in 0..w {
                    let l = k as usize;
                    let idx = self.get_index(j, i);
                    let idx_double = self.get_index_double(j + h/2 + self.charge[l].y, i + w/2 - self.charge[l].x);
                    if k == 0{
                        next_x[idx] = self.charge[0].q * efx[idx_double];
                        next_y[idx] = self.charge[0].q * efy[idx_double];
                    }else{
                        // next_x[idx] += self.charge[l].q * efx[idx_double];
                        // next_y[idx] += self.charge[l].q * efy[idx_double];

                    }
                }
            }
        }
        self.electric_field_x = next_x;
        self.electric_field_y = next_y;
    }

    pub fn polar_conversion(&mut self) {
        let mut next_r = self.electric_field_r.clone();
        let width = self.width;
        let height = self.height;
        let efx = &self.electric_field_x;
        let efy = &self.electric_field_y;
        for j in 0..height{
            for i in 0..width{
                let idx = self.get_index(j, i);
                next_r[idx] = (efx[idx]*efx[idx] + efy[idx]*efy[idx]).sqrt();
            }
        }
        self.electric_field_r = next_r;
    }

    pub fn render(&mut self) {
        let mut next = self.electric_field_render.clone();
        let h = self.height;
        let w = self.width;
        let efx = &self.electric_field_r;
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
}

#[wasm_bindgen]
impl ElectricField{
    pub fn new() -> ElectricField{
        let width: usize = 513;
        let height: usize = 513;
        let width_double = width * 2 + 1;
        let height_double = height * 2 + 1;
        let mut electric_field_template_x: Vec<f64> = Vec::new();
        let mut electric_field_template_y: Vec<f64> = Vec::new();
        for j in 0..height_double{
            for i in 0..width_double{
                let y = if j<height {j - height - 1} else {height - j - 1};
                let x = if i<width {i - width - 1} else {width - i - 1};
                let r: f64 = ((x*x + y*y) as f64).sqrt();
                let r_three = r * r * r;
                let e_norm = 9.0 / r_three;
                let e_y = if j<height {e_norm * y as f64} else {-e_norm * y as f64};
                let e_x = if i<width {-e_norm * x as f64} else {e_norm * x as f64};
                // electric_field_template_x.push(1271024439182.8);
                // electric_field_template_y.push(1271024439182.8);
                electric_field_template_x.push(e_x);
                electric_field_template_y.push(e_y);
            }
        }
        let n:usize = width * height;
        let n_four:usize = 4 * n;
        let electric_field_render: Vec<u8> = vec![255; n_four];
        let electric_field_x: Vec<f64> = vec![0.0; n];
        let electric_field_y: Vec<f64> = vec![0.0; n];
        let electric_field_r: Vec<f64> = vec![0.0; n];
        let mut charge: Vec<Charge> = Vec::new();
        let c1 = Charge::new(1.0, 200, 256);
        charge.push(c1);
        let c2 = Charge::new(1.0, 300, 256);
        charge.push(c2);
    
        ElectricField{
            width,
            height,
            electric_field_template_x,
            electric_field_template_y,
            electric_field_x,
            electric_field_y,
            electric_field_r,
            electric_field_render,
            charge,
            charge_nummber: 2,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_pointer(&self) -> *const u8 {
        self.electric_field_render.as_ptr()
    }
}