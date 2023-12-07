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
    //let content = include_str!("input/day_05_part_1_test_input.txt");
    let content = include_str!("input/day_05_input.txt");

    let (seeds, almanac) = parse_alamanac(content);
    postMessageToWorker(true, &format!("Almanac: {:?}", almanac));
    let steps = ["seed", "soil", "fertilizer", "water", "light", "temperature", "humidity", "location"];
    let steps_length = steps.len();
    postMessageToWorker(true, &format!("Seeds: {:?}", seeds));
    let mut final_locations: Vec<u32> = Vec::new();
    for seed in seeds {
        postMessageToWorker(true, &format!("Seed: {}", seed));
        let mut current_value = seed;
        for i in 0..(steps_length-1) {
            let step = steps[i];
            let next_step = steps[i + 1];
            let key = format!("{}-{}", step, next_step);
            // postMessageToWorker(true, &format!("key: {}", key));
            if let Some(entry) = almanac.get(&key) {
                //postMessageToWorker(true, &format!("entry: {:?}", entry));
                for mapping in entry {
                    let applicable_map = is_applicable_map(current_value, mapping);
                    //postMessageToWorker(true, &format!("trying this map: {:?}, is applicable?: {:?}", mapping, applicable_map));
                    if applicable_map {
                        let output_location = compute_output_location(current_value, mapping);
                        postMessageToWorker(true, &format!("Current value before {}: {}", key, current_value));
                        postMessageToWorker(true, &format!("{} mapping: {:?}", key, mapping));
                        postMessageToWorker(true, &format!("Key: {}, output_location: {}", key, output_location));
                        current_value = output_location;
                        break;
                    }
                }
                //if let Some(value) = entry.get(&current_value) {
                    // postMessageToWorker(true, &format!("value: {}", value));
                    //current_value = *value;
                //}
            postMessageToWorker(true, &format!("Current value after {}: {}", key, current_value));
            }
        }
        postMessageToWorker(true, &format!("Final value: {}", current_value));
        final_locations.push(current_value);
    }
    if let Some(min) = final_locations.iter().min() {
        postMessageToWorker(true, &format!("Min: {:?}", min));
    }
}

fn is_applicable_map(input_number: u32, mapping: &HashMap<String, u32>) -> bool {
    let source_range = mapping.get("source_range").unwrap();
    let destination_range = mapping.get("destination_range").unwrap();
    let range_length = mapping.get("range_length").unwrap();
    if input_number >= *source_range && input_number <= source_range + range_length {
        return true;
    }
    false
}

fn compute_output_location(input: u32, mapping: &HashMap<String, u32>) -> u32 {
    let source_range = *mapping.get("source_range").unwrap() as i32;
    let destination_range = *mapping.get("destination_range").unwrap() as i32;
    let range_length = *mapping.get("range_length").unwrap() as i32;
    let shift_magnatude: i32 = destination_range - source_range;
    let output_location = input as i32 + shift_magnatude;
    output_location as u32
    //let output_location = input as i32 + shift_magnatude;
    //output_location as u32
}

fn parse_alamanac(content: &str) -> (Vec<u32>, HashMap<String, Vec<HashMap<String, u32>>>) {
    let show_message = true;

    let line_count = content.lines().count();
    let mut line_number = 0;

    let seed_detector = Regex::new(r"seeds: ([\d ]+)").unwrap();
    let map_detector = Regex::new(r"(\w+)-to-(\w+) map").unwrap();
    let digit_detector = Regex::new(r"\d").unwrap();

    let mut almanac: HashMap<String, Vec<HashMap<String, u32>>> = HashMap::new();
    let mut seeds = Vec::new();

    while line_number < line_count {
        let line = content.lines().nth(line_number).unwrap();
        //postMessageToWorker(show_message, &format!("Parsing line: {}", line));

        if let Some(captures) = seed_detector.captures(line) {
            seeds = split_digits_over_whitespace(captures.get(1).map_or("", |m| m.as_str()));
        }

        if let Some(captures) = map_detector.captures(line) {
            let origin = captures.get(1).map_or("", |m| m.as_str());
            let destination = captures.get(2).map_or("", |m| m.as_str());
            //postMessageToWorker(show_message, &format!("Origin: {}, Destination: {}", origin, destination));

            let key = format!("{}-{}", origin, destination);
            almanac.insert(key.clone(), Vec::new()); // Clone `key` here

            line_number += 1;
            loop {
                if line_number >= line_count {
                    return (seeds, almanac);
                    //break;
                }
                let line = content.lines().nth(line_number).unwrap();
                // postMessageToWorker(show_message, &format!("Parsing line!!: {}", line));
                if digit_detector.is_match(&line) {
                    line_number += 1;
                    // postMessageToWorker(show_message, &format!("line: {}", line));
                    let digits = split_digits_over_whitespace(line);
                    //postMessageToWorker(show_message, &format!("digits: {:?}", digits));
                    let mut mapping: HashMap<String, u32> = HashMap::new();
                    mapping.insert("source_range".to_string(), digits[1]);
                    mapping.insert("destination_range".to_string(), digits[0]);
                    mapping.insert("range_length".to_string(), digits[2]);
                    almanac.get_mut(&key).unwrap().push(mapping);
                } else {
                    break;
                }
            }
        }


        line_number += 1; 
    }

    (seeds, almanac)
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