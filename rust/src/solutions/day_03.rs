#![allow(unreachable_code)]
//use web_sys::console;
use wasm_bindgen::prelude::*;
use regex::Regex;

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(do_print: bool, message: &str);
}

pub fn solution_part_1() -> () {
    postMessageToWorker(true, "Part 1: Sumation of symbol adjacent scalars.\n");
    let mut iteration = -1;
    let content = include_str!("input/day_03_part_1_test_input.txt");
    // let content = include_str!("input/day_XX_input.txt");

    let is_digit_regex  = Regex::new(r"\d").unwrap();
    let is_blank_regex  = Regex::new(r"\.").unwrap();

    content.lines().for_each(|line| {
        // Provide a mechanism to limit the volume of output on the console.
        iteration += 1;
        let mut show_message = false;
        if (iteration) % 1 == 0  {
            show_message = true;
        }

        let characters: Vec<_> = line.chars().collect();
        if characters[0] == '`' {
            // postMessageToWorker(show_message, "Skipping line because it is a comment.");
            return;
        }

        postMessageToWorker(show_message, " ");
        postMessageToWorker(show_message, &format!("Iteration: {}, input: {}", iteration, line));

        let mut is_in_number = false;
        let mut number_location = 0;
        let mut number_as_string = String::new();
        for (index, character) in characters.iter().enumerate() {
            //postMessageToWorker(show_message, &format!("Character: {}", character));
            if is_digit_regex.is_match(character.to_string().as_str()) {
                postMessageToWorker(show_message, &format!("Found a digit: {}", character));
                number_as_string.push(*character);
                if !is_in_number {
                    is_in_number = true;
                    number_location = index;
                }
            } else { // either a blank or a symbol
                if is_in_number {
                    handle_found_number(number_location, number_as_string);
                }
                is_in_number = false;
                number_as_string = String::new();
                if is_blank_regex.is_match(character.to_string().as_str())  {
                } else {
                postMessageToWorker(show_message, &format!("Found a symbol: {}", character));
                }
            }
        }
        if is_in_number {
            handle_found_number(number_location, number_as_string);
        }
    });
}

fn handle_found_number(number_location: usize, number_as_string: String) -> () {
    postMessageToWorker(true, &format!("Found a number at {}: {}", number_location, number_as_string));
}

pub fn solution_part_2() -> () {
    return;
    postMessageToWorker(true, "Part 2: \n");
    let mut iteration = -1;
    let content = include_str!("input/day_03_part_1_test_input.txt");
    // let content = include_str!("input/day_XX_input.txt");

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