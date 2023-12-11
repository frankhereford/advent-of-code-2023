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
    //let content = include_str!("input/day_06_test_input.txt");
    let content = include_str!("input/day_06_input.txt");

    let time_line = Regex::new(r"Time:(.*)").unwrap();
    let distance_line = Regex::new(r"Distance:(.*)").unwrap();

    let mut times: Vec<u32> = Vec::new();
    if let Some(caps) = time_line.captures(content) {
        let times_as_string = caps.get(1).map_or("", |m| m.as_str().trim());
        times = split_digits_over_whitespace(times_as_string);
    }

    let mut distances: Vec<u32> = Vec::new();
    if let Some(caps) = distance_line.captures(content) {
        let distances_as_string = caps.get(1).map_or("", |m| m.as_str().trim());
        distances = split_digits_over_whitespace(distances_as_string);
    }

    //postMessageToWorker(true, &format!("Times: {:?}", times));
    //postMessageToWorker(true, &format!("Distances: {:?}", distances));

    let mut margins: Vec<u32> = Vec::new();
    for race in 0..times.len() {
        let duration = times[race];
        let distance = distances[race];
        postMessageToWorker(true, &format!("duration: {}, distance: {}", duration, distance));
        let mut count_of_winners = 0;
        for milisecond in 0..duration+1 {
            let distance_traveled = milisecond * (duration - milisecond);
            if distance_traveled > distance {
                count_of_winners += 1;
            }
            //postMessageToWorker(true, &format!("milisecond: {}, distance_traveled: {}", milisecond, distance_traveled));
        }
        postMessageToWorker(true, &format!("count_of_winners: {}", count_of_winners));
        margins.push(count_of_winners);
    }
    let margins_product: u32 = margins.iter().product();

    postMessageToWorker(true, &format!("Product of margins_product: {}", margins_product));
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
    postMessageToWorker(true, "Part 1: \n");
    //let content = include_str!("input/day_06_part_2_test_input.txt");
    let content = include_str!("input/day_06_part_2_input.txt");

    let time_line = Regex::new(r"Time:(.*)").unwrap();
    let distance_line = Regex::new(r"Distance:(.*)").unwrap();

    let mut times: Vec<u64> = Vec::new();
    if let Some(caps) = time_line.captures(content) {
        let times_as_string = caps.get(1).map_or("", |m| m.as_str().trim());
        times = split_digits_over_whitespace_64(times_as_string);
    }

    let mut distances: Vec<u64> = Vec::new();
    if let Some(caps) = distance_line.captures(content) {
        let distances_as_string = caps.get(1).map_or("", |m| m.as_str().trim());
        distances = split_digits_over_whitespace_64(distances_as_string);
    }

    //postMessageToWorker(true, &format!("Times: {:?}", times));
    //postMessageToWorker(true, &format!("Distances: {:?}", distances));

    let mut margins: Vec<u64> = Vec::new();
    for race in 0..times.len() {
        let duration = times[race];
        let distance = distances[race];
        postMessageToWorker(true, &format!("duration: {}, distance: {}", duration, distance));
        let mut count_of_winners = 0;
        for milisecond in 0..duration+1 {
            let distance_traveled = milisecond * (duration - milisecond);
            if distance_traveled > distance {
                count_of_winners += 1;
            }
            //postMessageToWorker(true, &format!("milisecond: {}, distance_traveled: {}", milisecond, distance_traveled));
        }
        postMessageToWorker(true, &format!("count_of_winners: {}", count_of_winners));
        margins.push(count_of_winners);
    }
    let margins_product: u64 = margins.iter().product();

    postMessageToWorker(true, &format!("Product of margins_product: {}", margins_product));
}

fn split_digits_over_whitespace_64(input: &str) -> Vec<u64> {
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
