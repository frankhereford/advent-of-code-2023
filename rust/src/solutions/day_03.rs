//use web_sys::console;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(do_print: bool, message: &str);
}

/* Twist prediction:

Part one is about numbers adjacent to symbols. Part two is about symbols adjacent to numbers.
Meaning, part two will use the symbol's meaning to determine the operator applied to the numbers.
The quality of the parsing routine's output data structure will make part two easier or harder.

*/

#[derive(Clone)]
enum SchematicElement {
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
    // let content = include_str!("input/day_03_part_1_test_input.txt");
    let content = include_str!("input/day_03_input.txt");

    let mut schematic: Vec<Vec<SchematicElement>> = Vec::new();

    content.lines().for_each(|line| {
        // Provide a mechanism to limit the volume of output on the console.
        iteration += 1;
        let show_message = false;

        let characters: Vec<_> = line.chars().collect();
        if characters[0] == '`' {
            // defining a special comment character for today
            return;
        }

        postMessageToWorker(show_message, " ");
        postMessageToWorker(
            show_message,
            &format!("Iteration: {}, input: {}", iteration, line),
        );

        parse_schematic_line(
            &mut schematic,
            iteration,
            line.to_string(),
            characters,
            show_message,
        );
    });
    //postMessageToWorker(true, &format!("Schematic: {:?}", schematic));

    let mut part_numbers: Vec<u32> = Vec::new();

    calculate_sum_of_symbol_adjacent_parts(&schematic, &mut part_numbers);

    let sum: u32 = part_numbers.iter().sum();
    postMessageToWorker(true, &format!("⭐️ part number sum: {}", sum));
}

fn calculate_sum_of_symbol_adjacent_parts(
    schematic: &Vec<Vec<SchematicElement>>,
    part_numbers: &mut Vec<u32>,
) {
    let mut symbols_found: u32 = 0;
    for (i, row) in schematic.iter().enumerate() {
        for (j, element) in row.iter().enumerate() {
            match element {
                SchematicElement::Number(_value) => {}
                SchematicElement::Void(_is_void) => {}
                SchematicElement::Symbol(symbol) => {
                    let mut show_message = false;
                    if symbols_found % 200 == 0 {
                        show_message = true;
                    }

                    // ! huge help: there are never symbols along the outside of the schematic

                    postMessageToWorker(
                        show_message,
                        &format!(
                            "Found a symbol, #{} to work '{}' at ({}, {})",
                            symbols_found, symbol, i, j
                        ),
                    );

                    symbols_found += 1;

                    let mut neighbors = HashMap::new();

                    neighbors.insert("up", &schematic[i - 1][j]);
                    neighbors.insert("down", &schematic[i + 1][j]);
                    neighbors.insert("left", &schematic[i][j - 1]);
                    neighbors.insert("right", &schematic[i][j + 1]);

                    match neighbors.get("up") {
                        // i really like this match thing, this is super safe
                        Some(&SchematicElement::Number(_value)) => {}
                        _ => {
                            //postMessageToWorker(show_message, &format!("up is not a number"));
                            neighbors.insert("up_left", &schematic[i - 1][j - 1]);
                            neighbors.insert("up_right", &schematic[i - 1][j + 1]);
                        }
                    }

                    match neighbors.get("down") {
                        Some(&SchematicElement::Number(_value)) => {}
                        _ => {
                            //postMessageToWorker(show_message, &format!("down is not a number"));
                            neighbors.insert("down_left", &schematic[i + 1][j - 1]);
                            neighbors.insert("down_right", &schematic[i + 1][j + 1]);
                        }
                    }

                    postMessageToWorker(show_message, &format!("neighbors: {:?}", neighbors));
                    // we're going to make a huge assumption here: that a number is never
                    // touched by more than one symbol. the input data looks safe. this better
                    // not be the twist!! 😅
                    for (_neighbor_name, neighbor) in neighbors.iter() {
                        match neighbor {
                            SchematicElement::Number(value) => {
                                postMessageToWorker(
                                    show_message,
                                    &format!(
                                        "Found a neighbor number '{}' at ({}, {})",
                                        value, i, j
                                    ),
                                );
                                part_numbers.push(*value);
                            }
                            _ => {} // don't care about other things we find here
                        }
                    }
                }
            }
        }
    }
}

fn parse_schematic_line(
    schematic: &mut Vec<Vec<SchematicElement>>,
    line_number: i32,
    line: String,
    characters: Vec<char>,
    show_message: bool,
) -> () {
    // initialize the schematic line
    schematic.push(Vec::new());
    let line_length = line.len();
    for _ in 0..line_length {
        schematic[line_number as usize].push(SchematicElement::Void(false));
    }

    let is_digit_regex = Regex::new(r"\d").unwrap();
    let is_blank_regex = Regex::new(r"\.").unwrap();

    let mut is_in_number = false;
    let mut number_location = 0;
    let mut number_as_string = String::new();
    for (index, character) in characters.iter().enumerate() {
        if is_digit_regex.is_match(character.to_string().as_str()) {
            //postMessageToWorker(show_message, &format!("Found a digit: {}", character));
            number_as_string.push(*character);
            if !is_in_number {
                is_in_number = true;
                number_location = index;
            }
        } else {
            // either a blank or a symbol
            if is_in_number {
                handle_found_number(
                    schematic,
                    line_number,
                    number_location,
                    number_as_string,
                    show_message,
                );
            }
            is_in_number = false;
            number_as_string = String::new();
            if !is_blank_regex.is_match(character.to_string().as_str()) {
                //postMessageToWorker(show_message, &format!("Found a symbol: {}", character));
                schematic[line_number as usize][index] =
                    SchematicElement::Symbol(character.to_owned());
            }
        }
    }
    if is_in_number {
        handle_found_number(
            schematic,
            line_number,
            number_location,
            number_as_string,
            show_message,
        );
    }
}

fn handle_found_number(
    schematic: &mut Vec<Vec<SchematicElement>>,
    line_number: i32,
    number_location: usize,
    number_as_string: String,
    _show_message: bool,
) -> () {
    let number_length = number_as_string.len();
    for index in 0..number_location + number_length {
        if index >= number_location && index < (number_location + number_length) {
            //postMessageToWorker( show_message, &format!( "Setting a number at {},{}: {}", line_number, index, number_as_string),);
            schematic[line_number as usize][index] =
                SchematicElement::Number(number_as_string.parse::<u32>().unwrap());
        }
    }
}

pub fn solution_part_2() -> () {
    postMessageToWorker(true, "Part 2: Sumation of gear ratio scalars.\n");
    let mut iteration = -1;
    //let content = include_str!("input/day_03_part_1_test_input.txt");
    let content = include_str!("input/day_03_input.txt");

    let mut schematic: Vec<Vec<SchematicElement>> = Vec::new();

    content.lines().for_each(|line| {
        iteration += 1;
        let show_message = false;

        let characters: Vec<_> = line.chars().collect();
        if characters[0] == '`' {
            // defining a special comment character for today
            return;
        }

        postMessageToWorker(show_message, " ");
        postMessageToWorker(
            show_message,
            &format!("Iteration: {}, input: {}", iteration, line),
        );

        parse_schematic_line(
            &mut schematic,
            iteration,
            line.to_string(),
            characters,
            show_message,
        );
    });
    //postMessageToWorker(true, &format!("Schematic: {:?}", schematic));

    let mut gear_ratios: Vec<u32> = Vec::new();

    calculate_sum_of_gear_ratios(&schematic, &mut gear_ratios);

    let sum: u32 = gear_ratios.iter().sum();
    postMessageToWorker(true, &format!("⭐️ gear ratio sum: {}", sum));
}

fn calculate_sum_of_gear_ratios(
    schematic: &Vec<Vec<SchematicElement>>,
    gear_ratios: &mut Vec<u32>,
) {
    let mut symbols_found: u32 = 0;
    for (i, row) in schematic.iter().enumerate() {
        for (j, element) in row.iter().enumerate() {
            match element {
                SchematicElement::Number(_value) => {}
                SchematicElement::Void(_is_void) => {}
                SchematicElement::Symbol(symbol) => {
                    // ! huge help: there are never symbols along the outside of the schematic
                    if *symbol != '*' {
                        // only care about the gear symbols
                        continue;
                    }

                    let mut show_message = false;
                    if symbols_found % 100 == 0 {
                        show_message = true;
                    }

                    postMessageToWorker(
                        show_message,
                        &format!(
                            "Found a symbol, #{} to work '{}' at ({}, {})",
                            symbols_found, symbol, i, j
                        ),
                    );

                    symbols_found += 1;

                    let mut neighbors = HashMap::new();

                    neighbors.insert("up", &schematic[i - 1][j]);
                    neighbors.insert("down", &schematic[i + 1][j]);
                    neighbors.insert("left", &schematic[i][j - 1]);
                    neighbors.insert("right", &schematic[i][j + 1]);

                    match neighbors.get("up") {
                        // i really like this match thing, this is super safe
                        Some(&SchematicElement::Number(_value)) => {}
                        _ => {
                            //postMessageToWorker(show_message, &format!("up is not a number"));
                            neighbors.insert("up_left", &schematic[i - 1][j - 1]);
                            neighbors.insert("up_right", &schematic[i - 1][j + 1]);
                        }
                    }

                    match neighbors.get("down") {
                        Some(&SchematicElement::Number(_value)) => {}
                        _ => {
                            //postMessageToWorker(show_message, &format!("down is not a number"));
                            neighbors.insert("down_left", &schematic[i + 1][j - 1]);
                            neighbors.insert("down_right", &schematic[i + 1][j + 1]);
                        }
                    }

                    postMessageToWorker(show_message, &format!("neighbors: {:?}", neighbors));
                    let mut neighbor_numbers: Vec<u32> = Vec::new();
                    for (_neighbor_name, neighbor) in neighbors.iter() {
                        match neighbor {
                            SchematicElement::Number(value) => {
                                postMessageToWorker(
                                    show_message,
                                    &format!(
                                        "Found a neighbor number '{}' at ({}, {})",
                                        value, i, j
                                    ),
                                );
                                neighbor_numbers.push(*value);
                            }
                            _ => {} // don't care about other things we find here
                        }
                    }
                    if neighbor_numbers.len() == 2 {
                        // some gears only touch one number
                        let product: u32 = neighbor_numbers.iter().product();
                        postMessageToWorker(
                            show_message,
                            &format!("Found a product of {} for gear at ({}, {})", product, i, j),
                        );
                        gear_ratios.push(product);
                    }
                }
            }
        }
    }
}
