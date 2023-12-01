//use web_sys::console;
use wasm_bindgen::prelude::*;
use regex::Regex;




#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(message: &str);
}

pub fn solution(_n: u32) -> String {
    let content = include_str!("/home/frank/development/advent-of-code-2023/rust/src/solutions/input/day_01_test_input.txt");

    postMessageToWorker("Concatenate the first and last digits found in a string.\n");
    
    content.lines().for_each(|line| {
        postMessageToWorker(&format!("line: {}", line));

        let re = Regex::new(r"\d").unwrap();

        if let Some(cap) = re.captures(line) {
            let matched_char = &cap[0];
            postMessageToWorker(&format!("Found character: {}", matched_char));
        } 

    });

    "hi".to_string()
}

