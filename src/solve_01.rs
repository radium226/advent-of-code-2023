
use num_bigint::{self, BigUint, ToBigUint};
use fancy_regex::Regex;
use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref DIGITS_BY_WORD: HashMap<String, u32> = {
        let mut digits_by_word: HashMap<String, u32> = HashMap::new();

        for (position, digit) in ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].iter().enumerate() {
            let position: u32 = position as u32;
            digits_by_word.insert(digit.to_string(), position);
        }

        digits_by_word
    };
}

fn parse_number(line: &str) -> Option<BigUint> {
    let regex: Regex = Regex::new(r"(?=((?P<digit>[0-9])|(?P<word>one|two|three|four|five|six|seven|eight|nine)))").expect("Unable to create regex! ");

    let digits: Vec<u32> = regex
        .captures_iter(line)
        .filter_map(|captures| {
            let captures = captures.expect("Wut?");

            let digit: Option<u32> = captures
                .name("digit")
                .and_then(|digit| digit.as_str().parse::<u32>().ok());

            // println!("digit: {:?}", digit);

            let word: Option<u32> = captures
                .name("word")
                .and_then(|word| DIGITS_BY_WORD.get(word.as_str()).map(|digit| digit.clone()));
            // println!("word: {:?}", word);

            digit.or(word)
        })
        .collect();

    let number: Option<String> = match &digits[..] {
        [first_digit, .., last_digit] => Some(format!("{}{}", first_digit, last_digit)),
        [single_digit] => Some(format!("{}{}", single_digit, single_digit)),
        _ => None,
    };

    let number: Option<BigUint> = number
        .and_then(|number| number.parse::<u32>().ok())
        .and_then(|number| number.to_biguint());

    return number;
}

fn do_solve_01<I: IntoIterator<Item=String>>(lines: I) -> BigUint {
    let mut numbers: Vec<BigUint> = vec![];

    for line in lines {
        if let Some(number) = parse_number(&line) {
            //println!("{} -> {}", line, number);
            numbers.push(number);
        }
    }

    return numbers.iter().sum();
}

pub fn solve_01<I: IntoIterator<Item=String>>(lines: I) -> () {
    let solution = do_solve_01(lines);
    println!("solution: {}", solution);
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse_number_should_work() {
        use super::*;
        use num_bigint::ToBigUint;

        // let number = parse_number("oneight");
        // assert_eq!(number, Some(88_u32.to_biguint().unwrap()));

        let number = parse_number("1abc2");
        assert_eq!(number, Some(12_u32.to_biguint().unwrap()));

        let number = parse_number("pqr3stueightvwx");
        assert_eq!(number, Some(38_u32.to_biguint().unwrap()));

        let number = parse_number("athreeb");
        assert_eq!(number, Some(33_u32.to_biguint().unwrap()));

        let number = parse_number("toto");
        assert_eq!(number, None);
    }

    #[test]
    fn do_solve_01_should_work_for_part_one() {
        use super::*;
        use num_bigint::ToBigUint;

        let lines: Vec<&str> = vec![
            "1abc2",
            "pqr3stu8vwx",
            "a1b2c3d4e5f",
            "treb7uchet",
        ];
        let lines = lines.iter().map(|line| line.to_string());
        let sum = do_solve_01(lines);

        assert_eq!(sum, 142_u32.to_biguint().unwrap());
    }

    #[test]
    fn do_solve_01_should_work_for_part_two() {
        use super::*;
        use num_bigint::ToBigUint;

        let lines: Vec<&str> = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        let lines = lines.iter().map(|line| line.to_string());
        let sum = do_solve_01(lines);

        assert_eq!(sum, 281_u32.to_biguint().unwrap());
    }
}