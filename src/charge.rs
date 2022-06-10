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
    ax: f64,
    ay: f64,
}

impl Charge{
    pub fn new(q: f64, x: usize, y: usize) -> Charge{
        let q: f64 = q;
        let x: usize = x;
        let y: usize = y;
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
}