pub struct ElectricField{
    width: u16,
    height: u16,
    electric_field_x: Vec<i64>,
    electric_field_y: Vec<i64>,
}

impl ElectricField{
    pub fn new() -> ElectricField{
        let width = 513 as u16;
        let height = 513 as u16;
        let width_half = width / 2;
        let height_half = height / 2;
        let mut electric_field_x: Vec<i64> = Vec::new();
        let mut electric_field_y: Vec<i64> = Vec::new();
        for j in 0..height{
            for i in 0..width{
                let y = j - height_half;
                let x = i - width_half;
                let r = ((x*x + y*y) as f64).sqrt();
                let r_three = r * r * r as f64;
                let e_norm = (8987552000.0 as f64) / r_three;
                let e_x = e_norm * x as f64;
                let e_y = e_norm * y as f64;
                electric_field_x.push(e_x as i64);
                electric_field_y.push(e_y as i64);
            }
        }
    
        ElectricField{
            width,
            height,
            electric_field_x,
            electric_field_y,
        }
    }
}