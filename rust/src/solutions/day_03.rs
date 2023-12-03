#![allow(unreachable_code)]
//use web_sys::console;
use wasm_bindgen::prelude::*;
use regex::Regex;
use std::fmt;


#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(do_print: bool, message: &str);
}

/* Twist prediction:

Part one is about numbers adjacent to symbols. Part two is about symbols adjacent to numbers.
Meaning, part two will use the symbol's meaning to determine the operator applied to the numbers.
The quality of the parsing routine's output data structure will make part two easier or harder.

*/

enum SchematicElement{
    Number(u32),
    Symbol(char),
    Void(bool),
}

impl fmt::Debug for SchematicElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SchematicElement::Number(num) => write!(f, "Number({})", num),
            SchematicElement::Symbol(ch) => write!(f, "Symbol({})", ch),
            SchematicElement::Void(_bool) => write!(f, "Void()"),
        }
    }
}

pub fn solution_part_1() -> () {
    postMessageToWorker(true, "Part 1: Sumation of symbol adjacent scalars.\n");
    let mut iteration = -1;
    let content = include_str!("input/day_03_part_1_test_input.txt");
    // let content = include_str!("input/day_XX_input.txt");

    let mut schematic: Vec<Vec<SchematicElement>> = Vec::new();

    content.lines().for_each(|line| {
        // Provide a mechanism to limit the volume of output on the console.
        iteration += 1;
        let show_message = false;
        // if (iteration) % 1000 == 0  {
            // show_message = true;
        // }

        let characters: Vec<_> = line.chars().collect();
        if characters[0] == '`' { // defining a special comment character for today
            // postMessageToWorker(show_message, "Skipping line because it is a comment.");
            return;
        }

        postMessageToWorker(show_message, " ");
        postMessageToWorker(show_message, &format!("Iteration: {}, input: {}", iteration, line));

        parse_schematic_line(&mut schematic, iteration, line.to_string(), characters, show_message);
    });
    postMessageToWorker(true, &format!("Schematic: {:?}", schematic));


    for (i, row) in schematic.iter().enumerate() {
        for (j, element) in row.iter().enumerate() {
            postMessageToWorker(true, &format!("{:?}", element));

            match element {
                SchematicElement::Number(value) => {
                    postMessageToWorker(true, &format!("Found a number {} at ({}, {})", value, i, j));
                }
                SchematicElement::Symbol(symbol) => {
                    postMessageToWorker(true, &format!("Found a symbol '{}' at ({}, {})", symbol, i, j));
                }
                SchematicElement::Void(is_void) => {
                }
            }
        }
    }
}

fn parse_schematic_line(schematic: &mut Vec<Vec<SchematicElement>>, line_number: i32, line: String, characters: Vec<char>, show_message: bool) -> () {
    // initialize the schematic line
    schematic.push(Vec::new());
    let line_length = line.len();
    for _ in 0..line_length {
        schematic[line_number as usize].push(SchematicElement::Void(false));
    }


    let is_digit_regex  = Regex::new(r"\d").unwrap();
    let is_blank_regex  = Regex::new(r"\.").unwrap();


    let mut is_in_number = false;
    let mut number_location = 0;
    let mut number_as_string = String::new();
    for (index, character) in characters.iter().enumerate() {
        if is_digit_regex.is_match(character.to_string().as_str()) {
            postMessageToWorker(show_message, &format!("Found a digit: {}", character));
            number_as_string.push(*character);
            if !is_in_number {
                is_in_number = true;
                number_location = index;
            }
        } else { // either a blank or a symbol
            if is_in_number {
                handle_found_number(schematic, line_number, number_location, number_as_string, show_message);
            }
            is_in_number = false;
            number_as_string = String::new();
            if !is_blank_regex.is_match(character.to_string().as_str())  {
                postMessageToWorker(show_message, &format!("Found a symbol: {}", character));
                schematic[line_number as usize][index] = SchematicElement::Symbol(character.to_owned());
            } 
        }
    }
    if is_in_number {
        handle_found_number(schematic, line_number, number_location, number_as_string, show_message);
    }
}

fn handle_found_number(schematic: &mut Vec<Vec<SchematicElement>>, line_number: i32, number_location: usize, number_as_string: String, show_message: bool) -> () {
    let number_length = number_as_string.len();
    for index in 0..number_location + number_length {
        if index >= number_location && index < (number_location + number_length) {
            postMessageToWorker(show_message, &format!("Setting a number at {},{}: {}", line_number, index, number_as_string));
            schematic[line_number as usize][index] = SchematicElement::Number(number_as_string.parse::<u32>().unwrap());
        }
    }
}









pub fn solution_part_2() -> () {
    return;
    postMessageToWorker(true, "Part 2: \n");
    let mut iteration = -1;
    let content = include_str!("input/day_03_part_1_test_input.txt");
    // let content = include_str!("input/day_XX_input.txt");

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