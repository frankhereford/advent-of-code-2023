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
        postMessageToWorker(" ");
        postMessageToWorker(&format!("line: {}", line));
        let characters: Vec<_> = line.chars().collect();
        //postMessageToWorker(&format!("characters: {:?}", characters));

        let mut first_digit_location: i32 = -1;

        let mut first_digit_location_found = false;
        let mut last_digit_location = 0;
        for (index, character) in characters.iter().enumerate() {
            //postMessageToWorker(&format!("Index: {}, Character: {}", index, character));
            let re = Regex::new(r"\d").unwrap();
            let is_match = re.is_match(character.to_string().as_str());
            if is_match {
                if !first_digit_location_found {
                    postMessageToWorker(&format!("Found first digit: {}", character));
                    first_digit_location = index as i32;
                    first_digit_location_found = true;
                }
                last_digit_location = index;
            }   

            let first_digit = characters[first_digit_location as usize];
            let last_digit = characters[last_digit_location];



            //let sum = first_digit.to_digit(10).unwrap() + last_digit.to_digit(10).unwrap();
            //postMessageToWorker(&format!("Sum: {}", sum));


            //first_digit_location = index;

            //for digit in re.find_iter(line) {
                //postMessageToWorker(&format!("Found digit: {}", digit.as_str()));
            //}
        }
        postMessageToWorker(&format!("First digit location: {}", first_digit_location));
        postMessageToWorker(&format!("Last digit location: {}", last_digit_location));



    });

    "hi".to_string()
}

