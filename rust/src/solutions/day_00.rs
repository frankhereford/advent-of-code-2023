//use web_sys::console;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(message: &str);
}

pub fn solution(n: u32) -> String {
    postMessageToWorker("Challenge statement: \nUse a compiled language in the client's browser to compute the 80th prime number.\n");

    let mut count = 0;
    let mut num = 2;

    while count < n {
        if is_prime(num) {
            count += 1;
            if count % 10 == 0 { // Post message only for every 10th prime
                let message = format!("{count}th prime in sequence: {num}", count = count, num = num);
                postMessageToWorker(&message);
            }
        }
        num += 1;
    }

    postMessageToWorker("\n");
    let message = format!("🎉 {n}th prime found: {num}\n\n", n = n, num = num - 1);
    postMessageToWorker(&message);
    (num - 1).to_string()
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
