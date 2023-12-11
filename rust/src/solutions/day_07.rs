#![allow(unreachable_code)]
#![allow(unused_variables)]
//use web_sys::console;
use wasm_bindgen::prelude::*;

use regex::Regex;
use radix_fmt::radix;


#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(do_print: bool, message: &str);
}

type Card = char;
type Hand = [Card; 5];
type Wager = u32;
type ScoringClassification = (String, String);
type CardHand = (Hand, Wager, ScoringClassification);
type CardHands = Vec<CardHand>;

pub fn solution_part_1() -> () {
    postMessageToWorker(true, "Part 1: \n");
    let mut iteration = -1;
    let content = include_str!("input/day_07_test_input.txt");
    // let content = include_str!("input/day_07_input.txt");

    let hand_regex = Regex::new(r"(\w{5}) (\d+)").unwrap();

    content.lines().for_each(|line| {
        // Provide a mechanism to limit the volume of output on the console.
        iteration += 1;
        let mut show_message = false;
        if (iteration) % 1 == 0  {
            show_message = true;
        }

        let characters: Vec<_> = line.chars().collect();
        if characters[0] == '#' {
            // postMessageToWorker(show_message, "Skipping line because it is a comment.");
            return;
        }

        postMessageToWorker(show_message, " ");
        postMessageToWorker(show_message, &format!("Iteration: {}, input: {}", iteration, line));

        if let Some(caps) = hand_regex.captures(line) {
            let cards = caps.get(1).map_or("", |m| m.as_str().trim());
            let wager = caps.get(2).map_or("", |m| m.as_str().trim());
        }

    });
}


pub fn solution_part_2() -> () {
    return;
    postMessageToWorker(true, "Part 2: \n");
    let mut iteration = -1;
    let content = include_str!("input/day_07_test_input.txt");
    // let content = include_str!("input/day_07_input.txt");

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