mod solutions;
mod utils;
use chrono::Local;
use wasm_bindgen::prelude::*;

pub fn get_current_date_time() -> String {
    let now = Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[wasm_bindgen(start)]
pub fn init() {
    utils::set_panic_hook(); // Optional: Sets up better panic messages
                             // Any other initialization code
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let date_time = get_current_date_time();
    let greeting = format!(
        "Hello, television! The current date and time is: {}",
        date_time
    );
    alert(&greeting);
}

#[wasm_bindgen]
pub fn solution(n: u32) -> () {
    if n == 0 {
        let _solution = solutions::day_00::solution(1111111);
        // solution
    }
    else if n == 1 {
        let _solution_part_1 = solutions::day_01::solution_part_1(100);
        // solution
    } else {
        // Handle other cases or return a default value
        // String::new() // or any other appropriate default String value
    }
}
