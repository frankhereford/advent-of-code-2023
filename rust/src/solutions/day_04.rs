#![allow(unreachable_code)]
#![allow(unused_variables)]
use web_sys::console;
use wasm_bindgen::prelude::*;
use regex::Regex;

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(do_print: bool, message: &str);
}

pub fn solution_part_1() -> () {
    postMessageToWorker(true, "Part 1: \n");
    let mut iteration = -1;
    let content = include_str!("input/day_04_part_1_test_input.txt");
    // let content = include_str!("input/day_04_input.txt");

    let input_chunks = Regex::new(r"Card (\d+): +(.*) \| +(.*)").unwrap();

    let mut card_scores: Vec<u32> = Vec::new();
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


        if let Some(caps) = input_chunks.captures(line) {
            let card = caps.get(1).map_or("", |m| m.as_str());
            let card_int = card.parse::<u32>().expect("Should be able to parse game");
            let winning_numbers_as_string = caps.get(2).map_or("", |m| m.as_str());
            let our_numbers_as_string = caps.get(3).map_or("", |m| m.as_str());
            let winning_numbers = split_digits_over_whitespace(winning_numbers_as_string);
            let our_numbers = split_digits_over_whitespace(our_numbers_as_string);
            let intersection: Vec<_> = winning_numbers.iter().filter(|&n| our_numbers.contains(n)).collect();
            let intersection_count = intersection.len() as u32;
            let mut card_score = 0;
            if intersection_count > 0 {
                card_score = 2_u32.pow(intersection_count - 1);
            }
            card_scores.push(card_score);
            postMessageToWorker(show_message, &format!("card: {}", card));
            postMessageToWorker(show_message, &format!("winning numbers: {:?}", winning_numbers));
            postMessageToWorker(show_message, &format!("our numbers: {:?}", our_numbers));
            postMessageToWorker(show_message, &format!("our winners: {:?}", intersection));
            postMessageToWorker(show_message, &format!("card score: {:?}", card_score));
        }

    });
    let sum: u32 = card_scores.iter().sum();
    postMessageToWorker(true, &format!("⭐️ Total cards score: {}", sum));
}

fn split_digits_over_whitespace(input: &str) -> Vec<u32> {
    console::log_1(&format!("input: {}", input).into());
    let whitespace = Regex::new(r"\s+").unwrap();
    let parts_as_strings: Vec<&str> = whitespace.split(input).collect();
    let mut found_numbers: Vec<u32> = Vec::new();
    for number in parts_as_strings {
        console::log_1(&format!("number: {}", number).into());
        let number_as_int = number.parse::<u32>().expect("Should be able to parse number");
        found_numbers.push(number_as_int);
    }
    found_numbers
}

pub fn solution_part_2() -> () {
    return;
    postMessageToWorker(true, "Part 2: \n");
    let mut iteration = -1;
    let content = include_str!("input/day_04_part_1_test_input.txt");
    // let content = include_str!("input/day_04_input.txt");

    content.lines().for_each(|line| {
        // Provide a mechanism to limit the volume of output on the console.
        iteration += 1;
        let mut show_message = false;
        if (iteration) %  1== 0  {
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