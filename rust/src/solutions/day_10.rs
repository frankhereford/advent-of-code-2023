#![allow(unreachable_code)]
#![allow(unused_variables)]
//use web_sys::console;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(do_print: bool, message: &str);
}

struct Connection {
    cxn_one: Option<(isize, isize)>,
    cxn_two: Option<(isize, isize)>,
    character: char,
}
type Cell = Vec<Connection>;
type Grid = Vec<Cell>;

pub fn solution_part_1() -> () {
    postMessageToWorker(true, "Part 1: \n");
    let mut iteration = 0;
    let content = include_str!("input/day_10_part_1_test_input.txt");
    // let content = include_str!("input/day_10_part_1_input.txt");



    content.lines().for_each(|line| {
        // Provide a mechanism to limit the volume of output on the console.
        let mut show_message = false;
        if (iteration) % 1 == 0  {
            show_message = true;
        }

        let characters: Vec<_> = line.chars().collect();

        postMessageToWorker(show_message, " ");
        postMessageToWorker(show_message, &format!("Iteration: {}, input: {:?}", iteration, characters));

        //for character in characters {
        for (index, character) in characters.iter().enumerate() {
            postMessageToWorker(show_message, &format!("Character: {} / {} / {}", character, iteration, index));

            let index = index as isize; 

            let mut cxn_one: Option<(isize, isize)> = None;
            let mut cxn_two: Option<(isize, isize)> = None;
            match *character {
                'J' => {
                    cxn_one = Some((index, iteration + 1));
                    cxn_two = Some((index - 1, iteration));
                },
                'L' => {
                    cxn_one = Some((index, iteration + 1));
                    cxn_two = Some((index + 1, iteration));
                },
                'F' => {
                    cxn_one = Some((index + 1, iteration));
                    cxn_two = Some((index, iteration - 1));
                },
                '7' => {
                    cxn_one = Some((index - 1, iteration));
                    cxn_two = Some((index, iteration - 1));
                },
                '|' => {
                    cxn_one = Some((index, iteration - 1));
                    cxn_two = Some((index, iteration + 1));
                },
                '-' => {
                    cxn_one = Some((index - 1, iteration));
                    cxn_two = Some((index + 1, iteration));
                },
                _ => {}
            }
            postMessageToWorker(show_message, &format!("letter: {}, cxn_one: {:?}, cxn_two: {:?}", character, cxn_one, cxn_two));
            let mut connection = Connection {
                cxn_one: cxn_one,
                cxn_two: cxn_two,
                character: *character,
            };

        }
    iteration += 1;
    });
}


pub fn solution_part_2() -> () {
    return;
    postMessageToWorker(true, "Part 2: \n");
    let mut iteration = -1;
    let content = include_str!("input/day_10_part_1_test_input.txt");
    // let content = include_str!("input/day_10_part_1_input.txt");

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