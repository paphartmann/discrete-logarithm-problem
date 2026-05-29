use num_bigint::*;

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

fn ord(alpha: &BigUint, modulus: &BigUint) -> BigUint {
    let mut i = BigUint::ZERO;
    while i < *modulus {
        i += 1u8;
        if alpha.modpow(&i, modulus) == BigUint::from(1u8) {
            return i
        }
    }
    i
}

fn sub_mod(a: &BigUint, b: &BigUint, n: &BigUint) -> BigUint {
    if a >= b {
        a - b
    } else {
        n - (b - a)
    }
}

fn main() {
    let number_problem_str = std::env::args_os().nth(1).unwrap_or_default();
    if number_problem_str.is_empty() {
        panic!("usage: pollard-rho <problem number>")
    }

    let number_problem = number_problem_str.into_string().unwrap().parse::<u32>().unwrap();
    let (p, alpha, pub_a, pub_b) = parameters(number_problem, "desafios.txt");
    let n = ord(&alpha, &p);
    
    let mut x: BigUint;
    let mut a = BigUint::from(0u8);
    let mut b: BigUint;
    let mut x2: BigUint;
    let mut a2 = BigUint::from(0u8);
    let mut b2: BigUint;
    let mut r = BigUint::from(0u8);

    for i in 1.. {
        a = i.to_biguint().unwrap();
        b = i.to_biguint().unwrap();
        x = (alpha.modpow(&a, &p) * pub_a.modpow(&b, &p)) % &p;

        x2 = x.clone();
        a2 = a.clone();
        b2 = b.clone();

        loop {
            (x, a, b) = step(x, a, b, &p, &pub_a, &alpha, &n);

            (x2, a2, b2) = step(x2, a2, b2, &p, &pub_a, &alpha, &n);
            (x2, a2, b2) = step(x2, a2, b2, &p, &pub_a, &alpha, &n);

            if x == x2 {
                break;
            }
        }

        r = sub_mod(&b, &b2, &n);
        if r != BigUint::ZERO {
            break
        }
    }
    let r_inv = r.modinv(&n).unwrap();
    let alice_private_key = (r_inv * sub_mod(&a2, &a, &n)) % &n;
    println!("k_ab = {}", pub_b.modpow(&alice_private_key, &p))
}
