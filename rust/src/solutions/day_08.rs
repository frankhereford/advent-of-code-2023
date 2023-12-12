#![allow(unused_variables)]
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
    let content = include_str!("input/day_08_part_1_test_1_input.txt");
    // let content = include_str!("input/day_08_input.txt");
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
    postMessageToWorker(true, &format!("Linkages: {:?}", linkages));
}


pub fn solution_part_2() -> () {
}