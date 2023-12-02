//use web_sys::console;
use wasm_bindgen::prelude::*;
use regex::Regex;

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(message: &str);
}

pub fn solution(_n: u32) -> () {
    postMessageToWorker("Concatenate the first and last digits found in a string.\n");

    let content = include_str!("input/day_01_test_input.txt");
    let is_digit_regex  = Regex::new(r"\d").unwrap();

    content.lines().for_each(|line| {
        let characters: Vec<_> = line.chars().collect();
        if characters[0] == '#' {
            //postMessageToWorker("Skipping line because it is a comment.");
            return;
        }

        postMessageToWorker(" ");
        postMessageToWorker(&format!("line: {}", line));
        postMessageToWorker(&format!("characters: {:?}", characters));

        let mut first_digit: Option<&char> = None;
        let mut last_digit: Option<&char> = None;

        for (index, character) in characters.iter().enumerate() {
            if is_digit_regex.is_match(character.to_string().as_str()) {
                postMessageToWorker(&format!("index: {}, character: {}", index, character));
                if first_digit == None {
                    first_digit = Some(character);
                }
                last_digit = Some(character);
            }
        }

        postMessageToWorker(&format!("first_digit: {:?}, last_digit: {:?}", first_digit, last_digit));
        let found_code_string = format!("{}{}", first_digit.unwrap(), last_digit.unwrap());
        let found_code_int = found_code_string.parse::<u32>().unwrap();
        postMessageToWorker(&format!("found_code: {}", found_code_int));

    });
}
