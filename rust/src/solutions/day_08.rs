#![allow(unused_variables)]
//use web_sys::console;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(do_print: bool, message: &str);
}

pub fn solution_part_1() -> () {
    postMessageToWorker(true, "Part 1: \n");
    let content = include_str!("input/day_08_part_1_test_1_input.txt");
    // let content = include_str!("input/day_08_input.txt");
    let input = content.lines().collect::<Vec<&str>>();
    let instructions = input[0].chars().collect::<Vec<char>>();
    let mut instruction = instructions.iter().cycle(); // can just access this forever
}


pub fn solution_part_2() -> () {
}