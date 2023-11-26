use web_sys::console;


pub fn solution(n: u32) -> String {
    let mut count = 0;
    let mut num = 2;

    while count < n {
        if is_prime(num) {
            count += 1;
            // Log the current count - replace this with actual message sending
            console::log_1(&format!("Prime count: {}", count).into());
        }
        if count < n {
            num += 1;
        }
    }

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
