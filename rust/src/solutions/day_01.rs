//use web_sys::console;
use wasm_bindgen::prelude::*;
use regex::Regex;

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(do_print: bool, message: &str);
}

pub fn solution_part_1(_n: u32) -> () {
    postMessageToWorker(true, "Concatenate the first and last digits found in a string.\n");
    let mut iteration = -1;
    // let content = include_str!("input/day_01_test_input.txt");
    let content = include_str!("input/day_01_input.txt");

    let is_digit_regex  = Regex::new(r"\d").unwrap();
    let mut codes: Vec<u32> = Vec::new();

    content.lines().for_each(|line| {
        // Provide a mechanism to limit the volume of output on the console.
        iteration += 1;
        let mut show_message = false;
        if (iteration) % 100 == 0  {
            show_message = true;
        }

        let characters: Vec<_> = line.chars().collect();
        if characters[0] == '#' {
            // postMessageToWorker(show_message, "Skipping line because it is a comment.");
            return;
        }

        postMessageToWorker(show_message, " ");
        postMessageToWorker(show_message, &format!("line: {}", line));

        let mut first_digit: Option<&char> = None;
        let mut last_digit: Option<&char> = None;

        for (_index, character) in characters.iter().enumerate() {
            if is_digit_regex.is_match(character.to_string().as_str()) {
                postMessageToWorker(show_message, &format!("Found a digit: {}", character));
                if first_digit == None {
                    first_digit = Some(character);
                }
                last_digit = Some(character);
            }
        }

        let found_code_string = format!("{}{}", first_digit.unwrap(), last_digit.unwrap());
        let found_code_int = found_code_string.parse::<u32>().unwrap();
        postMessageToWorker(show_message, &format!("found_code: {}", found_code_int));
        codes.push(found_code_int);
    });

    let sum: u32 = codes.iter().sum();
    postMessageToWorker(true, &format!("⭐️ sum: {}", sum));
}
