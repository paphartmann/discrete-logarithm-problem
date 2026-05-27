use crate::file::parameters;

mod file;
mod problem;

fn main() {
    let number_problem_str = std::env::args_os().nth(1).unwrap_or_default();
    if number_problem_str.is_empty() {
        panic!("usage: pollard-rho <problem number>")
    }

    let number_problem = number_problem_str.into_string().unwrap().parse::<u32>().unwrap();
    let parameters = parameters(number_problem, "desafios.txt".to_string());
    println!("p = {}", parameters.prime_modulus);
    println!("alpha = {}", parameters.alpha_generator);
    println!("A = {}", parameters.a_public_exp);
    println!("B = {}", parameters.b_public_exp);
}
