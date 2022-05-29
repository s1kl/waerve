mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
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
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in (0..self.height*4).step_by(4) {
            for col in (0..self.height*4).step_by(4) {
                let idx = self.get_index(row, col);
                next[idx] += 1;
                next[idx+1] += 1;
                next[idx+2] += 1;
                next[idx+3] += 1;
            }
        }
        
        // let n = self.height*self.width*4;
        // for i in (0..n){
        //     next[i] += 1;
        // }

        self.cells = next;
    }

    pub fn new() -> Universe {
        let width = 1200 as u32;
        let height = 1200 as u32;
        let n = (width * height * 4) as usize;

        let cells = vec![0; n];
        // let mut cells = vec![];
        // for i in 0..width * height {
        //     if i % 2 == 0 || i % 11 == 0 {
        //         for _ in 0..4 {
        //             cells.push(1);
        //         }
        //     } else {
        //         for _ in 0..4 {
        //             cells.push(100);
        //         }
        //     }
        // }

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

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}