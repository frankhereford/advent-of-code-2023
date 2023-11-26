mod utils;
mod solutions;
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
    let greeting = format!("Hello, television! The current date and time is: {}", date_time);
    alert(&greeting);
}

#[wasm_bindgen]
pub fn solution() {
    let solution = solutions::day_00::solution(5000000);
    alert(&solution);
}