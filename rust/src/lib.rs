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
    match n {
        0 => solutions::day_00::solution(1111111),
        1 => {
            solutions::day_01::solution_part_1();
            solutions::day_01::solution_part_2();
        },
        2 => {
            solutions::day_02::solution_part_1();
            solutions::day_02::solution_part_2();
        },
        3 => {
            solutions::day_03::solution_part_1();
            solutions::day_03::solution_part_2();
        },
        4 => {
            solutions::day_04::solution_part_1();
            solutions::day_04::solution_part_2();
        },
        5 => {
            solutions::day_05::solution_part_1();
            solutions::day_05::solution_part_2();
        },
        6 => {
            solutions::day_06::solution_part_1();
            solutions::day_06::solution_part_2();
        },
        7 => {
            solutions::day_07::solution_part_1();
            solutions::day_07::solution_part_2();
        },
        8 => {
            solutions::day_08::solution_part_1();
            solutions::day_08::solution_part_2();
        },
        9 => {
            solutions::day_09::solution_part_1();
            solutions::day_09::solution_part_2();
        },
        10 => {
            solutions::day_10::solution_part_1();
            solutions::day_10::solution_part_2();
        },
        _ => ()
    }
}