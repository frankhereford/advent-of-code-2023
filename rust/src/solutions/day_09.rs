#![allow(unused_variables)]
#![allow(unreachable_code)]
use wasm_bindgen::prelude::*;
//use regex::Regex;
//use std::collections::{HashSet, HashMap};

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(do_print: bool, message: &str);
}

pub fn solution_part_1() -> () {
    postMessageToWorker(true, "Part 1: \n");
    let content = include_str!("input/day_09_part_1_test_input.txt");
    //let content = include_str!("input/day_09_part_1_input.txt");

    let mut iteration = -1;
    content.lines().for_each(|line| {
        iteration += 1;
        let mut digits: Vec<i32> = line.split_whitespace()
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect();

        let mut pyramid: Vec<Vec<i32>> = Vec::new();
        pyramid.push(digits.clone());
        loop {
            digits = calculate_differences(&digits);
            postMessageToWorker(true, &format!("{}: {:?}", iteration, digits));
            let is_done = check_all_zeros(&digits);
            if is_done {
                break;
            }
        }
    });

}

fn calculate_differences(digits: &[i32]) -> Vec<i32> {
    let mut differences: Vec<i32> = Vec::new();
    for i in 0..digits.len() - 1 {
        let difference: i32 = digits[i+1] - digits[i];
        differences.push(difference);
    }
    differences
}

fn check_all_zeros(digits: &[i32]) -> bool {
    let mut all_zeros = true;
    for i in 0..digits.len() {
        if digits[i] != 0 {
            all_zeros = false;
            break;
        }
    }
    all_zeros
}

pub fn solution_part_2() -> () {
    return;
    postMessageToWorker(true, "Part 2: \n");
    let content = include_str!("input/day_09_part_1_test_input.txt");
    //let content = include_str!("input/day_09_part_1_input.txt");
}
