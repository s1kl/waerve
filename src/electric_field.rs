use wasm_bindgen::prelude::*;
use super::charge::Charge;

const W_CONST: usize = 513;
const H_CONST: usize = 513;
const K_CONST: f64 = 1000000000.0;
const COLOR_CONST: f64 = 10000.0;
const COLOR_MAX: u8 = 255;

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
    charge_nummber: usize,
}

impl ElectricField{
    fn get_index(&self, y: usize, x: usize) -> usize {
        (y * self.width + x) as usize
    }

    fn get_index_i(&self, y: isize, x: isize) -> usize {
        (y * self.width as isize + x).abs() as usize
    }

    fn get_index_double_i(&self, y: isize, x: isize) -> usize {
        (y * (self.width * 2 + 1) as isize + x).abs() as usize
    }

    fn compression_f64_u8(&self, n: f64) -> u8 {
        if (n / COLOR_CONST) as u8 >= COLOR_MAX{
            return COLOR_MAX as u8;
        }
        return (n / COLOR_CONST) as u8;
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
        if qnum == 0 {
            for j in 0..h {
                for i in 0..w {
                    let idx = self.get_index(j, i);
                    next_x[idx] = 0.0;
                    next_y[idx] = 0.0;
                }
            }
        }
        for k in 0..qnum {
            for j in 0..h {
                for i in 0..w {
                    let idx = self.get_index(j, i);
                    let idx_double = self.get_index_double_i(j as isize + hp2 - self.charge[k].y, i as isize + wp2 - self.charge[k].x);
                    if k == 0{
                        next_x[idx] = self.charge[0].q * efx[idx_double];
                        next_y[idx] = self.charge[0].q * efy[idx_double];
                    }else{
                        next_x[idx] += self.charge[k].q * efx[idx_double];
                        next_y[idx] += self.charge[k].q * efy[idx_double];
                        
                    }
                }
            }
        }
        self.electric_field_x = next_x;
        self.electric_field_y = next_y;
    }

    pub fn calc_accuration(&mut self) {
        let qnum = self.charge_nummber;
        let hp2: isize = self.height as isize / 2;
        let wp2: isize = self.width as isize / 2;
        for k in 0..qnum {
            let idx_next = self.get_index_i(hp2 + self.charge[k].y, wp2 + self.charge[k].x);
            self.charge[k].ay = self.charge[k].q*self.electric_field_y[idx_next];
            self.charge[k].ax = self.charge[k].q*self.electric_field_x[idx_next];

        }
    }

    pub fn calc_v_p(&mut self) {
        let qnum = self.charge_nummber;
        for k in 0..qnum {
            self.charge[k].calc_v_p_charge();
        }
    }

    pub fn colision(&mut self) {
        let qnum = self.charge_nummber;
        for k in 1..qnum {
            for l in 0..k {
                let cix = self.charge[k].x;
                let ciy = self.charge[k].y;
                let cjx = self.charge[l].x;
                let cjy = self.charge[l].y;
                let cixf = cix as f64;
                let ciyf = ciy as f64;
                let cjxf = cjx as f64;
                let cjyf = cjy as f64;
                let distance = (cixf-cjxf)*(cixf-cjxf)+(ciyf-cjyf)*(ciyf-cjyf);
                let ri = self.charge[k].qs;
                let rj = self.charge[l].qs;
                let radius = (ri + rj) * (ri + rj);
                if distance<=radius {
                    let rs = (ri + rj) as isize;
                    let e = 0.005;
                    let vxi = self.charge[k].vx * e;
                    let vyi = self.charge[k].vy * e;
                    let vxj = self.charge[l].vx * e;
                    let vyj = self.charge[l].vy * e;
                    let wp2 = self.width as isize / 2;
                    let hp2 = self.height as isize / 2;
                    self.charge[k].fix_v(vxj, vyj);
                    self.charge[l].fix_v(vxi, vyi);
                    let mut cixn;
                    let mut cjxn;
                    let mut ciyn;
                    let mut cjyn;
                    if cix <= cjx {
                        cixn = cix - rs;
                        cjxn = cjx + rs;
                        if cixn < -wp2 {
                            cixn = -wp2;
                            cjxn = -wp2+2*rs;
                        }
                        if cjxn > wp2 {
                            cjxn = wp2;
                            cixn = wp2-2*rs;
                        }
                    } else {
                        cixn = cix + rs;
                        cjxn = cjx - rs;
                        if cixn > wp2 {
                            cixn = wp2;
                            cjxn = wp2-2*rs;
                        }
                        if cjxn < -wp2 {
                            cjxn = -wp2;
                            cixn = -wp2+2*rs;
                        }
                    }
                    if ciy <= cjy {
                        ciyn = ciy - rs;
                        cjyn = cjy + rs;
                        if ciyn < -hp2 {
                            ciyn = -hp2;
                            cjyn = -hp2+2*rs;
                        }
                        if cjyn > hp2 {
                            cjyn = hp2;
                            ciyn = hp2-2*rs;
                        }
                    } else {
                        ciyn = ciy + rs;
                        cjyn = cjy - rs;
                        if ciyn > hp2 {
                            ciyn = hp2;
                            cjyn = hp2-2*rs;
                        }
                        if cjyn < -hp2 {
                            cjyn = -hp2;
                            ciyn = -hp2+2*rs;
                        }
                    }
                    self.charge[k].fix_p(cixn, ciyn);
                    self.charge[l].fix_p(cjxn, cjyn);
                }
            } 
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
        let efx = &self.electric_field_r;
        for j in 0..h {
            for i in 0..w {
                let idx = self.get_index(j, i);
                let idx4 = 4 * idx;
                let efx_idx = self.compression_f64_u8(efx[idx]);
                next[idx4] = efx_idx;
                next[idx4+1] = 0;
                next[idx4+2] = efx_idx;
                
            }
        }
        self.electric_field_render = next;
    }
}

#[wasm_bindgen]
impl ElectricField{
    pub fn new() -> ElectricField{
        let width: usize = W_CONST;
        let height: usize = H_CONST;
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
                let e_norm = K_CONST / r_three;
                let e_y = e_norm * y;
                let e_x = e_norm * x;
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
        charge.push(Charge::new(1.0, width as isize * 1 / 3 , height as isize / 3, width, height));
        charge.push(Charge::new(-1.0, width as isize * 2 / 3 , height as isize * 2 / 3, width, height));
        charge.push(Charge::new(1.0, width as isize *  3 / 4 , height as isize * 1 / 4, width, height));
        let qnum = charge.len() as usize;
    
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

    pub fn cqn(&self) -> usize {
        self.charge_nummber
    }

    pub fn get_pointer(&self) -> *const u8 {
        self.electric_field_render.as_ptr()
    }

    pub fn install_charge(&mut self, q: f64, x: isize, y: isize) {
        self.charge.push(Charge::new(q, x, y, W_CONST, H_CONST));
        self.charge_nummber+=1;
    }

    pub fn remove_charge(&mut self, x: f64, y: f64) {
        let qnum = self.charge_nummber;
        let xi = x - self.width as f64 / 2.0;
        let yi = y - self.height as f64 / 2.0;
        for k in 0..qnum {
            let ri = self.charge[k].qs;
            // let ri = 40.0;
            let rxp = self.charge[k].x as f64 + ri;
            let ryp = self.charge[k].y as f64 + ri;
            let rxm = self.charge[k].x as f64 - ri;
            let rym = self.charge[k].y as f64 - ri;
            if xi <= rxp && xi >= rxm && yi <= ryp && yi >= rym {
                self.charge.remove(k);
                self.charge_nummber -= 1;
                break;
            }
        }
    }

    pub fn control_charge(&mut self, x: f64, y: f64) {
        let qnum = self.charge_nummber;
        let xi = x - self.width as f64 / 2.0;
        let yi = y - self.height as f64 / 2.0;
        for k in 0..qnum {
            let ri = self.charge[k].qs;
            let rxp = self.charge[k].x as f64 + ri;
            let ryp = self.charge[k].y as f64 + ri;
            let rxm = self.charge[k].x as f64 - ri;
            let rym = self.charge[k].y as f64 - ri;
            if xi <= rxp && xi >= rxm && yi <= ryp && yi >= rym {
                self.charge[k].control_move();
                break;
            }
        }
    }

    pub fn not_control_charge(&mut self) {
        let qnum = self.charge_nummber;
        for k in 0..qnum {
            if self.charge[k].cm == true {
                self.charge[k].control_move();
                break;
            }
        }
    }

    pub fn mouse_charge(&mut self, x:isize, y:isize) {
        let qnum = self.charge_nummber;
        let xi = x - self.width as isize / 2;
        let yi = y - self.height as isize / 2;
        for k in 0..qnum {
            if self.charge[k].cm == true {
                self.charge[k].mouse(xi, yi)
            }
        }
    }

    pub fn test2(&self)->bool{if self.charge_nummber!=0 { self.charge[0].test() } else {false}}
}