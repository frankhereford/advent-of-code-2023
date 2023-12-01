//use web_sys::console;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(message: &str);
}

pub fn solution(n: u32) -> String {
    postMessageToWorker("Challenge statement, Day 0: \nUse a compiled (on the server) rust program in the client's browser to compute the 1,111,111th prime number.\n");

    let mut count = 0;
    let mut num = 2;

    while count < n {
        if is_prime(num) {
            count += 1;
            if count % 100000 == 0 {
                let message = format!(
                    "{}th prime: {}",
                    format_with_commas(count),
                    format_with_commas(num)
                );
                postMessageToWorker(&message);
            }
        }
        num += 1;
    }

    postMessageToWorker("\n");
    let message = format!(
        "🎉🎯 {}th prime found: {}\n\n",
        format_with_commas(n),
        format_with_commas(num - 1)
    );
    postMessageToWorker(&message);
    format_with_commas(num - 1)
}

fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }
    if num <= 3 {
        return true;
    }
    if num % 2 == 0 || num % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= num {
        if num % i == 0 || num % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn format_with_commas(num: u32) -> String {
    // Convert the number to string and insert commas
    num.to_string()
        .chars()
        .rev()
        .enumerate()
        .fold(String::new(), |mut acc, (i, c)| {
            if i % 3 == 0 && i != 0 {
                acc.push(',');
            }
            acc.push(c);
            acc
        })
        .chars()
        .rev()
        .collect()
}
