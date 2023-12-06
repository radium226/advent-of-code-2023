#[allow(dead_code, unused_variables)]

use std::env;
use std::io::{self, BufRead};

mod solve_01;
use solve_01::solve_01;

mod solve_02;
use solve_02::{solve_02_part_one, solve_02_part_two};


fn main() {
    let args: Vec<String> = env::args().collect();
    let ref args: Vec<&str> = args.iter().map(String::as_str).skip(1).collect();
    dbg!(args);

    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let lines = lines.filter_map(|line| line.ok());

    match &args[..] {
        ["01"] => {
            solve_01(lines);
        }

        ["02", "part-one"] => {
            solve_02_part_one(lines);
        }

        ["02", "part-two"] => {
            solve_02_part_two(lines);
        }

        other => {
            dbg!(other);
            panic!("Unknown day! ");
        }
    }
}