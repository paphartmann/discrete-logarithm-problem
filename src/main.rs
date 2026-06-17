use std::time::Instant;
use num_prime::nt_funcs::factorize;
use num_bigint::BigUint;
use num_traits::ToPrimitive;

use crate::{file::parameters, pollard_rho_log::find_log};

mod file;
mod pollard_rho_log;

fn ord(alpha: &BigUint, p: &BigUint) -> BigUint {
    let mut n = p - 1u8;
    for (q,exp) in factorize(n.clone()) {
        for _ in 0..exp {
            let candidate = &n / &q;
            if alpha.modpow(&candidate, p) == BigUint::from(1u8) {
                n = candidate;
            }
        }
    }

    n
}

fn main() {
    let number_problem_str = std::env::args().nth(1).expect("usage: discrete-logarithm-problem <number>");

    let number_problem = number_problem_str.parse::<u32>().expect("argument should be a number");
    let (p, alpha, pub_a, pub_b) = parameters(number_problem, "desafios.txt");
    println!("p\t= {p}");
    println!("alpha\t= {alpha}");
    println!("A\t= {pub_a}");
    println!("B\t= {pub_b}");

    let begin = Instant::now();
    let q = ord(&alpha, &p);
    let xis: Vec<_> = factorize(q.clone()).into_iter()
        .map(|(pi,ei)| -> (BigUint, BigUint) {
            let piei = pi.pow(ei.to_u32().unwrap());
            let gi = alpha.modpow(&(&q/&piei), &p);
            let hi = pub_a.modpow(&(&q/&piei), &p);
            (find_log(&p, &gi, &hi, &piei),piei)
        })
        .collect();

    let priv_a = xis.into_iter()
        .map(|(xi,ni)| -> BigUint {
            let bign_i = &q/&ni;
            xi * &bign_i * bign_i.modinv(&ni).unwrap()
        })
        .sum();

    let end = Instant::now();

    let k_ab = pub_b.modpow(&priv_a, &p);
    assert_eq!(pub_a, alpha.modpow(&priv_a, &p), "Algorithm found wrong k_ab");
    println!("k_ab\t= {}", k_ab);
    println!("It took {} seconds", (end - begin).as_secs_f32());
}
