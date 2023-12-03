//use web_sys::console;
use regex::Regex;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(do_print: bool, message: &str);
}

pub fn solution_part_1() -> () {
    postMessageToWorker(
        true,
        "Part 1: Concatenate the first and last digits found in a string.\n",
    );
    let mut iteration = -1;
    // let content = include_str!("input/day_01_part_1_test_input.txt");
    let content = include_str!("input/day_01_input.txt");

    let is_digit_regex = Regex::new(r"\d").unwrap();
    let mut codes: Vec<u32> = Vec::new();

    content.lines().for_each(|line| {
        // Provide a mechanism to limit the volume of output on the console.
        iteration += 1;
        let mut show_message = false;
        if (iteration) % 300 == 0 {
            show_message = true;
        }

        let characters: Vec<_> = line.chars().collect();
        if characters[0] == '#' {
            // postMessageToWorker(show_message, "Skipping line because it is a comment.");
            return;
        }

        postMessageToWorker(show_message, " ");
        postMessageToWorker(
            show_message,
            &format!("Iteration: {}, input: {}", iteration, line),
        );

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

pub fn solution_part_2() -> () {
    postMessageToWorker(
        true,
        "Part 2: Concatenate the first and last digits found in a string.\n",
    );
    let mut iteration = -1;
    // let content = include_str!("input/day_01_part_2_test_input.txt");
    let content = include_str!("input/day_01_input.txt");

    let is_digit_regex = Regex::new(r"\d").unwrap();
    let mut codes: Vec<u32> = Vec::new();

    content.lines().for_each(|line| {
        // Provide a mechanism to limit the volume of output on the console.
        iteration += 1;
        let mut show_message = false;
        if (iteration) % 300 == 0 {
            show_message = true;
        }

        let characters: Vec<_> = line.chars().collect();
        if characters[0] == '#' {
            // postMessageToWorker(show_message, "Skipping line because it is a comment.");
            return;
        }

        postMessageToWorker(show_message, " ");
        postMessageToWorker(
            show_message,
            &format!("Iteration: {}, input: {}", iteration, line),
        );

        let mut first_digit: Option<char> = None;
        let mut last_digit: Option<char> = None;

        let numbers = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        for (character_index, character) in characters.iter().enumerate() {
            if is_digit_regex.is_match(character.to_string().as_str()) {
                if first_digit == None {
                    let temp_string = character.to_string();
                    let char = temp_string.chars().nth(0).unwrap();
                    first_digit = Some(char);
                    postMessageToWorker(
                        show_message,
                        &format!("New first digit from numeric character: {}", character),
                    );
                }
                let temp_string = character.to_string();
                let char = temp_string.chars().nth(0).unwrap();
                last_digit = Some(char);
                postMessageToWorker(
                    show_message,
                    &format!("New last digit from numeric character: {}", character),
                );
            }
            for (number_index, number) in numbers.iter().enumerate() {
                let length = number.len();
                let slice = get_string_slice(line, character_index, length);
                // postMessageToWorker(show_message, &format!("number: {}, slice: {}", number, slice));
                if &slice == number {
                    let found_scalar = number_index + 1;

                    if first_digit.is_none() {
                        let temp_string = found_scalar.to_string();
                        let char = temp_string.chars().nth(0).unwrap();
                        first_digit = Some(char);
                        postMessageToWorker(
                            show_message,
                            &format!("Found first digit: {}", first_digit.unwrap()),
                        );
                    }

                    let temp_string = found_scalar.to_string();
                    let char = temp_string.chars().nth(0).unwrap();
                    last_digit = Some(char);
                    postMessageToWorker(
                        show_message,
                        &format!(
                            "New last digit from number ({}): {}",
                            number,
                            last_digit.unwrap()
                        ),
                    );
                }
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

fn get_string_slice(input: &str, start: usize, chars: usize) -> &str {
    let start_index = input.char_indices().nth(start);
    let end_index = input
        .char_indices()
        .nth(start + chars)
        .unwrap_or((input.len(), ' '))
        .0;

    match start_index {
        Some((start_idx, _)) => &input[start_idx..end_index],
        None => "",
    }
}
