//use web_sys::console;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(message: &str);
}

pub fn solution(n: u32) -> String {
    let mut count = 0;
    let mut num = 2;

    while count < n {
        if is_prime(num) {
            count += 1;
            let message = format!("Prime count: {}", count);
            // console::log_1(&format!("{}", message).into());
            postMessageToWorker(&message);
        }
        if count < n {
            num += 1;
        }
    }

    let message = format!("{n}th prime found: {num}", n = n, num = num);
    postMessageToWorker(&message);
    num.to_string()
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
