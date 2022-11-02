//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate wasm_game_of_life;
use wasm_game_of_life::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
pub fn test_tick() {
    // □ □ □ □ □ □
    // □ □ □ ■ □ □
    // □ ■ □ ■ □ □
    // □ □ ■ ■ □ □
    // □ □ □ □ □ □
    // □ □ □ □ □ □
    let mut input_universe = Universe::new();
    input_universe.set_width(6);
    input_universe.set_height(6);
    input_universe.set_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
    input_universe.tick();

    // □ □ □ □ □ □
    // □ □ ■ □ □ □
    // □ □ □ ■ ■ □
    // □ □ ■ ■ □ □
    // □ □ □ □ □ □
    // □ □ □ □ □ □
    let mut expected_universe = Universe::new();
    expected_universe.set_width(6);
    expected_universe.set_height(6);
    expected_universe.set_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);

    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}
