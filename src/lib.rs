mod utils;

extern crate js_sys;
use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
#[repr(u8)] // So that each cell is represented as a byte.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

/// Private helper methods.
impl Universe {
    /// Translate a 2D (`row`, `col`) position into an internal array `index`.
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    /// Count the number of living neighbors surrounding a 2D (`row`, `col`) position.
    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8; // +1 if Cell is Alive.
            }
        }

        count
    }
}

/// Public methods exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    /// Get the width of the universe, i.e. number of columns.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get the height of the universe, i.e. number of rows.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Get a pointer to the universe's cells.
    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    /// Construct a new universe.
    pub fn new() -> Self {
        let width = 8;
        let height = 8;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, i % 2 == 0 || i % 7 == 0);
        }

        Self {
            width,
            height,
            cells,
        }
    }

    /// Advance the state of the world by 1 tick.
    pub fn tick(&mut self) {
        let mut next_cells = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let live_neighbors = self.live_neighbor_count(row, col);
                let cell = self.cells[idx];
                next_cells.set(
                    idx,
                    match (cell, live_neighbors) {
                        // Rule 1: Any live cell with fewer than two live neighbours
                        // dies, as if caused by underpopulation.
                        (true, x) if x < 2 => false,
                        // Rule 2: Any live cell with two or three live neighbours
                        // lives on to the next generation.
                        (true, 2) | (true, 3) => true,
                        // Rule 3: Any live cell with more than three live
                        // neighbours dies, as if by overpopulation.
                        (true, x) if x > 3 => false,
                        // Rule 4: Any dead cell with exactly three live neighbours
                        // becomes a live cell, as if by reproduction.
                        (false, 3) => true,
                        // All other cells remain in the same state.
                        (otherwise, _) => otherwise,
                    },
                );
            }
        }

        self.cells = next_cells;
    }
}
