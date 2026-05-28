use std::fs;
use num_bigint::*;

use super::problem::Problem;

fn parse_value(line: &str) -> BigUint {
    line.split_once('=')
        .unwrap()
        .1
        .trim()
        .parse()
        .unwrap()
}

pub fn parameters(problem_number: u32, filepath: String) -> Problem {
    let prefix = format!("[C{}]", problem_number);
    let content = fs::read_to_string(filepath).unwrap();
    let mut file_it = content.lines().skip_while(|s| {!s.starts_with(&prefix)});

    file_it.next();
    let p = parse_value(file_it.next().unwrap());
    let alpha = parse_value(file_it.next().unwrap());
    let a_public_exp = parse_value(file_it.next().unwrap());
    let b_public_exp = parse_value(file_it.next().unwrap());

    Problem {
        prime_modulus: p,
        alpha_generator: alpha,
        a_public_exp: a_public_exp,
        b_public_exp: b_public_exp,
    }
}
