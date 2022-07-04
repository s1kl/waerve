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

    fn get_index_i(&self, y: isize, x: isize) -> usize {
        (y * self.width as isize + x).abs() as usize
    }

    fn get_index_double(&self, y: usize, x: usize) -> usize {
        (y * (self.width * 2 + 1) + x) as usize
    }

    fn get_index_double_i(&self, y: isize, x: isize) -> usize {
        (y * (self.width * 2 + 1) as isize + x).abs() as usize
    }

    fn compression_f64_u8(&self, n: f64) -> u8 {
        if (n / 10000.0) as u8 > 254{
            return 255 as u8;
        }
        return (n / 10000.0) as u8;
    }

    pub fn surpose_electric_field(&mut self) {
        let mut next_x = self.electric_field_x.clone();
        let mut next_y = self.electric_field_y.clone();
        let h = self.height;
        let w = self.width;
        let hp2: isize = self.height as isize / 2;
        let wp2: isize = self.width as isize / 2;
        let efx = &self.electric_field_template_x;
        let efy = &self.electric_field_template_y;
        let qnum = self.charge_nummber;
        for k in 0..qnum {
            let l = k as usize;
            for j in 0..h {
                for i in 0..w {
                    let idx = self.get_index(j, i);
                    let idx_double = self.get_index_double_i(j as isize + hp2 - self.charge[l].y, i as isize + wp2 - self.charge[l].x);
                    if k == 0{
                        next_x[idx] = self.charge[0].q * efx[idx_double];
                        next_y[idx] = self.charge[0].q * efy[idx_double];
                    }else{
                        next_x[idx] += self.charge[l].q * efx[idx_double];
                        next_y[idx] += self.charge[l].q * efy[idx_double];
                        
                    }
                }
            }
        }
        self.electric_field_x = next_x;
        self.electric_field_y = next_y;
    }

    pub fn calc_accuration(&mut self) {
        let qnum = self.charge_nummber;
        let h = self.height;
        let w = self.width;
        let hp2: isize = self.height as isize / 2;
        let wp2: isize = self.width as isize / 2;
        for k in 0..qnum {
            let l = k as usize;
            // 座標の取得をなんとかする
            let idx_next = self.get_index_i(hp2 + self.charge[l].y, wp2 + self.charge[l].x);
            self.charge[l].ay = self.electric_field_y[idx_next];
            self.charge[l].ax = self.electric_field_x[idx_next];
            // self.charge[l].ay = 4.0;
            // self.charge[l].ax = 4.0;

        }
    }

    pub fn calc_velocity(&mut self) {
        let qnum = self.charge_nummber;
        for k in 0..qnum {
            let l = k as usize;
            self.charge[l].calc_velocity_charge();
        }
    }

    pub fn calc_position(&mut self) {
        let qnum = self.charge_nummber;
        for k in 0..qnum {
            let l = k as usize;
            self.charge[l].calc_position_charge();
        }
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
        let efx = &self.electric_field_x;
        for j in 0..h {
            for i in 0..w {
                let idx = self.get_index(j, i);
                let idx4 = 4 * idx;
                let efx_idx = self.compression_f64_u8(efx[idx]);
                let efx_idxm = self.compression_f64_u8(-efx[idx]);
                next[idx4] = efx_idx;
                next[idx4+1] = 0;
                next[idx4+2] = efx_idxm;
                
            }
        }
        
        let qnum = self.charge_nummber;
        let h = self.height;
        let w = self.width;
        let hp2: isize = self.height as isize / 2;
        let wp2: isize = self.width as isize / 2;
        for k in 0..qnum {
            let l = k as usize;
            let idx = self.get_index_i(hp2 + self.charge[l].y, wp2 + self.charge[l].x);
            let idx4 = 4 * idx;
            next[idx4+1] = 255;
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
                let y: f64 = height as f64 - j as f64 - 1.0;
                let x: f64 = width as f64 - i as f64 - 1.0;
                let r = ((x*x + y*y)).sqrt();
                let r_three = r * r * r;
                let e_norm = 10000000000.0 / r_three;
                let e_y = e_norm * y;
                let e_x = e_norm * x;
                // electric_field_template_x.push(1271024439182.8);
                // electric_field_template_y.push(1271024439182.8);
                electric_field_template_x.push(e_x);
                electric_field_template_y.push(e_y);
            }
        }
        let e_center = (height-1) * (width * 2 + 1) + (width-1);
        electric_field_template_y[e_center] = 0.0;
        electric_field_template_x[e_center] = 0.0;
        let n:usize = width * height;
        let n_four:usize = 4 * n;
        let electric_field_render: Vec<u8> = vec![255; n_four];
        let electric_field_x: Vec<f64> = vec![0.0; n];
        let electric_field_y: Vec<f64> = vec![0.0; n];
        let electric_field_r: Vec<f64> = vec![0.0; n];
        let mut charge: Vec<Charge> = Vec::new();
        // Chargeのとこでx、yからh/2、w/2を引いて座標変換してる処理のせいでヤバい
        charge.push(Charge::new(1.0, 280, 330));
        charge.push(Charge::new(1.0, 250, 130));
        // charge.push(Charge::new(1.0, 50, 430));
        // charge.push(Charge::new(1.0, 450, 30));
        let qnum = charge.len() as u8;
    
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
            charge_nummber: qnum,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn charge_ax0(&self) -> usize {
        self.get_index_i(self.height() as isize / 2 + self.charge[0].y, self.width as isize / 2 + self.charge[0].x)
        // self.charge[0].ax()
    }

    pub fn charge_ax1(&self) -> usize {
        self.get_index_i(self.height() as isize / 2 + self.charge[1].y, self.width as isize / 2 + self.charge[1].x)
        // self.charge[1].ax()
    }

    pub fn charge_ax3(&self) -> f64 {
        self.charge[0].ax()
    }

    pub fn charge_ax4(&self) -> f64 {
        self.charge[1].ax()
    }

    pub fn tmp(&self) -> f64 {
        // 電荷の位置において電界がNaNを返す
        // NaNに和算ないし乗算してもNanから不変
        self.electric_field_template_x[self.get_index_double(self.height-1, self.width-1)]
    }

    pub fn get_pointer(&self) -> *const u8 {
        self.electric_field_render.as_ptr()
    }
}