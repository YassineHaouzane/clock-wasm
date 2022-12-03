mod digits;
use chrono::prelude::*;
use digits::*;

pub fn get_current_time() -> (u32, u32, u32) {
    let local: DateTime<Local> = Local::now();
    (local.hour(), local.minute(), local.second())
}

fn get_digits(num: u32) -> (u32, u32) {
    (num / 10, num % 10)
}

pub fn get_time(hours: u32, minutes: u32, secs: u32) -> String {
    let (h1, h2) = get_digits(hours);
    let (m1, m2) = get_digits(minutes);
    let (s1, s2) = get_digits(secs);
    let mut res_string = String::new();
    let time_s = format!("{}{}:{}{}:{}{}", h1, h2, m1, m2, s1, s2);
    for row in &DIGITS {
        for c in time_s.chars() {
            let col = match c {
                '0'..='9' => c as usize - '0' as usize,
                ':' => 10,
                _ => 10,
            };
            res_string.push_str(row[col]);
        }
        res_string.push_str("<br>");
    }
    res_string
}
