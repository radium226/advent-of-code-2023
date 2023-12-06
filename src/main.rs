#[allow(dead_code, unused_variables)]

use std::io::{self, BufRead};
use regex::Regex;


fn solve_01<I: IntoIterator<Item=String>>(lines: I) -> u32 {
    let Ok(regex) = Regex::new(r"[0-9]+") else {
        panic!("Unable to create regex! ");
    };

    let mut numbers: Vec<u32> = vec![];

    for line in lines {
        let digits: Vec<&str> = regex
            .find_iter(&line)
            .map(|m| m.as_str())
            .collect();

        let number: String = match &digits[..] {
            [first_digit, .., last_digit] => format!("{}{}", first_digit, last_digit),
            [only_digit] => only_digit.to_string() + only_digit,
            _ => continue,
        };

        let Ok(number) = number.parse::<u32>() else {
            println!("{:?}", number);
            panic!("Unable to parse number! ");
        };

        numbers.push(number);
    }

    println!("{:?}", numbers);

    let sum: u32 = numbers.iter().sum();

    return sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_01_should_work() {
        use super::*;

        let lines: Vec<&str> = vec![
            "1abc2",
            "pqr3stu8vwx",
            "a1b2c3d4e5f",
            "treb7uchet",
        ];
        let lines = lines.iter().map(|line| line.to_string());
        let sum = solve_01(lines);

        assert_eq!(sum, 142);
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let lines = lines.filter_map(|line| line.ok());
    let result = solve_01(lines);
    print!("{}", result)
}