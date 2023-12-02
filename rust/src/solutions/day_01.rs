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

        //let mut first_digit = ' ';
        //let mut first_digit_location: i32 = -1;

        //let mut last_digit = ' ';
        //let mut last_digit_location: i32 = -1;


    });

}

