use num_bigint::*;
use rayon::prelude::*;
use std::hash::*;
use std::vec::*;

use crate::file::parameters;

mod file;

fn step(
    x: BigUint,
    a: BigUint,
    b: BigUint,
    p: &BigUint,
    pub_a: &BigUint,
    alpha: &BigUint,
    q: &BigUint,
) -> (BigUint, BigUint, BigUint) {
    let remainder = &x % 3u8;

    if remainder == BigUint::ZERO {
        let x_next = (pub_a * &x) % p;
        let a_next = a;
        let b_next = (b + 1u8) % q;
        (x_next, a_next, b_next)

    } else if remainder == BigUint::from(1u8) {
        let x_next = (&x * &x) % p;
        let a_next = (a << 1) % q;
        let b_next = (b << 1) % q;
        (x_next, a_next, b_next)

    } else {
        let x_next = (alpha * &x) % p;
        let a_next = (a + 1u8) % q;
        let b_next = b;
        (x_next, a_next, b_next)
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
        (x, a, b) = step(x, a, b, p, pub_a, alpha, n);

        (x2, a2, b2) = step(x2, a2, b2, p, pub_a, alpha, n);
        (x2, a2, b2) = step(x2, a2, b2, p, pub_a, alpha, n);

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
