#![allow(unreachable_code)]
#![allow(unused_variables)]
//use web_sys::console;
use wasm_bindgen::prelude::*;

use regex::Regex;
use radix_fmt::radix;
//use std::collections::{HashMap};
use indexmap::IndexMap;


#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(do_print: bool, message: &str);
}

type Card = char;
type Hand = [Card; 5];
type Wager = u32;
type ScoringValue = (String, f32);
type Game = (Hand, Wager, ScoringValue);
type Games = Vec<Game>;

pub fn solution_part_1() -> () {
    postMessageToWorker(true, "Part 1: \n");
    let mut iteration = -1;
    //let content = include_str!("input/day_07_test_input.txt");
    let content = include_str!("input/day_07_input.txt");

    let hand_regex = Regex::new(r"(\w{5}) (\d+)").unwrap();

    let mut games: Games = Vec::new();

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

        //postMessageToWorker(show_message, " ");
        //postMessageToWorker(show_message, &format!("Iteration: {}, input: {}", iteration, line));

        if let Some(captures) = hand_regex.captures(line) {
            let cards_as_string = captures.get(1).map_or("", |m| m.as_str().trim());
            let wager_as_string = captures.get(2).map_or("", |m| m.as_str().trim());
            let wager: Wager = wager_as_string.parse().unwrap_or(0);
            let hand_vector: Vec<char> = cards_as_string.chars().collect();
            if hand_vector.len() == 5 {
                let hand: Hand = [hand_vector[0], hand_vector[1], hand_vector[2], hand_vector[3], hand_vector[4]];
                let score: ScoringValue = ("".to_string(), 0.0);
                let game: Game = (hand, wager, score);
                games.push(game);
            }
        }
    });

    for game in games.iter_mut() {
        postMessageToWorker(true, &format!("\n"));
        game.2 = score_hand(true, game.0);
    }
    games.sort_by(|a, b| a.2.1.partial_cmp(&b.2.1).unwrap());

    let mut total_score: u32 = 0;
    for i in 0..games.len() {
        let game = &games[i];
        //postMessageToWorker(true, &format!("Game: {:?}, Wager: {:?}", game, game.1));
        let points = game.1 * (i as u32 + 1);
        total_score += points;
    }
    postMessageToWorker(true, &format!("Games: {:?}", games));
    postMessageToWorker(true, &format!("Total score: {:?}", total_score));
}


fn map_card_to_base_13_digit(card: Card) -> String {
    let cards_string = ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"];
    let cards: Vec<char> = cards_string.iter().map(|&s| s.chars().next().unwrap()).collect();
    for i in 0..cards.len() {
        if cards[i] == card {
            return radix(i, 13).to_string();
        }
    }
    panic!("This code should never be reached");
    return "0".to_string();
}

fn base13_float_to_base10(s: &str) -> f32 {
    let parts: Vec<&str> = s.split('.').collect();
    let integer_part = i32::from_str_radix(parts[0], 13).unwrap() as f32;
    let mut fractional_part = 0_f32;
    for (i, digit) in parts[1].chars().enumerate() {
        let value = i32::from_str_radix(&digit.to_string(), 13).unwrap() as f32;
        fractional_part += value * 13f32.powi(-(i as i32 + 1));
    }
    integer_part + fractional_part
}

fn score_hand(show_message: bool, hand: Hand) -> ScoringValue {
    //postMessageToWorker(true, &format!("Hand: {:?}", hand));
    let mut card_number_in_tridecimal: String = "".to_string();
    let mut decending_cards_in_tridecimal: Vec<String> = Vec::new();
    for i in 0..13 {
        decending_cards_in_tridecimal.push(radix(i, 13).to_string());
    }

    let mut card_count = IndexMap::new();
    for card in decending_cards_in_tridecimal.iter() {
        card_count.insert(card.to_string(), 0);
    }

    for card in hand.iter() {
        let card_tridecimal = map_card_to_base_13_digit(*card);
        card_number_in_tridecimal.push_str(&card_tridecimal);
        let count = card_count.entry(card_tridecimal).or_insert(0);
        *count += 1;
    }
    //postMessageToWorker(true, &format!("Card count: {:?}", card_count));

    let category_score = catagorize_hand(show_message, hand, card_count);
    
    let tridecimal_score_as_string: String = category_score.to_string() + "." + &card_number_in_tridecimal;
    let decimal_score = base13_float_to_base10(&tridecimal_score_as_string);
    let hand_score: ScoringValue = (tridecimal_score_as_string.clone(), decimal_score);

    //postMessageToWorker(true, &format!("tridecimal_score_as_string: {:?}", tridecimal_score_as_string));
    //postMessageToWorker(true, &format!("decimal_score: {:?}", decimal_score));
    postMessageToWorker(true, &format!("hand_score: {:?}", hand_score));

    return hand_score
}

fn catagorize_hand(show_message: bool, hand: Hand, card_count: IndexMap<String, u32>) -> u32 {
    // five of a kind
    for (card, count) in card_count.iter() {
        if *count == 5 {
            postMessageToWorker(true, &format!("Five of a kind: {:?}", hand));
            return 6;
        }
    }

    // four of a kind
    for (card, count) in card_count.iter() {
        if *count == 4 {
            postMessageToWorker(true, &format!("Four of a kind: {:?}", hand));
            return 5;
        }
    }
    
    // full house
    let mut full_house_triplet = false;
    let mut full_house_pair = false;
    for (card, count) in card_count.iter() {
        if *count == 3 { full_house_triplet = true; }
        if *count == 2 { full_house_pair = true; }
        if full_house_triplet && full_house_pair {
            postMessageToWorker(true, &format!("Full house: {:?}", hand));
            return 4;
        }
    }

    // three of a kind
    for (card, count) in card_count.iter() {
        if *count == 3 {
            postMessageToWorker(true, &format!("Three of a kind: {:?}", hand));
            return 3;
        }
    }

    // two pair
    let mut two_pair_first_pair= false;
    for (card, count) in card_count.iter() {
        //postMessageToWorker(true, &format!("card: {:?}, count: {:?}", card, count));
        if *count == 2 && !two_pair_first_pair {
            //postMessageToWorker(true, &format!("Found a pair: {:?}", hand));
            two_pair_first_pair = true; 
            continue;
        }
        if *count == 2 && two_pair_first_pair { 
            postMessageToWorker(true, &format!("Two pair: {:?}", hand));
            return 2;
        }
    }

    // single pair
    for (card, count) in card_count.iter() {
        if *count == 2 {
            postMessageToWorker(true, &format!("One pair: {:?}", hand));
            return 1;
        }
    }

    // this catch all case is the "high card" case

    postMessageToWorker(true, &format!("High card: {:?}", hand));
    return 0
}



pub fn solution_part_2() -> () {
    return;
    postMessageToWorker(true, "Part 2: \n");
    let mut iteration = -1;
    let content = include_str!("input/day_07_test_input.txt");
    // let content = include_str!("input/day_07_input.txt");

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