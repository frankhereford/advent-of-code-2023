//use web_sys::console;
use wasm_bindgen::prelude::*;
use regex::Regex;

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(do_print: bool, message: &str);
}

pub fn solution_part_1() -> () {
    postMessageToWorker(true, "Part 1: Check game validity by color counts.\n");
    let mut iteration = -1;
    let content = include_str!("input/day_02_part_1_test_input.txt");
    //let content = include_str!("input/day_02_input.txt");

    let input_chunks = Regex::new(r"Game (\d+): (.*)").unwrap();
    let datum_parser = Regex::new(r"(\d+) (\w+)").unwrap();

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


        // how use regex to extract substrings from an input string
        if let Some(caps) = input_chunks.captures(line) {
            postMessageToWorker(show_message, &format!(""));
            let game = caps.get(1).map_or("", |m| m.as_str());
            postMessageToWorker(show_message, &format!("game: {}", game));
            let demonstrations_as_string = caps.get(2).map_or("", |m| m.as_str());
            let demonstrations: Vec<&str> = demonstrations_as_string.split(';').collect();
            for demonstration in demonstrations {
                postMessageToWorker(show_message, &format!("demonstration: {}", demonstration));
                let dice_datum: Vec<&str> = demonstration.split(',').collect();
                for datum in dice_datum {
                    if let Some(caps) = datum_parser.captures(datum) {
                        let count = caps.get(1).map_or("", |m| m.as_str());
                        let color = caps.get(2).map_or("", |m| m.as_str());
                        postMessageToWorker(show_message, &format!("__count: {}, color: {}", count, color));
                    }
                }
            }


        }


    });
}


pub fn solution_part_2() -> () {
    #![allow(unreachable_code)]
    return;
    postMessageToWorker(true, "Part 2: \n");
    let mut iteration = -1;
    let content = include_str!("input/day_02_part_1_test_input.txt");
    //let content = include_str!("input/day_02_input.txt");

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