// Chargeの配列はwasmとの兼ね合いで無理
// pub x: Vec<usize>みたいなことも出来ない
// electric_fieldの中にchargeを入れてみようかな

// 次はadd chargeを作ろうかな
// charge_nummberを増やしつつ

pub struct Charge {
    pub q: f64,
    pub x: isize,
    pub y: isize,
    vx: f64,
    vy: f64,
    pub ax: f64,
    pub ay: f64,
}

impl Charge{
    pub fn new(qi: f64, xi: isize, yi: isize) -> Charge{
        let q: f64 = qi;
        let x: isize = xi - 256;
        let y: isize = yi - 256;
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
        let next_x: f64 = self.vx - self.ax * 0.1;
        let next_y: f64 = self.vy - self.ay * 0.1;
        self.vx = next_x;
        self.vy = next_y;
    }

    pub fn calc_position_charge(&mut self){
        // マイナスにした途端下のfn axでもfn ax2でも値が更新されない
        let next_x: f64 = self.x as f64 + self.vx * 0.000001;
        let next_y: f64 = self.y as f64 + self.vy * 0.000001;
        if next_x > -256.0 && next_x < 256.0{self.x = next_x as isize;}
        if next_y > -256.0 && next_y < 256.0{self.y = next_y as isize;}
        // self.x = next_x as isize;
        // self.y = next_y as isize;
    }

    pub fn calc_v_p_charge(&mut self){
        // calc_velocity
        let next_vx: f64 = self.vx - self.ax * 0.1;
        let next_vy: f64 = self.vy - self.ay * 0.1;
        // calc_position
        let next_rx: f64 = self.x as f64 + next_vx * 0.000001;
        let next_ry: f64 = self.y as f64 + next_vy * 0.000001;
        if next_rx > -256.0 && next_rx < 256.0{
            self.vx = next_vx;
            self.x = next_rx as isize;
        }else{
            self.vx = 0.0; // 完全非弾性
            // self.vx = -next_vx; // 完全弾性
            // self.vx = -next=vx * 0.5; // 弾性
        }
        if next_ry > -256.0 && next_ry < 256.0{
            self.vy = next_vy;
            self.y = next_ry as isize;
        }else{
            self.vy = 0.0;
        }
    }

    pub fn ax(&self) -> isize {
        self.y
    }

    pub fn ax2(&self) -> isize {
        (self.y as f64 - self.vy) as isize
    }
}