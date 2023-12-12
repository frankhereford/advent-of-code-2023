#![allow(unused_variables)]
#![allow(unreachable_code)]
//use web_sys::console;
use wasm_bindgen::prelude::*;
use regex::Regex;
use std::collections::{HashMap};

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(do_print: bool, message: &str);
}

pub fn solution_part_1() -> () {
    return;
    postMessageToWorker(true, "Part 1: \n");
    //let content = include_str!("input/day_08_part_1_test_1_input.txt");
    let content = include_str!("input/day_08_part_1_input.txt");
    let input = content.lines().collect::<Vec<&str>>();
    let instructions = input[0].chars().collect::<Vec<char>>();
    let mut instruction = instructions.iter().cycle(); // can just access this forever
    let linkage_regex = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
    let mut linkages: HashMap<String, HashMap<String, String>> = HashMap::new();
    for i in 2..input.len() {
        let linkage = input[i];
        let captures = linkage_regex.captures(linkage).unwrap();
        let location = captures.get(1).unwrap().as_str();
        let left = captures.get(2).unwrap().as_str();
        let right = captures.get(3).unwrap().as_str();
        let mut destionations: HashMap<String, String> = HashMap::new();
        destionations.insert("left".to_string(), left.to_string());
        destionations.insert("right".to_string(), right.to_string());
        linkages.insert(location.to_string(), destionations);
    }
    let mut current_location = "AAA";
    let end = "ZZZ";
    let mut move_count = 0;
    loop {
        move_count += 1;
        let next_instruction = instruction.next().unwrap();
        let next_location = match next_instruction {
            &'R' => linkages.get(current_location).unwrap().get("right").unwrap(),
            _ => linkages.get(current_location).unwrap().get("left").unwrap(),
        };
        if move_count % 1000 == 0 {
            postMessageToWorker(true, &format!("move {} - from: {}, via: {}, to: {}", move_count, current_location, next_instruction, next_location));
        }
        if next_location == end {
            postMessageToWorker(true, &format!("Found the end on move: {}", move_count));
            break;
        } else {
            current_location = next_location;
        }
    }
}


pub fn solution_part_2() -> () {
    postMessageToWorker(true, "Part 2: \n");
    let content = include_str!("input/day_08_part_2_test_input.txt");
    //let content = include_str!("input/day_08_part_1_input.txt");
    let input = content.lines().collect::<Vec<&str>>();
    let instructions = input[0].chars().collect::<Vec<char>>();
    let mut instruction = instructions.iter().cycle(); // can just access this forever
    let linkage_regex = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
    let mut linkages: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut starting_locations: Vec<String> = Vec::new();
    for i in 2..input.len() {
        let linkage = input[i];
        let captures = linkage_regex.captures(linkage).unwrap();
        let location = captures.get(1).unwrap().as_str();
        let left = captures.get(2).unwrap().as_str();
        let right = captures.get(3).unwrap().as_str();
        let mut destionations: HashMap<String, String> = HashMap::new();
        destionations.insert("left".to_string(), left.to_string());
        destionations.insert("right".to_string(), right.to_string());
        linkages.insert(location.to_string(), destionations);
        if location.chars().nth(2).unwrap() == 'A' {
            starting_locations.insert(0, location.to_string());
        }
    }
    postMessageToWorker(true, &format!("starting_locations: {:?}", starting_locations));
}