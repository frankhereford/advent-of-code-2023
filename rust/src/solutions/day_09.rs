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
    return;
    postMessageToWorker(true, "Part 1: \n");
    //let content = include_str!("input/day_09_part_1_test_input.txt");
    let content = include_str!("input/day_09_part_1_input.txt");

    let mut iteration = -1;
    let mut added_values: Vec<i32> = Vec::new();
    content.lines().for_each(|line| {
        iteration += 1;
        postMessageToWorker(true, &format!(""));
        let mut digits: Vec<i32> = line.split_whitespace()
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect();

        let mut pyramid: Vec<Vec<i32>> = Vec::new();
        pyramid.push(digits.clone());
        loop {
            digits = calculate_differences(&digits);
            pyramid.push(digits.clone());
            //postMessageToWorker(true, &format!("{}: {:?}", iteration, digits));
            let is_done = check_all_zeros(&digits);
            if is_done {
                break;
            }
        }

        //postMessageToWorker(true, &format!("{}: {:?}", iteration, pyramid));

        let mut previous_rightmost_value = 0;
        for i in 0..pyramid.len() {
            let n = pyramid.len() - i - 1;
            //postMessageToWorker(true, &format!("{}/ i: {}, n: {}", iteration, i, n));
            if n == pyramid.len() - 1 {
                let row_length = pyramid[n].len();
                pyramid[n].insert(row_length, 0);
            }
            else {
                let rightmost_value_location = pyramid[n].len() - 1; 
                let rightmost_value = pyramid[n].get(rightmost_value_location).unwrap();
                let new_value = rightmost_value + previous_rightmost_value;
                pyramid[n].insert(rightmost_value_location + 1, new_value);
                previous_rightmost_value = new_value;
                if n == 0 {
                    added_values.push(new_value);
                }
            }
            //postMessageToWorker(true, &format!("{}: {:?}", iteration, pyramid[n]));
        }
    });

    let sum: i32 = added_values.iter().sum();
    postMessageToWorker(true, &format!("{}/ {:?}", iteration, added_values));
    postMessageToWorker(true, &format!("{}/ {:?}", iteration, sum));
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
    postMessageToWorker(true, "Part 2: \n");
    let content = include_str!("input/day_09_part_1_test_input.txt");
    let content = include_str!("input/day_09_part_1_input.txt");

    let mut iteration = -1;
    let mut added_values: Vec<i32> = Vec::new();
    content.lines().for_each(|line| {
        iteration += 1;
        postMessageToWorker(true, &format!(""));
        let mut digits: Vec<i32> = line.split_whitespace()
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect();

        let mut pyramid: Vec<Vec<i32>> = Vec::new();
        pyramid.push(digits.clone());
        loop {
            digits = calculate_differences(&digits);
            pyramid.push(digits.clone());
            //postMessageToWorker(true, &format!("{}: {:?}", iteration, digits));
            let is_done = check_all_zeros(&digits);
            if is_done {
                break;
            }
        }

        //postMessageToWorker(true, &format!("{}: {:?}", iteration, pyramid));

        let mut previous_leftmost_value = 0;
        for i in 0..pyramid.len() {
            let n = pyramid.len() - i - 1;
            //postMessageToWorker(true, &format!("{}/ i: {}, n: {}", iteration, i, n));
            if n == pyramid.len() - 1 {
                //let row_length = pyramid[n].len();
                pyramid[n].insert(0, 0);
            }
            else {
                //let rightmost_value_location = pyramid[n].len() - 1; 
                let leftmost_value = pyramid[n].get(0).unwrap();
                let new_value = leftmost_value - previous_leftmost_value;
                pyramid[n].insert(0, new_value);
                previous_leftmost_value = new_value;
                if n == 0 {
                    added_values.push(new_value);
                }
            }
            postMessageToWorker(true, &format!("{}: {:?}", iteration, pyramid[n]));
        }
    });

    let sum: i32 = added_values.iter().sum();
    postMessageToWorker(true, &format!("{}/ {:?}", iteration, added_values));
    postMessageToWorker(true, &format!("{}/ {:?}", iteration, sum));
}