#![allow(unused_variables)]
#![allow(unreachable_code)]
//use web_sys::console;
use wasm_bindgen::prelude::*;
use regex::Regex;
use std::collections::{HashSet, HashMap};

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(do_print: bool, message: &str);
}

pub fn solution_part_1() -> () {
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
        let instruction = instruction.next().unwrap();
        let next_location = match instruction {
            &'R' => linkages.get(current_location).unwrap().get("right").unwrap(),
            _ => linkages.get(current_location).unwrap().get("left").unwrap(),
        };
        if move_count % 2000 == 0 {
            postMessageToWorker(true, &format!("move {} - from: {}, via: {}, to: {}", move_count, current_location, instruction, next_location));
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
    //let content = include_str!("input/day_08_part_2_test_input.txt");
    let content = include_str!("input/day_08_part_1_input.txt");
    let input = content.lines().collect::<Vec<&str>>();
    let instructions = input[0].chars().collect::<Vec<char>>();
    let linkage_regex = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
    let mut linkages: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut paths: Vec<String> = Vec::new();
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
            paths.insert(0, location.to_string());
        }
    }
    postMessageToWorker(true, &format!("paths: {:?}", paths));
    let mut path_distances: Vec<u64> = Vec::new();
    for i in 0..paths.len() {
        postMessageToWorker(true, &format!("paths[{}]: {:?}", i, paths[i]));
        let mut move_count = 0;
        let mut instruction = instructions.iter().cycle();
        let mut current_location = &paths[i];
        loop {
            move_count += 1;
            let instruction = instruction.next().unwrap();
            let next_location = match instruction {
                &'R' => linkages.get(current_location).unwrap().get("right").unwrap(),
                _ => linkages.get(current_location).unwrap().get("left").unwrap(),
            };
            if next_location.chars().nth(2).unwrap() == 'Z' {
                postMessageToWorker(true, &format!("found end on move {} - from: {}, via: {}, to: {}", move_count, current_location, instruction, next_location));
                path_distances.push(move_count);
            break;
            }
            current_location = next_location;
        }
    }
    postMessageToWorker(true, &format!("path_distances: {:?}", path_distances));

    let mut common_factors: HashSet<u64> = HashSet::new();
    for distance in path_distances {
        let factors = factorize(distance);
        for factor in factors {
            common_factors.insert(factor);
        }
    }
    let product: u64 = common_factors.iter().product();
    postMessageToWorker(true, &format!("common_factors: {:?}, product: {}", common_factors, product));
}

fn factorize(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut divisor = 2;

    while n > 1 {
        while n % divisor == 0 {
            factors.push(divisor);
            n /= divisor;
        }
        divisor += 1;
    }

    factors
}
