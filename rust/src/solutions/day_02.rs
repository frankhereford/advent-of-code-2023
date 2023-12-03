//use web_sys::console;
#![allow(unreachable_code)]
use regex::Regex;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(do_print: bool, message: &str);
}

pub fn solution_part_1() -> () {
    postMessageToWorker(true, "Part 1: Check game validity by color counts.\n");
    let mut iteration = -1;
    //let content = include_str!("input/day_02_part_1_test_input.txt");
    let content = include_str!("input/day_02_input.txt");

    let input_chunks = Regex::new(r"Game (\d+): (.*)").unwrap();
    let datum_parser = Regex::new(r"(\d+) (\w+)").unwrap();

    let mut possible_games: Vec<u32> = Vec::new();

    content.lines().for_each(|line| {
        // Provide a mechanism to limit the volume of output on the console.
        iteration += 1;
        let mut show_message = false;
        if (iteration) % 30 == 0 {
            show_message = true;
        }

        let characters: Vec<_> = line.chars().collect();
        if characters[0] == '#' {
            // postMessageToWorker(show_message, "Skipping line because it is a comment.");
            return;
        }

        postMessageToWorker(show_message, " ");
        postMessageToWorker(
            show_message,
            &format!("Iteration: {}, input: {}", iteration, line),
        );

        let mut proposed_setup = HashMap::new();
        proposed_setup.insert("red", 12);
        proposed_setup.insert("green", 13);
        proposed_setup.insert("blue", 14);

        if let Some(caps) = input_chunks.captures(line) {
            postMessageToWorker(show_message, &format!(""));
            let game = caps.get(1).map_or("", |m| m.as_str());
            let game_int = game.parse::<u32>().expect("Should be able to parse game");

            let mut demonstration_knowledge = HashMap::new();

            // postMessageToWorker(show_message, &format!("game: {}", game));
            let demonstrations_as_string = caps.get(2).map_or("", |m| m.as_str());
            let demonstrations: Vec<&str> = demonstrations_as_string.split(';').collect();
            for demonstration in demonstrations {
                // postMessageToWorker(show_message, &format!("demonstration: {}", demonstration));
                let dice_datum: Vec<&str> = demonstration.split(',').collect();
                for datum in dice_datum {
                    if let Some(caps) = datum_parser.captures(datum) {
                        let count = caps.get(1).map_or("", |m| m.as_str());
                        let color = caps.get(2).map_or("", |m| m.as_str());
                        // postMessageToWorker(show_message, &format!("__count: {}, color: {}", count, color));

                        // added .expect()s with copilot here
                        if demonstration_knowledge.contains_key(color) {
                            let current_count = *demonstration_knowledge
                                .get(color)
                                .expect("Value must exist since key exists");
                            let parsed_count =
                                count.parse::<u32>().expect("Should be able to parse count");
                            if parsed_count > current_count {
                                demonstration_knowledge.insert(color, parsed_count);
                            }
                        } else {
                            demonstration_knowledge.insert(
                                color,
                                count.parse::<u32>().expect("Should be able to parse count"),
                            );
                        }
                    }
                }
            }
            postMessageToWorker(
                show_message,
                &format!("demonstration_knowledge: {:?}", demonstration_knowledge),
            );
            let is_possible = is_possible_game(&proposed_setup, &demonstration_knowledge);
            if is_possible {
                possible_games.push(game_int)
            }
            postMessageToWorker(show_message, &format!("is possible: {:?}", is_possible));
        }
    });
    let sum: u32 = possible_games.iter().sum();
    postMessageToWorker(true, &format!("⭐️ sum: {}", sum));
}

fn is_possible_game(
    proposed_setup: &HashMap<&str, u32>,
    demonstration_knowledge: &HashMap<&str, u32>,
) -> bool {
    let mut is_possible = true;
    for (color, count) in proposed_setup {
        if demonstration_knowledge.contains_key(color) {
            let demonstration_count = *demonstration_knowledge
                .get(color)
                .expect("Value must exist since key exists");
            if demonstration_count > *count {
                is_possible = false;
            }
        } else {
            is_possible = false;
        }
    }
    is_possible
}

pub fn solution_part_2() -> () {
    postMessageToWorker(true, "Part 2: Minimize valid case.\n");
    let mut iteration = -1;
    // let content = include_str!("input/day_02_part_1_test_input.txt");
    let content = include_str!("input/day_02_input.txt");

    let input_chunks = Regex::new(r"Game (\d+): (.*)").unwrap();
    let datum_parser = Regex::new(r"(\d+) (\w+)").unwrap();

    let mut game_powers: Vec<u32> = Vec::new();

    content.lines().for_each(|line| {
        // Provide a mechanism to limit the volume of output on the console.
        iteration += 1;
        let mut show_message = false;
        if (iteration) % 30 == 0 {
            show_message = true;
        }

        let characters: Vec<_> = line.chars().collect();
        if characters[0] == '#' {
            // postMessageToWorker(show_message, "Skipping line because it is a comment.");
            return;
        }

        postMessageToWorker(show_message, " ");
        postMessageToWorker(
            show_message,
            &format!("Iteration: {}, input: {}", iteration, line),
        );

        if let Some(caps) = input_chunks.captures(line) {
            postMessageToWorker(show_message, &format!(""));

            let mut demonstration_knowledge = HashMap::new();

            // postMessageToWorker(show_message, &format!("game: {}", game));
            let demonstrations_as_string = caps.get(2).map_or("", |m| m.as_str());
            let demonstrations: Vec<&str> = demonstrations_as_string.split(';').collect();
            for demonstration in demonstrations {
                // postMessageToWorker(show_message, &format!("demonstration: {}", demonstration));
                let dice_datum: Vec<&str> = demonstration.split(',').collect();
                for datum in dice_datum {
                    if let Some(caps) = datum_parser.captures(datum) {
                        let count = caps.get(1).map_or("", |m| m.as_str());
                        let color = caps.get(2).map_or("", |m| m.as_str());
                        // postMessageToWorker(show_message, &format!("__count: {}, color: {}", count, color));

                        // added .expect()s with copilot here
                        if demonstration_knowledge.contains_key(color) {
                            let current_count = *demonstration_knowledge
                                .get(color)
                                .expect("Value must exist since key exists");
                            let parsed_count =
                                count.parse::<u32>().expect("Should be able to parse count");
                            if parsed_count > current_count {
                                demonstration_knowledge.insert(color, parsed_count);
                            }
                        } else {
                            demonstration_knowledge.insert(
                                color,
                                count.parse::<u32>().expect("Should be able to parse count"),
                            );
                        }
                    }
                }
            }
            postMessageToWorker(
                show_message,
                &format!("demonstration_knowledge: {:?}", demonstration_knowledge),
            );
            let mut game_power = 1;
            for (_color, count) in demonstration_knowledge {
                game_power = game_power * count;
            }
            game_powers.push(game_power);
            postMessageToWorker(show_message, &format!("game_power: {:?}", game_power));
        }
    });

    let sum: u32 = game_powers.iter().sum();
    postMessageToWorker(true, &format!("⭐️ sum: {}", sum));
}
