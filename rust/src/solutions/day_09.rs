#![allow(unused_variables)]
#![allow(unreachable_code)]
use wasm_bindgen::prelude::*;
//use regex::Regex;
//use std::collections::{HashSet, HashMap};
use polyfit_rs::polyfit_rs::polyfit;

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(do_print: bool, message: &str);
}

pub fn solution_part_1() -> () {
    postMessageToWorker(true, "Part 1: \n");
    //let content = include_str!("input/day_09_part_1_test_input.txt");
    let content = include_str!("input/day_09_part_1_input.txt");

    let mut iteration = -1;
    let mut results: Vec<f64> = Vec::new();
    content.lines().for_each(|line| {
        iteration += 1;
        let y_values: Vec<f64> = line.split_whitespace()
                                .map(|s| s.parse::<f64>().unwrap())
                                .collect();
        let x_values: Vec<f64> = (0..y_values.len()).map(|x| x as f64).collect();

        match polyfit(&x_values, &y_values, x_values.len() - 1) {
            Ok(coefficients) => {
                let mut meaningful_coefficients: Vec<f64> = Vec::new();
                for coefficient in coefficients {
                    //postMessageToWorker(true, &format!("{} / coefficient: {:?}", iteration, coefficient));
                    if coefficient > 0.01 {
                        //postMessageToWorker(true, &format!("{} / passing coefficient: {:?}", iteration, coefficient));
                        meaningful_coefficients.push(coefficient);
                    } else {
                        //postMessageToWorker(true, &format!("{} / zeroing coefficient: {:?}", iteration, 0.0));
                        meaningful_coefficients.push(0.0);
                    }
                    
                //postMessageToWorker(true, &format!("{} / coefficients: {:?}", iteration, meaningful_coefficients));
                }

                //let degree = meaningful_coefficients.len() - 1;
                //postMessageToWorker(true, &format!("{} / degree: {:?}", iteration, degree));
                let mut found_non_zero_coefficient = false;
                let mut non_zero_coefficients: Vec<f64> = Vec::new();
                for coefficient in meaningful_coefficients.iter().rev() {
                        if *coefficient > 0.0 || found_non_zero_coefficient {
                            found_non_zero_coefficient = true;
                            non_zero_coefficients.push(*coefficient);
                        }
                }
                let non_zero_coefficients = non_zero_coefficients.iter().rev().cloned().collect::<Vec<f64>>();

                postMessageToWorker(true, &format!("{} / non_zero_coefficients: {:?}", iteration, non_zero_coefficients));

                let result_as_float = compute_polynomial(x_values.len() as u64, non_zero_coefficients);
                let result = round_to_precision(result_as_float,2) as f64;
                results.push(result);
                postMessageToWorker(true, &format!("{} / result: {:?}", iteration, result));
            },
            Err(e) => { postMessageToWorker(true, &format!("Error: {}", e)); }
        }
    });
    let sum = results.iter().sum::<f64>();
    postMessageToWorker(true, &format!("Sum: {}", sum));
}

fn compute_polynomial(x: u64, coefficients: Vec<f64>) -> f64 {
    let mut result: f64 = 0.0;
    for n in 0..coefficients.len() {
        result += coefficients[n] * x.pow(n as u32) as f64;
    }
    result
}


fn round_to_precision(num: f64, precision: u32) -> f64 {
    let multiplier = 10f64.powi(precision as i32);
    (num * multiplier).round() / multiplier
}


pub fn solution_part_2() -> () {
    return;
    postMessageToWorker(true, "Part 2: \n");
    let content = include_str!("input/day_09_part_1_test_input.txt");
    //let content = include_str!("input/day_09_part_1_input.txt");
}
