#![allow(unused_variables)]
//use web_sys::console;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(do_print: bool, message: &str);
}

pub fn solution_part_1() -> () {
    postMessageToWorker(true, "Part 1: \n");
    let mut iteration = -1;
    let content = include_str!("input/day_08_part_1_test_1_input.txt");
    // let content = include_str!("input/day_08_input.txt");

    content.lines().for_each(|line| {
        // Provide a mechanism to limit the volume of output on the console.
        iteration += 1;
        let mut show_message = false;
        if (iteration) % 1 == 0  {
            show_message = true;
        }

        postMessageToWorker(show_message, " ");
        postMessageToWorker(show_message, &format!("Iteration: {}, input: {}", iteration, line));
    });
}


pub fn solution_part_2() -> () {
}