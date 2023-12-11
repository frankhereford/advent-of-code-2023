#![allow(unreachable_code)]
#![allow(unused_variables)]
//use web_sys::console;
use wasm_bindgen::prelude::*;

use regex::Regex;

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(do_print: bool, message: &str);
}



pub fn solution_part_1() -> () {
    postMessageToWorker(true, "Part 1: \n");
    let content = include_str!("input/day_06_test_input.txt");
    // let content = include_str!("input/day_06_input.txt");

    let time_line = Regex::new(r"Time:(.*)").unwrap();
    let distance_line = Regex::new(r"Distance:(.*)").unwrap();

    if let Some(caps) = time_line.captures(content) {
        let times_as_string = caps.get(1).map_or("", |m| m.as_str());
        postMessageToWorker(true, &format!("Times as string: {}", times_as_string));
    }
    if let Some(caps) = distance_line.captures(content) {
        let distances_as_string = caps.get(1).map_or("", |m| m.as_str());
        postMessageToWorker(true, &format!("Distances as string: {}", distances_as_string));
    }



}

fn split_digits_over_whitespace(input: &str) -> Vec<u32> {
    //console::log_1(&format!("input: {}", input).into());
    let whitespace = Regex::new(r"\s+").unwrap();
    let parts_as_strings: Vec<&str> = whitespace.split(input).collect();
    let mut found_numbers: Vec<u32> = Vec::new();
    for number in parts_as_strings {
        //console::log_1(&format!("number: {}", number).into());
        let number_as_int = number.parse::<u32>().expect("Should be able to parse number");
        found_numbers.push(number_as_int);
    }
    found_numbers
}

pub fn solution_part_2() -> () {
    return;
    postMessageToWorker(true, "Part 2: \n");
    let mut iteration = -1;
    let content = include_str!("input/day_06_test_input.txt");
    // let content = include_str!("input/day_06_input.txt");

    content.lines().for_each(|line| {
        // Provide a mechanism to limit the volume of output on the console.
        iteration += 1;
        let mut show_message = false;
        if (iteration) % 300 == 0  {
            show_message = true;
        }

        let characters: Vec<_> = line.chars().collect();
        if characters[0] == '#' {
            // postMessageToWorker(show_message, "Skipping line because it is a comment.");
            return;
        }

        postMessageToWorker(show_message, " ");
        postMessageToWorker(show_message, &format!("Iteration: {}, input: {}", iteration, line));
    });
}