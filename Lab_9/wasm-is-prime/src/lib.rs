mod utils;

use wasm_bindgen::prelude::*;
use primal::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn check_prime(s: &JsValue) -> u32 {
    let input: String = s.as_string().unwrap();
    match input.parse::<u64>() {
        Ok(num) => {
            if is_prime(num) {
                alert("Input is Prime");
                return 1;
            } else {
                alert("Input is NOT Prime");
                return 0;
            }
        },
        Err(_) => {
            alert(&format!("Couldn't parse {}", input));
            return 0;
        }
    }
}

