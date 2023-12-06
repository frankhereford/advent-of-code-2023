#![allow(unreachable_code)]
#![allow(unused_variables)]
//use web_sys::console;
use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use regex::Regex;

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(do_print: bool, message: &str);
}

pub fn solution_part_1() -> () {
    postMessageToWorker(true, "Part 1: \n");
    let content = include_str!("input/day_05_part_1_test_input.txt");
    // let content = include_str!("input/day_05_input.txt");

    let almanac = parse_alamanac(content);
    postMessageToWorker(true, &format!("Almanac: {:?}", almanac));

}


fn parse_alamanac(content: &str) -> HashMap<String, HashMap<u32, u32>> {
    let show_message = true;

    let line_count = content.lines().count();
    let mut line_number = 0;

    let seed_detector = Regex::new(r"seeds: ([\d ]+)").unwrap();
    let map_detector = Regex::new(r"(\w+)-to-(\w+) map").unwrap();
    let digit_detector = Regex::new(r"\d").unwrap();

    let mut almanac: HashMap<String, HashMap<u32, u32>> = HashMap::new();

    while line_number < line_count {
        let line = content.lines().nth(line_number).unwrap();
        postMessageToWorker(show_message, &format!("Parsing line: {}", line));

        if let Some(captures) = seed_detector.captures(line) {
            let seeds = split_digits_over_whitespace(captures.get(1).map_or("", |m| m.as_str()));
            postMessageToWorker(show_message, &format!("Seeds: {:?}", seeds));
        }

        if let Some(captures) = map_detector.captures(line) {
            let origin = captures.get(1).map_or("", |m| m.as_str());
            let destination = captures.get(2).map_or("", |m| m.as_str());
            postMessageToWorker(show_message, &format!("Origin: {}, Destination: {}", origin, destination));

            let key = format!("{}-{}", origin, destination);
            almanac.insert(key.clone(), HashMap::new()); // Clone `key` here

            line_number += 1;
            loop {
                if line_number >= line_count {
                    break;
                }
                let line = content.lines().nth(line_number).unwrap();
                // postMessageToWorker(show_message, &format!("Parsing line!!: {}", line));
                if digit_detector.is_match(&line) {
                    line_number += 1;
                    // postMessageToWorker(show_message, &format!("line: {}", line));
                    let digits = split_digits_over_whitespace(line);
                    postMessageToWorker(show_message, &format!("digits: {:?}", digits));
                    if let Some(entry) = almanac.get_mut(&key) {
                        for i in 0..digits[2] {
                            let range_key = digits[1] + i;
                            let range_value = digits[0] + i;
                            postMessageToWorker(show_message, &format!("range_key: {}, range_value: {}", range_key, range_value));
                            entry.insert(range_key, range_value);
                        }
                    }
                } else {
                    break;
                }
            }
        }


        line_number += 1; 
    }

    almanac
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
    let content = include_str!("input/day_05_part_1_test_input.txt");
    // let content = include_str!("input/day_05_input.txt");

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