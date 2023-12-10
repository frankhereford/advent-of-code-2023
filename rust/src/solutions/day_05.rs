//#![allow(unreachable_code)]
//#![allow(unused_variables)]
//#![allow(dead_code)]
//#![allow(unused_mut)]
//use web_sys::console;
use wasm_bindgen::prelude::*;
use regex::Regex;
use std::collections::{HashMap};

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(do_print: bool, message: &str);
}

pub fn solution_part_1() -> () {
    postMessageToWorker(true, "Part 1: \n");
    //let content = include_str!("input/day_05_part_1_test_input.txt");
    let content = include_str!("input/day_05_input.txt");

    let (seeds, almanac) = parse_almanac(content);
    let steps = ["seed", "soil", "fertilizer", "water", "light", "temperature", "humidity", "location"];
    let steps_length = steps.len();
    postMessageToWorker(true, &format!("Seeds: {:?}", seeds));
    let mut final_locations: Vec<u64> = Vec::new();
    for seed in seeds {
        postMessageToWorker(true, &format!("Seed: {}", seed));
        let mut current_value: u64 = seed as u64;
        for i in 0..(steps_length-1) {
            let step = steps[i];
            let next_step = steps[i + 1];
            let key = format!("{}-{}", step, next_step);
            if let Some(entry) = almanac.get(&key) {
                for mapping in entry {
                    let applicable_map = is_applicable_map(current_value, mapping);
                    if applicable_map {
                        let output_location = compute_output_location(current_value, mapping);
                        current_value = output_location;
                        break;
                    }
                }
            }
        }
        postMessageToWorker(true, &format!("Final value: {}", current_value));
        final_locations.push(current_value);
    }
    if let Some(min) = final_locations.iter().min() {
        postMessageToWorker(true, &format!("Min: {:?}", min));
    }
}

fn is_applicable_map(input_number: u64, mapping: &HashMap<String, u64>) -> bool {
    let source_range = mapping.get("source_range").unwrap();
    let range_length = mapping.get("range_length").unwrap();
    if input_number >= *source_range && input_number <= source_range + range_length {
        return true;
    }
    false
}

fn compute_output_location(input: u64, mapping: &HashMap<String, u64>) -> u64 {
    let source_range = *mapping.get("source_range").unwrap() as i64;
    let destination_range = *mapping.get("destination_range").unwrap() as i64;
    let shift_magnatude: i64 = destination_range - source_range;
    let output_location = input as i64 + shift_magnatude;
    output_location as u64
}

fn parse_almanac(content: &str) -> (Vec<u64>, HashMap<String, Vec<HashMap<String, u64>>>) {

    let line_count = content.lines().count();
    let mut line_number = 0;

    let seed_detector = Regex::new(r"seeds: ([\d ]+)").unwrap();
    let map_detector = Regex::new(r"(\w+)-to-(\w+) map").unwrap();
    let digit_detector = Regex::new(r"\d").unwrap();

    let mut almanac: HashMap<String, Vec<HashMap<String, u64>>> = HashMap::new();
    let mut seeds = Vec::new();

    while line_number < line_count {
        let line = content.lines().nth(line_number).unwrap();

        if let Some(captures) = seed_detector.captures(line) {
            seeds = split_digits_over_whitespace(captures.get(1).map_or("", |m| m.as_str()));
        }

        if let Some(captures) = map_detector.captures(line) {
            let origin = captures.get(1).map_or("", |m| m.as_str());
            let destination = captures.get(2).map_or("", |m| m.as_str());

            let key = format!("{}-{}", origin, destination);
            almanac.insert(key.clone(), Vec::new()); // Clone `key` here

            line_number += 1;
            loop {
                if line_number >= line_count {
                    return (seeds, almanac);
                }
                let line = content.lines().nth(line_number).unwrap();
                if digit_detector.is_match(&line) {
                    line_number += 1;
                    let digits = split_digits_over_whitespace(line);
                    let mut mapping: HashMap<String, u64> = HashMap::new();
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

fn split_digits_over_whitespace(input: &str) -> Vec<u64> {
    //console::log_1(&format!("input: {}", input).into());
    let whitespace = Regex::new(r"\s+").unwrap();
    let parts_as_strings: Vec<&str> = whitespace.split(input).collect();
    let mut found_numbers: Vec<u64> = Vec::new();
    for number in parts_as_strings {
        //console::log_1(&format!("number: {}", number).into());
        let number_as_int = number.parse::<u64>().expect("Should be able to parse number");
        found_numbers.push(number_as_int);
    }
    found_numbers
}

pub fn solution_part_2() -> () {
    postMessageToWorker(true, "Part 2: \n");
    //let content = include_str!("input/day_05_part_1_test_input.txt");
    let content = include_str!("input/day_05_input.txt");

    let (seeds_parse, almanac) = parse_almanac(content);
    let mut steps = ["seed", "soil", "fertilizer", "water", "light", "temperature", "humidity", "location"];
    steps.reverse();

    //let skip = 10000;
    //let mut location: u64 = 0;
    let skip = 1;
    let mut location: u64 = 81960000 - 10000;
    let mut iteration = 0;
    loop {
      if iteration % 10000 == 0 { postMessageToWorker(true, &format!("Iteration: {}", iteration)); }
      let testing_location = location;
        
        for n in 0..steps.len()-1 {
            let key = format!("{}-{}", steps[n+1], steps[n]);
        
            for map in almanac.get(&key).unwrap() {
                let map_source_start = map.get("source_range").unwrap();
                let map_destination_start = map.get("destination_range").unwrap();
                let map_range_length = map.get("range_length").unwrap();

                let map_destination_end = map_destination_start + map_range_length;

                if location >= *map_destination_start && location < map_destination_end {
                    let offset_off_destination_start = location - map_destination_start; 
                    let inferred_source_value = map_source_start + offset_off_destination_start;
                    location = inferred_source_value;
                    break; // break out of checking any more maps for this path because we found one
                }

            }
        }

        let mut seeds_parse_for_this_iteration = seeds_parse.clone();
        while seeds_parse_for_this_iteration.len() > 0 {
            let seed_range_start: u64 = seeds_parse_for_this_iteration.remove(0);
            let seed_range_length: u64 = seeds_parse_for_this_iteration.remove(0);
            let seed_range_end: u64 = seed_range_start + seed_range_length;
            if location >= seed_range_start && location < seed_range_end {
                postMessageToWorker(true, &format!("Found a valid seed for testing-location {} at seed value: {}", testing_location, location));
                return
            }

        }

        location = testing_location + skip;
        iteration += 1;
    }
    
}
