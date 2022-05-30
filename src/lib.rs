mod ElectricField;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<u8>,
}

impl Universe {
    fn get_index(&self, y: u32, x: u32) -> usize {
        (y * self.width + x) as usize
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for j in 0..self.height {
            for i in 0..self.width {

                let y = j - self.height / 2;
                let x = i - self.width / 2;
                let r = ((x.pow(2) + y.pow(2)) / 2080) as u8;

                let idx = self.get_index(j*4, i*4);
                next[idx] += r;
                next[idx+1] += 1;
                next[idx+2] += 1;
                
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        let width = 513 as u32;
        let height = 513 as u32;
        let n = (width * height * 4) as usize;

        let cells: Vec<u8> = vec![255; n];

        Universe {  
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u8 {
        self.cells.as_ptr()
    }
}