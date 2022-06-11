// Chargeの配列はwasmとの兼ね合いで無理
// pub x: Vec<usize>みたいなことも出来ない
// electric_fieldの中にchargeを入れてみようかな

// 次はadd chargeを作ろうかな
// charge_nummberを増やしつつ

pub struct Charge {
    pub q: f64,
    pub x: usize,
    pub y: usize,
    vx: f64,
    vy: f64,
    pub ax: f64,
    pub ay: f64,
}

impl Charge{
    pub fn new(qi: f64, xi: usize, yi: usize) -> Charge{
        let q: f64 = qi;
        let x: usize = xi - 256;
        let y: usize = yi - 256;
        Charge{
            q,
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            ax: 0.0,
            ay: 0.0,
        }
    }

    pub fn calc_velocity_charge(&mut self){
        let next_x: f64 = self.vx + self.ax * 0.000001;
        let next_y: f64 = self.vy + self.ay * 0.000001;
        self.vx = next_x;
        self.vy = next_y;
    }

    pub fn calc_position_charge(&mut self){
        let next_x: f64 = self.x as f64 + self.vx * 0.000001;
        let next_y: f64 = self.y as f64 + self.vy * 0.000001;
        self.x = next_x as usize;
        self.y = next_y as usize;
    }
}