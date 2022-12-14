mod utils;

extern crate js_sys;
extern crate web_sys;
use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
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

/// Public methods, not exposed to JavaScript.
impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[u32] {
        &self.cells.as_slice()
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }
}

/// Public methods exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    /// Get the width of the universe, i.e. number of columns.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Set the width of the universe to `width`.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells.clear();
    }

    /// Set the height of the universe to `height`.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells.clear();
    }

    /// Get the height of the universe, i.e. number of rows.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Get a pointer to the universe's cells.
    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    /// Turn a dead cell into a live cell, and vice versa.
    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row % self.height, col % self.width);
        self.cells.set(idx, !self.cells[idx]);
    }

    /// Turn a dead cell into a live cell.
    pub fn set_cell(&mut self, row: u32, col: u32, state: bool) {
        let idx = self.get_index(row % self.height, col % self.width);
        self.cells.set(idx, state);
    }

    /// Clear the state, turning all cells dead.
    pub fn clear(&mut self) {
        self.cells.clear();
    }

    /// Randomize the state of the universe.
    pub fn randomize(&mut self) {
        for idx in 0..self.cells.len() {
            self.cells.set(idx, js_sys::Math::random() < 0.5);
        }
    }

    /// Construct a new universe.
    pub fn new() -> Self {
        utils::set_panic_hook();

        let width = 32;
        let height = 32;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, i % 2 == 0 || i % 7 == 0);
        }

        log!(
            "Creating a new [{}, {}] universe with {} live cells.",
            width,
            height,
            cells.count_ones(..)
        );

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
