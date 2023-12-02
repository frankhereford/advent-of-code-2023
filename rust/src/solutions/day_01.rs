//use web_sys::console;
use wasm_bindgen::prelude::*;
use regex::Regex;

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(do_print: bool, message: &str);
}

pub fn solution(_n: u32) -> () {

    postMessageToWorker(true, "Concatenate the first and last digits found in a string.\n");

    let content = include_str!("input/day_01_test_input.txt");
    let is_digit_regex  = Regex::new(r"\d").unwrap();

    let mut codes: Vec<u32> = Vec::new();

    content.lines().for_each(|line| {
        let characters: Vec<_> = line.chars().collect();
        if characters[0] == '#' {
            // postMessageToWorker(false, "Skipping line because it is a comment.");
            return;
        }

        postMessageToWorker(false, " ");
        postMessageToWorker(false, &format!("line: {}", line));
        // postMessageToWorker(false, &format!("characters: {:?}", characters));

        let mut first_digit: Option<&char> = None;
        let mut last_digit: Option<&char> = None;

        for (_index, character) in characters.iter().enumerate() {
            if is_digit_regex.is_match(character.to_string().as_str()) {
                // postMessageToWorker(false, &format!("index: {}, character: {}", index, character));
                if first_digit == None {
                    first_digit = Some(character);
                }
                last_digit = Some(character);
            }
        }

        // postMessageToWorker(false, &format!("first_digit: {:?}, last_digit: {:?}", first_digit, last_digit));

        let found_code_string = format!("{}{}", first_digit.unwrap(), last_digit.unwrap());
        let found_code_int = found_code_string.parse::<u32>().unwrap();
        postMessageToWorker(false, &format!("found_code: {}", found_code_int));
        codes.push(found_code_int);

    });
    let sum: u32 = codes.iter().sum();
    postMessageToWorker(false, &format!("sum: {}", sum));
}
