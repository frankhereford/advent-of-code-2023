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

    for i in 0..games.len() {
        let mut show_message = false;
        if (i) % 100 == 0  {
            show_message = true;
        }
        games[i].2 = score_hand(show_message, games[i].0);
    }
    games.sort_by(|a, b| a.2.1.partial_cmp(&b.2.1).unwrap());

    let mut total_score: u32 = 0;
    for i in 0..games.len() {
        let game = &games[i];
        //postMessageToWorker(true, &format!("Game: {:?}, Wager: {:?}", game, game.1));
        let points = game.1 * (i as u32 + 1);
        total_score += points;
    }
    //postMessageToWorker(true, &format!("Games: {:?}", games));
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
    //postMessageToWorker(show_message, &format!("Hand: {:?}", hand));
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

    let category_score = catagorize_hand(show_message, hand, card_count);
    let tridecimal_score_as_string: String = category_score.to_string() + "." + &card_number_in_tridecimal;
    let decimal_score = base13_float_to_base10(&tridecimal_score_as_string);
    let hand_score: ScoringValue = (tridecimal_score_as_string.clone(), decimal_score);

    return hand_score
}

fn catagorize_hand(show_message: bool, hand: Hand, card_count: IndexMap<String, u32>) -> u32 {
    // five of a kind
    for (card, count) in card_count.iter() {
        if *count == 5 {
            postMessageToWorker(show_message, &format!("Five of a kind: {:?}", hand));
            return 6;
        }
    }

    // four of a kind
    for (card, count) in card_count.iter() {
        if *count == 4 {
            postMessageToWorker(show_message, &format!("Four of a kind: {:?}", hand));
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
            postMessageToWorker(show_message, &format!("Full house: {:?}", hand));
            return 4;
        }
    }

    // three of a kind
    for (card, count) in card_count.iter() {
        if *count == 3 {
            postMessageToWorker(show_message, &format!("Three of a kind: {:?}", hand));
            return 3;
        }
    }

    // two pair
    let mut two_pair_first_pair = false;
    for (card, count) in card_count.iter() {
        if *count == 2 && !two_pair_first_pair {
            two_pair_first_pair = true; 
            continue;
        }
        if *count == 2 && two_pair_first_pair { 
            postMessageToWorker(show_message, &format!("Two pair: {:?}", hand));
            return 2;
        }
    }

    // single pair
    for (card, count) in card_count.iter() {
        if *count == 2 {
            postMessageToWorker(show_message, &format!("One pair: {:?}", hand));
            return 1;
        }
    }

    // this catch all case is the "high card" case
    postMessageToWorker(show_message, &format!("High card: {:?}", hand));
    return 0
}


pub fn solution_part_2() -> () {
    postMessageToWorker(true, "Part 2: Jokers Wild \n");
    let mut iteration = -1;
    //let content = include_str!("input/day_07_test_input.txt");
    let content = include_str!("input/day_07_input.txt");

    let hand_regex = Regex::new(r"(\w{5}) (\d+)").unwrap();

    let mut games: Games = Vec::new();

    content.lines().for_each(|line| {
        // Provide a mechanism to limit the volume of output on the console.
        iteration += 1;

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

    for i in 0..games.len() {
        let mut show_message = false;
        if (i) % 100 == 0  {
            show_message = true;
        }
        games[i].2 = score_hand_part_2(show_message, games[i].0);
    }
    games.sort_by(|a, b| a.2.1.partial_cmp(&b.2.1).unwrap());

    let mut total_score: u32 = 0;
    for i in 0..games.len() {
        let game = &games[i];
        //postMessageToWorker(true, &format!("Game: {:?}, Wager: {:?}", game, game.1));
        let points = game.1 * (i as u32 + 1);
        total_score += points;
    }
    //postMessageToWorker(true, &format!("Games: {:?}", games));
    postMessageToWorker(true, &format!("Total score: {:?}", total_score));
}

fn map_card_to_base_13_digit_part_2(card: Card) -> String {
    let cards_string = ["J", "2", "3", "4", "5", "6", "7", "8", "9", "T", "Q", "K", "A"];
    let cards: Vec<char> = cards_string.iter().map(|&s| s.chars().next().unwrap()).collect();
    for i in 0..cards.len() {
        if cards[i] == card {
            return radix(i, 13).to_string();
        }
    }
    panic!("This code should never be reached");
    return "0".to_string();
}


fn score_hand_part_2(show_message: bool, hand: Hand) -> ScoringValue {
    //postMessageToWorker(show_message, &format!("Hand: {:?}", hand));
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
        let card_tridecimal = map_card_to_base_13_digit_part_2(*card);
        card_number_in_tridecimal.push_str(&card_tridecimal);
        let count = card_count.entry(card_tridecimal).or_insert(0);
        *count += 1;
    }

    let category_score = catagorize_hand_part_2(show_message, hand, card_count);
    let tridecimal_score_as_string: String = category_score.to_string() + "." + &card_number_in_tridecimal;
    let decimal_score = base13_float_to_base10(&tridecimal_score_as_string);
    let hand_score: ScoringValue = (tridecimal_score_as_string.clone(), decimal_score);

    return hand_score
}

fn catagorize_hand_part_2(show_message: bool, hand: Hand, card_count: IndexMap<String, u32>) -> u32 {
    
    //postMessageToWorker(show_message, &format!("Hand: {:?}, card_count: {:?}", hand, card_count));
    
    // five of a kind
    for (card, count) in card_count.iter() {
        if card == "0" { continue; }
        //postMessageToWorker(show_message, &format!("card: {:?}, count: {:?}, jokers: {}", card, count, card_count["0"]));
        if *count == 5 {
            postMessageToWorker(show_message, &format!("Five of a kind: {:?}", hand));
            return 6;
        }
    }

    // four of a kind
    for (card, count) in card_count.iter() {
        if card == "0" { continue; }
        if *count == 4 && card_count["0"] == 1 {
            postMessageToWorker(show_message, &format!("Four of a kind + J: {:?}", hand));
            return 6;
        }
        if *count == 4 {
            postMessageToWorker(show_message, &format!("Four of a kind: {:?}", hand));
            return 5;
        }
    }
    
    // full house 
    let mut full_house_triplet = false;
    let mut full_house_pair = false;
    for (card, count) in card_count.iter() {
        if card == "0" { continue; }
        if *count == 3 { full_house_triplet = true; }
        if *count == 2 { full_house_pair = true; }

    }

    let mut full_house_pair_count = 0;
    for (card, count) in card_count.iter() {
        if card == "0" { continue; }
        if *count == 2 { full_house_pair_count += 1; continue; }
        if *count == 2 { full_house_pair_count += 1; }
    }

    if card_count["0"] == 2 && full_house_triplet {
        postMessageToWorker(show_message, &format!("three of a kind + JJ: {:?}", hand));
        return 6; // five of a kind
    } else if card_count["0"] == 1 && full_house_triplet {
        postMessageToWorker(show_message, &format!("three of a kind + J: {:?}", hand));
        return 5; // four of a kind
    } else if card_count["0"] == 3 && full_house_pair {
        postMessageToWorker(show_message, &format!("pair + JJJ: {:?}", hand));
        return 6; // five of a kind
    } else if full_house_pair_count == 2 && card_count["0"] == 1 {
        postMessageToWorker(show_message, &format!("two pair + J: {:?}", hand));
        return 4; // full house
    } else if card_count["0"] == 2 && full_house_pair {
        postMessageToWorker(show_message, &format!("pair + JJ: {:?}", hand));
        return 5; // four of a kind 
    } else if card_count["0"] == 1 && full_house_pair {
        postMessageToWorker(show_message, &format!("pair + J: {:?}", hand));
        return 3; // three of a kind
    } else if full_house_triplet && full_house_pair {
        postMessageToWorker(show_message, &format!("Full house: {:?}", hand));
        return 4;
    }


    // three of a kind
    for (card, count) in card_count.iter() {
        if card == "0" { continue; }
        if *count == 3 && card_count["0"] == 2 {
            postMessageToWorker(show_message, &format!("Three of a kind + JJ: {:?}", hand));
            return 6; // five of a kind
        } else if *count == 3 && card_count["0"] == 1 {
            postMessageToWorker(show_message, &format!("Three of a kind + J: {:?}", hand));
            return 5; // four of a kind
        } else if *count == 3 {
            postMessageToWorker(show_message, &format!("Three of a kind: {:?}", hand));
            return 3;

        }
    }

    // two pair
    let mut two_pair_first_pair = false;
    for (card, count) in card_count.iter() {
        if card == "0" { continue; }
        if *count == 2 && !two_pair_first_pair {
            two_pair_first_pair = true; 
            continue;
        }
        if *count == 2 && two_pair_first_pair && card_count["0"] == 1 { 
            postMessageToWorker(show_message, &format!("Two pair + J: {:?}", hand));
            return 4; // full house
        } else
        if *count == 2 && two_pair_first_pair { 
            postMessageToWorker(show_message, &format!("Two pair: {:?}", hand));
            return 2; // two pair
        }
    }

    // single pair
    for (card, count) in card_count.iter() {
        if card == "0" { continue; }
        if *count == 2 && card_count["0"] == 3 {
            postMessageToWorker(show_message, &format!("One pair + JJJ: {:?}", hand));
            return 6; // five of a kind
        } else if *count == 2 && card_count["0"] == 2 {
            postMessageToWorker(show_message, &format!("One pair + JJ: {:?}", hand));
            return 5; // four of a kind
        } else if *count == 2 && card_count["0"] == 1 {
            postMessageToWorker(show_message, &format!("One pair + J: {:?}", hand));
            return 3; 
        } else if *count == 2 {
            postMessageToWorker(show_message, &format!("One pair: {:?}", hand));
            return 1;
        }
    }

    // this catch all case is the "high card" case
    if card_count["0"] == 5 {
        postMessageToWorker(show_message, &format!("JJJJJ: {:?}", hand));
        return 6; // five of a kind
    } else if card_count["0"] == 4 {
        postMessageToWorker(show_message, &format!("High card + JJJJ: {:?}", hand));
        return 6; // five of a kind
    } else if card_count["0"] == 3 {
        postMessageToWorker(show_message, &format!("High card + JJJ: {:?}", hand));
        return 5;  // four of a kind
    } else if card_count["0"] == 2 {
        postMessageToWorker(show_message, &format!("High card + JJ: {:?}", hand));
        return 3; // three of a kind
    } else if card_count["0"] == 1 {
        postMessageToWorker(show_message, &format!("High card + J: {:?}", hand));
        return 1; // one pair
    }
    postMessageToWorker(show_message, &format!("High card: {:?}", hand));
    return 0
}
