use num_bigint::*;
use rayon::prelude::*;
use std::hash::*;
use std::vec::*;

use crate::file::parameters;

mod file;

use num_bigint::BigUint;
use num_traits::{One, Zero};

fn step(
    x: &mut BigUint,
    a: &mut BigUint,
    b: &mut BigUint,
    p: &BigUint,
    pub_a: &BigUint,
    alpha: &BigUint,
    q: &BigUint,) {
    let remainder = x.iter_u32_digits().next().unwrap_or(0) % 3;

    match remainder {
        0 => {
            *x *= pub_a;
            *x %= p;

            *b += 1u8;
            if *b == *q {
                *b = BigUint::ZERO;
            }
        }

        1 => {
            let old_x = x.clone();
            *x *= &old_x;
            *x %= p;

            *a <<= 1;
            if *a >= *q {
                *a -= q;
            }

            *b <<= 1;
            if *b >= *q {
                *b -= q;
            }
        }

        2 => {
            *x *= alpha;
            *x %= p;

            *a += 1u8;
            if *a == *q {
                *a = BigUint::ZERO;
            }
        }
        _ => unreachable!()
    }
}

fn prime_factors(mut n: BigUint) -> Vec<BigUint> {
    let mut factors = Vec::new();
    let mut d = BigUint::from(2u8);

    while &d * &d <= n {
        while &n % &d == BigUint::ZERO {
            factors.push(d.clone());
            n /= &d;
        }
        d += 1u8;
    }

    if n > BigUint::from(1u8) {
        factors.push(n);
    }

    factors
}
fn ord(alpha: &BigUint, p: &BigUint) -> BigUint {
    let mut n = p - 1u8;
    for q in prime_factors(n.clone()) {
        let candidate = &n / &q;
        if alpha.modpow(&candidate, p) == BigUint::from(1u8) {
            n = candidate;
        }
    }

    n
}

fn sub_mod(a: &BigUint, b: &BigUint, n: &BigUint) -> BigUint {
    if a >= b {
        a - b
    } else {
        n - (b - a)
    }
}

fn try_seed(
    i: u64,
    p: &BigUint,
    alpha: &BigUint,
    pub_a: &BigUint,
    pub_b: &BigUint,
    n: &BigUint,
) -> Option<BigUint> {
    let mut hasher = DefaultHasher::new();
    hasher.write_u64(i);
    let mut a = hasher.finish().to_biguint()? % n;
    hasher.write_u64(1);
    let mut b = hasher.finish().to_biguint()? % n;
    let mut x = (alpha.modpow(&a, p) * pub_a.modpow(&b, p)) % p;

    let mut x2 = x.clone();
    let mut a2 = a.clone();
    let mut b2 = b.clone();

    loop {
        step(&mut x, &mut a, &mut b, p, pub_a, alpha, n);

        step(&mut x2, &mut a2, &mut b2, p, pub_a, alpha, n);
        step(&mut x2, &mut a2, &mut b2, p, pub_a, alpha, n);

        if x == x2 {
            break;
        }
    }

    let r = sub_mod(&b, &b2, n);

    if r == BigUint::ZERO {
        return None;
    }

    let r_inv = r.modinv(n)?;

    let priv_key = (r_inv * sub_mod(&a2, &a, n)) % n;

    if alpha.modpow(&priv_key, p) != *pub_a {
        None
    } else {
        Some(pub_b.modpow(&priv_key, p))
    }
}


fn main() {
    let number_problem_str = std::env::args_os().nth(1).unwrap_or_default();
    if number_problem_str.is_empty() {
        panic!("usage: pollard-rho <problem number>")
    }

    let number_problem = number_problem_str.into_string().unwrap().parse::<u32>().unwrap();
    let (p, alpha, pub_a, pub_b) = parameters(number_problem, "desafios.txt");
    println!("p\t= {p}");
    println!("alpha\t= {alpha}");
    println!("pub_a\t= {pub_a}");
    println!("pub_b\t= {pub_b}");
    let n = ord(&alpha, &p);
    println!("ord\t= {n}");
    
    let result = (1u64..u64::MAX)
        .into_par_iter()
        .find_map_first(|i| {
            try_seed(i, &p, &alpha, &pub_a, &pub_b, &n)
        });

    println!("k_ab\t= {}", result.unwrap());
}
