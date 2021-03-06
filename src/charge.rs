pub struct Charge {
    pub q: f64,
    pub qs: f64,
    pub x: isize,
    pub y: isize,
    pub vx: f64,
    pub vy: f64,
    pub ax: f64,
    pub ay: f64,
    pub cm: bool,
    pub fix: bool,
    w: usize,
    h: usize,
}

impl Charge{
    pub fn new(qi: f64, xi: isize, yi: isize, w: usize, h: usize) -> Charge{
        let q: f64 = qi;
        let qs: f64 = 20.0 * qi.abs().sqrt();
        let x: isize = xi - w as isize / 2;
        let y: isize = yi - h as isize / 2;
        Charge{
            q,
            qs,
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            ax: 0.0,
            ay: 0.0,
            cm: false,
            fix: false,
            w: w,
            h: h,
        }
    }

    pub fn calc_v_p_charge(&mut self) {
        let dt = 0.005;
        let next_vx: f64 = self.vx - self.ax * dt;
        let next_vy: f64 = self.vy - self.ay * dt;
        let next_rx: f64 = self.x as f64 + next_vx * dt;
        let next_ry: f64 = self.y as f64 + next_vy * dt;
        let wp2: isize = (self.w / 2) as isize;
        let wp2f: f64 = wp2 as f64;
        let hp2: isize = (self.h / 2) as isize;
        let hp2f: f64 = hp2 as f64;
        if next_rx >= -wp2f && next_rx <= wp2f {
            self.vx = next_vx;
            self.x = next_rx.round() as isize;
        }else{
            self.vx = -next_vx * dt;
            if next_rx < 0.0 { self.x = -wp2} else { self.x = wp2 }
        }
        if next_ry >= -hp2f && next_ry <= hp2f {
            self.vy = next_vy;
            self.y = next_ry.round() as isize;
        }else{
            self.vy = -next_vy * dt;
            if next_ry < 0.0 { self.y = -hp2} else { self.y = hp2 }
        }
    }

    pub fn improve_p(&mut self, x: isize, y: isize) {
        self.x = x;
        self.y = y;
    }

    pub fn improve_v(&mut self, vx: f64, vy: f64) {
        self.vx = vx;
        self.vy = vy;
    }

    pub fn control_move(&mut self) {
        self.cm = !self.cm;
    }
    
    pub fn mouse(&mut self, x:isize, y:isize) {
        self.vx = 0.0;
        self.vy = 0.0;
        self.x = x;
        self.y = y;
    }

    pub fn fix(&mut self) {
        self.fix = !self.fix;
    }

    pub fn test(&self) -> bool {self.fix}
}