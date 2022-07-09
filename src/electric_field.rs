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
    charge_nummber: u8,
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
        let hp2: isize = self.height as isize / 2;
        let wp2: isize = self.width as isize / 2;
        for k in 0..qnum {
            let l = k as usize;
            let idx_next = self.get_index_i(hp2 + self.charge[l].y, wp2 + self.charge[l].x);
            self.charge[l].ay = self.charge[l].q*self.electric_field_y[idx_next];
            self.charge[l].ax = self.charge[l].q*self.electric_field_x[idx_next];

        }
    }

    pub fn calc_v_p(&mut self) {
        let qnum = self.charge_nummber;
        for k in 0..qnum {
            let l = k as usize;
            self.charge[l].calc_v_p_charge();
        }
    }

    pub fn colision(&mut self){
        let qnum = self.charge_nummber;
        for k in 1..qnum {
            for l in 0..k {
                let i = k as usize;
                let j = l as usize;
                let cix = self.charge[i].x;
                let ciy = self.charge[i].y;
                let cjx = self.charge[j].x;
                let cjy = self.charge[j].y;
                let cixf = cix as f64;
                let ciyf = ciy as f64;
                let cjxf = cjx as f64;
                let cjyf = cjy as f64;
                let distance = (cixf-cjxf)*(cixf-cjxf)+(ciyf-cjyf)*(ciyf-cjyf);
                let ri = 20.0 * self.charge[i].q.abs().sqrt();// 馬鹿にならない回数のsqrt
                let rj = 20.0 * self.charge[j].q.abs().sqrt();// charge.pushの時点で計算してcharge構造体の中に入れといたほうが...
                let radius = (ri + rj) * (ri + rj);
                if distance<=radius {
                    let rs = (ri + rj) as isize;
                    let e = 0.005;
                    let vxi = self.charge[i].vx * e;
                    let vyi = self.charge[i].vy * e;
                    let vxj = self.charge[j].vx * e;
                    let vyj = self.charge[j].vy * e;
                    let wp2 = self.width as isize / 2;
                    let hp2 = self.height as isize / 2;
                    self.charge[i].fix_v(vxj, vyj);
                    self.charge[j].fix_v(vxi, vyi);
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
                    self.charge[i].fix_p(cixn, ciyn);
                    self.charge[j].fix_p(cjxn, cjyn);
                    // if cix <= cjx && ciy <= cjy {
                    //     self.charge[i].fix_p(cix - rs, ciy - rs);
                    //     self.charge[j].fix_p(cjx + rs, cjy + rs);
                    // } else if cix >= cjx && ciy <= cjy {
                    //     self.charge[i].fix_p(cix + rs, ciy - rs);
                    //     self.charge[j].fix_p(cjx - rs, cjy + rs);
                    // } else if cix < cjx && ciy >= cjy {
                    //     self.charge[i].fix_p(cix - rs, ciy + rs);
                    //     self.charge[j].fix_p(cjx + rs, cjy - rs);
                    // } else if cix >= cjx && ciy >= cjy {
                    //     self.charge[i].fix_p(cix + rs, ciy + rs);
                    //     self.charge[j].fix_p(cjx - rs, cjy - rs);
                    // }
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
        // for i in 0..self.charge.len() as usize {
        //     let idx = self.get_index_i(self.charge[i].y + h as isize/2, self.charge[i].x + w as isize/2 + (20.0 * self.charge[i].q.sqrt()) as isize);
        //     next[4 * idx + 1]=255;
        // }
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
        charge.push(Charge::new(1.0, width as isize * 2 / 3 , height as isize * 2 / 3, width, height));
        //charge.push(Charge::new(1.0, width as isize * 2 / 3 , height as isize / 3, width, height));
        // charge.push(Charge::new(1.0, width as isize , height as isize * 2 / 3, width, height));
        // charge.push(Charge::new(1.0, width as isize , height as isize * 1 / 3, width, height));
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

    pub fn cqn(&self) -> u8 {
        self.charge_nummber
    }

    pub fn get_pointer(&self) -> *const u8 {
        self.electric_field_render.as_ptr()
    }

    pub fn install_charge(&mut self, q: f64, x: isize, y: isize) {
        self.charge.push(Charge::new(q, x, y, W_CONST, H_CONST));
        self.charge_nummber+=1;
    }

    pub fn cx0(&self) -> isize { self.charge[0].x() }
    pub fn cy0(&self) -> isize { self.charge[0].y() }
    pub fn cvx0(&self) -> f64 { self.charge[0].vx() }
    pub fn cvy0(&self) -> f64 { self.charge[0].vy() }
    pub fn cax0(&self) -> f64 { self.charge[0].ax() }
    pub fn cay0(&self) -> f64 { self.charge[0].ay() }
    pub fn cx1(&self) -> isize { self.charge[1].x() }
    pub fn cy1(&self) -> isize { self.charge[1].y() }
    pub fn cvx1(&self) -> f64 { self.charge[1].vx() }
    pub fn cvy1(&self) -> f64 { self.charge[1].vy() }
    pub fn cax1(&self) -> f64 { self.charge[1].ax() }
    pub fn cay1(&self) -> f64 { self.charge[1].ay() }
}