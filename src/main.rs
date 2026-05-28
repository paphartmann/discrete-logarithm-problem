use num_bigint::*;

use crate::{file::parameters, problem::Problem};

mod file;
mod problem;

fn next_xi(xi: &BigUint, params: &Problem) -> BigUint {
    let remainder = xi % 3u8;
    if remainder == BigUint::ZERO {
        &params.a_public_exp * xi
    } else if remainder == BigUint::from(1u8) {
        xi * xi
    } else {
        &params.alpha_generator * xi
    }
}

fn next_ai(ai: BigUint, xi: &BigUint, params: &Problem) -> BigUint {
    let remainder = xi % 3u8;
    if remainder == BigUint::ZERO {
        ai
    } else if remainder == BigUint::from(1u8) {
        (ai << 1) % &params.prime_modulus
    } else {
        (ai + 1u8) % &params.prime_modulus
    }
}

fn next_bi(bi: BigUint, xi: &BigUint, params: &Problem) -> BigUint {
    let remainder = xi % 3u8;
    if remainder == BigUint::ZERO {
        (bi + 1u8) % &params.prime_modulus
    } else if remainder == BigUint::from(1u8) {
        (bi << 1) % &params.prime_modulus
    } else {
        bi
    }
}

fn main() {
    let number_problem_str = std::env::args_os().nth(1).unwrap_or_default();
    if number_problem_str.is_empty() {
        panic!("usage: pollard-rho <problem number>")
    }

    let number_problem = number_problem_str.into_string().unwrap().parse::<u32>().unwrap();
    let params = parameters(number_problem, "desafios.txt".to_string());

    let mut ai = BigUint::ZERO;
    let mut bi = ai.clone();
    let mut xi = bi.clone() + 1u32;
    let mut xii = xi.clone();
    let mut aii = ai.clone();
    let mut bii = bi.clone();

    loop {
        ai = next_ai(ai, &xi, &params);
        bi = next_bi(bi, &xi, &params);
        xi = next_xi(&xi, &params);

        
        aii = next_ai(aii, &xii, &params);
        bii = next_bi(bii, &xii, &params);
        xii = next_xi(&xii, &params);
        aii = next_ai(aii, &xii, &params);
        bii = next_bi(bii, &xii, &params);
        xii = next_xi(&xii, &params);

        if xi == xii {
            break;
        }
    }
    let r = bi - bii;
    if r.eq(&BigUint::ZERO) {
        panic!("r == 0")
    }
    let inv_r = r.modinv(&params.prime_modulus).unwrap();
    let alice_private_key = (inv_r * (aii - ai)) % &params.prime_modulus;
    println!("k_ab = {}", &params.b_public_exp.modpow(&alice_private_key, &params.prime_modulus))
}
