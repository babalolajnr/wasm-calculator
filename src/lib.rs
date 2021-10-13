mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn add(operand1: u8, operand2: u8) -> u8 {
    let answer = operand1 + operand2;
    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add_function() {
        let answer = add(1, 2);
        assert_eq!(answer, 3);
    }
}
