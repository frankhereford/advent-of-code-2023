#![allow(unused_variables)]
#![allow(unreachable_code)]
//use web_sys::console;
use wasm_bindgen::prelude::*;
//use regex::Regex;
//use std::collections::{HashSet, HashMap};
use polyfit_rs::polyfit_rs::polyfit;

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(do_print: bool, message: &str);
}

pub fn solution_part_1() -> () {
    postMessageToWorker(true, "Part 1: \n");
    let content = include_str!("input/day_09_part_1_test_input.txt");
    //let content = include_str!("input/day_09_part_1_input.txt");

}


pub fn solution_part_2() -> () {
    return;
    postMessageToWorker(true, "Part 2: \n");
    let content = include_str!("input/day_09_part_1_test_input.txt");
    //let content = include_str!("input/day_09_part_1_input.txt");
}
