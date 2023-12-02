//use web_sys::console;
use wasm_bindgen::prelude::*;
// use regex::Regex;

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(message: &str);
}

pub fn solution(_n: u32) -> () {
    let content = include_str!("input/day_01_test_input.txt");

    postMessageToWorker("Concatenate the first and last digits found in a string.\n");
    
    content.lines().for_each(|line| {
        let characters: Vec<_> = line.chars().collect();
        if characters[0] == '#' {
            //postMessageToWorker("Skipping line because it is a comment.");
            return;
        }

        postMessageToWorker(" ");
        postMessageToWorker(&format!("line: {}", line));
        postMessageToWorker(&format!("characters: {:?}", characters));

        let mut first_digit = ' ';
        let mut first_digit_location: i32 = -1;

        let mut last_digit = ' ';
        let mut last_digit_location: i32 = -1;


        /*
        for (index, character) in characters.iter().enumerate() {
            let re = Regex::new(r"\d").unwrap();
            let is_match = re.is_match(character.to_string().as_str());
            if is_match {
                if first_digit_location < 0 {
                    postMessageToWorker(&format!("Found first digit: {}", character));
                    first_digit_location = index as i32;
                }
                last_digit_location = index;
            }   

            let first_digit = characters[first_digit_location as usize];
            let last_digit = characters[last_digit_location as usize];

        }
        */

        //postMessageToWorker(&format!("First digit, {}, found at location: {}", first_digit, first_digit_location));
        //postMessageToWorker(&format!("Last digit, {}, found at location: {}", last_digit, last_digit_location));
    });

}

