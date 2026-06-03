use miller_rabin::is_prime;
use num_bigint::*;
use rayon::prelude::*;
use std::hash::*;
use std::time::Instant;
use std::collections::BTreeMap;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::file::parameters;

mod file;

static STOP: AtomicBool = AtomicBool::new(false);

fn step(
    x: &mut BigUint,
    a: &mut BigUint,
    b: &mut BigUint,
    p: &BigUint,
    pub_a: &BigUint,
    alpha: &BigUint,
    q: &BigUint) {
    let remainder = (&*x % 3u8).to_u8().unwrap();

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
            *x = &*x * &*x;
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

fn gcd(a: BigUint, b: BigUint) -> BigUint {
    if &b == &BigUint::ZERO {
        a
    } else {
        let r = a % &b;
        gcd(b, r)
    }
}

fn prime_factors(n: &BigUint) -> BTreeMap<BigUint, u32> {
    let mut factors = BTreeMap::new();
    let one = BigUint::from(1u8);

    if *n == one {
        return factors;
    }

    if is_prime(n, n.to_u64_digits().len() / 4) {
        factors.insert(n.clone(), 1);
        return factors;
    }

    for c in 1u32.. {
        let mut xi = BigUint::from(2u8);
        let mut x2i = xi.clone();
        let f = |x: &BigUint| -> BigUint { ((x * x) + c) % n };
        let mut d = one.clone();

        while &d == &one {
            xi = f(&xi);
            x2i = f(&f(&x2i));
            d = gcd(if &xi > &x2i {&xi - &x2i} else {&x2i - &xi}, n.clone())
        }
        if d < *n {
            for (k, v) in prime_factors(&d) {
                *factors.entry(k).or_default() += v;
            }

            for (k, v) in prime_factors(&(n/&d)) {
                *factors.entry(k).or_default() += v;
            }

            break
        }
    }

    factors
}

fn ord(alpha: &BigUint, p: &BigUint) -> BigUint {
    let mut n = p - 1u8;
    for (q,exp) in dbg!(prime_factors(&n)) {
        for _ in 0..exp {
            let candidate = &n / &q;
            if alpha.modpow(&candidate, p) == BigUint::from(1u8) {
                n = candidate;
            }
        }
    }

    n
}

fn sub_mod(a: &BigUint, b: &BigUint, n: &BigUint) -> BigUint {
    assert!(BigUint::ZERO <= *a && a < n && BigUint::ZERO <= *b && a < n);
    if a > b {
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
    // println!("{i}");
    let mut hasher = DefaultHasher::new();
    hasher.write_u64(i);
    let mut ai = hasher.finish().to_biguint()? % n;
    hasher.write_u64(1);
    let mut bi = hasher.finish().to_biguint()? % n;
    let mut xi = (alpha.modpow(&ai, p) * pub_a.modpow(&bi, p)) % p;

    let mut x2i = xi.clone();
    let mut a2i = ai.clone();
    let mut b2i = bi.clone();

    loop {
        step(&mut xi, &mut ai, &mut bi, p, pub_a, alpha, n);

        step(&mut x2i, &mut a2i, &mut b2i, p, pub_a, alpha, n);
        step(&mut x2i, &mut a2i, &mut b2i, p, pub_a, alpha, n);

        if xi == x2i {
            break;
        } else if STOP.load(Ordering::Relaxed) {
            return None
        }
    }

    let r = sub_mod(&bi, &b2i, n);

    if r == BigUint::ZERO {
        return None;
    }

    let r_inv = r.modinv(n)?;

    let a_priv_key = (r_inv * sub_mod(&a2i, &ai, n)) % n;

    if alpha.modpow(&a_priv_key, p) != *pub_a {
        None
    } else {
        STOP.store(true, Ordering::Relaxed);
        Some(pub_b.modpow(&a_priv_key, p))
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
    println!("A\t= {pub_a}");
    println!("B\t= {pub_b}");

    let begin = Instant::now();

    let n = ord(&alpha, &p);
    println!("ord\t= {n}");
    
    let result = (1u64..u64::MAX)
        .into_par_iter()
        .find_map_any(|i| {
            try_seed(i, &p, &alpha, &pub_a, &pub_b, &n)
        });

    let end = Instant::now();

    println!("k_ab\t= {}", result.unwrap());
    println!("It took {} minutes", (end - begin).as_secs_f32() / 60.0)
}
