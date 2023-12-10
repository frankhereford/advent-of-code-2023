#![allow(unreachable_code)]
#![allow(unused_variables)]
//#![allow(dead_code)]
//#![allow(unused_mut)]
//use web_sys::console;
use wasm_bindgen::prelude::*;
use regex::Regex;
use std::collections::{HashMap};

// for pretty printing
//use std::collections::{BTreeMap, HashMap};
//use serde_json;

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(do_print: bool, message: &str);
}

pub fn solution_part_1() -> () {
    return;
    postMessageToWorker(true, "Part 1: \n");
    //let content = include_str!("input/day_05_part_1_test_input.txt");
    let content = include_str!("input/day_05_input.txt");

    let (seeds, almanac) = parse_alamanac(content);
    postMessageToWorker(true, &format!("Almanac: {:?}", almanac));
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

fn is_applicable_map(input_number: u64, mapping: &HashMap<String, u64>) -> bool {
    let source_range = mapping.get("source_range").unwrap();
    let destination_range = mapping.get("destination_range").unwrap();
    let range_length = mapping.get("range_length").unwrap();
    if input_number >= *source_range && input_number <= source_range + range_length {
        return true;
    }
    false
}

fn compute_output_location(input: u64, mapping: &HashMap<String, u64>) -> u64 {
    let source_range = *mapping.get("source_range").unwrap() as i64;
    let destination_range = *mapping.get("destination_range").unwrap() as i64;
    let range_length = *mapping.get("range_length").unwrap() as i64;
    let shift_magnatude: i64 = destination_range - source_range;
    let output_location = input as i64 + shift_magnatude;
    output_location as u64
}

fn parse_alamanac(content: &str) -> (Vec<u64>, HashMap<String, Vec<HashMap<String, u64>>>) {
    let show_message = true;

    let line_count = content.lines().count();
    let mut line_number = 0;

    let seed_detector = Regex::new(r"seeds: ([\d ]+)").unwrap();
    let map_detector = Regex::new(r"(\w+)-to-(\w+) map").unwrap();
    let digit_detector = Regex::new(r"\d").unwrap();

    let mut almanac: HashMap<String, Vec<HashMap<String, u64>>> = HashMap::new();
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
                }
                let line = content.lines().nth(line_number).unwrap();
                // postMessageToWorker(show_message, &format!("Parsing line!!: {}", line));
                if digit_detector.is_match(&line) {
                    line_number += 1;
                    // postMessageToWorker(show_message, &format!("line: {}", line));
                    let digits = split_digits_over_whitespace(line);
                    //postMessageToWorker(show_message, &format!("digits: {:?}", digits));
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
    let content = include_str!("input/day_05_part_1_test_input.txt");
    //let content = include_str!("input/day_05_input.txt");

    let (mut seeds_parse, almanac) = parse_alamanac(content);
    //postMessageToWorker(true, &format!("Raw Seeds: {:?}", seeds_parse));
    //postMessageToWorker(true, &format!("Almanac: {:?}", almanac));
    let steps = ["seed", "soil", "fertilizer", "water", "light", "temperature", "humidity", "location"];
    
    // pretty_print_almanac_parse(&seeds_parse, &almanac);

    while seeds_parse.len() > 0 {
        let output_start: u64 = seeds_parse.remove(0);
        let output_length: u64 = seeds_parse.remove(0);
        let output_end = output_start + output_length;

        let mut previous_step = "seed";
        for step in steps {
            if step == "seed" { continue; }
            let key = format!("{}-{}", previous_step, step);
            postMessageToWorker(true, &format!("Key: {}", key));
            
            for map in almanac.get(&key).unwrap() {
                let input_range_start = map.get("source_range").unwrap();
                let input_range_end = map.get("source_range").unwrap() + map.get("range_length").unwrap();

                if output_end >= *input_range_start && output_start <= input_range_end {
                    let output_range_start = map.get("destination_range").unwrap();
                    let output_range_end = map.get("destination_range").unwrap() + map.get("range_length").unwrap();
                    // postMessageToWorker(true, &format!("output_range_start: {}, output_range_end: {}", output_range_start, output_range_end));
                }
            }

            previous_step = step;
        }
    }
}


    /*
    while seeds_parse.len() > 0 {
        let seed_start: u64 = seeds_parse.remove(0);
        let seed_range_length: u64 = seeds_parse.remove(0);
        let seed_end = seed_start + seed_range_length;
        postMessageToWorker(true, &format!("seed_start: {}, seed_end: {}", seed_start, seed_end));
        let mut previous_step = "seed";
        for step in steps {
            if step == "seed" { continue; }
            let key = format!("{}-{}", previous_step, step);
            postMessageToWorker(true, &format!("Key: {}", key));
            
            for map in almanac.get(&key).unwrap() {
                // postMessageToWorker(true, &format!("map: {:?}", map));

                let input_range_start = map.get("source_range").unwrap();
                let input_range_end = map.get("source_range").unwrap() + map.get("range_length").unwrap();
                postMessageToWorker(true, &format!("input_range_start: {}, input_range_end: {}", input_range_start, input_range_end));

                let output_range_start = map.get("destination_range").unwrap();
                let output_range_end = map.get("destination_range").unwrap() + map.get("range_length").unwrap();
                // postMessageToWorker(true, &format!("output_range_start: {}, output_range_end: {}", output_range_start, output_range_end));

                if seed_end >= *input_range_start && seed_start <= input_range_end {
                    // found a range we have some overlap with
                    let overlap_begin = if seed_start > *input_range_start { seed_start } else { *input_range_start };
                    let overlap_end = if seed_end < input_range_end { seed_end } else { input_range_end };
                    postMessageToWorker(true, &format!("overlap_begin: {}, overlap_end: {}", overlap_begin, overlap_end));

                }
            }

            previous_step = step;
        }
    }
    */
    
    /*
    let mut previous_step = "seed";
    for step in steps {
        if step == "seed" { continue; }
        let key = format!("{}-{}", previous_step, step);
        postMessageToWorker(true, &format!("Key: {}", key));
        for mapping in almanac.get(&key).unwrap() {
            postMessageToWorker(true, &format!("mapping: {:?}", mapping));
        }
        previous_step = step;
    }
    */


/*

    return;

    let steps_length = steps.len();
    let mut final_locations: Vec<u64> = Vec::new();
    
    //let mut seeds: Vec<u64> = Vec::new();
    while seeds_parse.len() > 0 {
        let seed_start: u64 = seeds_parse.remove(0);
        let seed_range_length: u64 = seeds_parse.remove(0);
        //postMessageToWorker(true, &format!("seed_start: {}, seed_range_length: {}", seed_start, seed_range_length));
        for i in 0..seed_range_length {
            let seed = seed_start + i;
            // postMessageToWorker(true, &format!("Seed: {}", seed));
            //1if seed != 82 {
                //1continue;
            //1}
            let mut current_value: u64 = seed as u64;
            for i in 0..(steps_length-1) {
                let step = steps[i];
                let next_step = steps[i + 1];
                let key = format!("{}-{}", step, next_step);
                if let Some(entry) = almanac.get(&key) {
                    let mut found_mapping = false;
                    for mapping in entry {
                        let applicable_map = is_applicable_map(current_value, mapping);
                        if applicable_map {
                            found_mapping = true;
                            let output_location = compute_output_location(current_value, mapping);
                            //let message = format!("seed: {}, key: {}, source: {:?}, destination: {:?}, length: {:?}, input: {}, output: {} ", seed, key, mapping.get("source_range").unwrap(), mapping.get("destination_range").unwrap(), mapping.get("range_length").unwrap(), current_value, output_location);
                            //postMessageToWorker(true, &message);
                            current_value = output_location;
                            break;
                        } 
                    }
                    if !found_mapping {
                        //let message = format!("seed: {}, key: {}, input: {}, output: {} ", seed, key, current_value, current_value);
                        //postMessageToWorker(true, &message);
                    }
                //postMessageToWorker(true, &format!("Current value after {}: {}", key, current_value));
                }
            }
            //let message = format!("seed: {}, output: {} ", seed, current_value);
            //postMessageToWorker(true, &message);
            final_locations.push(current_value);
        }
    }

    if let Some(min) = final_locations.iter().min() {
        postMessageToWorker(true, &format!("Min: {:?}", min));
    }
}
*/

/*

fn pretty_print_almanac_parse(data: &Vec<u64>, complex_data: &HashMap<String, Vec<HashMap<String, u64>>>) {
    // Serialize and print Vec<u64>
    let serialized_vec = serde_json::to_string_pretty(&data).unwrap();
    postMessageToWorker(true, &format!("Vec<u64>:\n{}", serialized_vec));

    // Create BTreeMap for sorting
    let mut sorted_map: BTreeMap<String, Vec<BTreeMap<String, u64>>> = BTreeMap::new();

    // Iterate over complex_data to populate sorted_map
    for (key, value) in complex_data {
        let mut sorted_submaps = vec![];
        
        // Sorting submaps of complex data
        for submap in value {
            let mut sorted_submap = BTreeMap::new();
            for (subkey, subvalue) in submap {
                sorted_submap.insert(subkey.clone(), *subvalue);
            }
            sorted_submaps.push(sorted_submap);
        }

        sorted_map.insert(key.clone(), sorted_submaps);
    }

    // Serialize and print the sorted map
    let serialized_map = serde_json::to_string_pretty(&sorted_map).unwrap();
    postMessageToWorker(true, &format!("Sorted HashMap<String, Vec<HashMap<String, u64>>>:\n{}", serialized_map));
}

*/