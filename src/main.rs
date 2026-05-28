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
        let a_next = (a << 1u8) % q;
        let b_next = (b << 1u8) % q;
        (x_next, a_next, b_next)

    } else {
        let x_next = (alpha * &x) % p;
        let a_next = (a + 1u8) % q;
        let b_next = b;
        (x_next, a_next, b_next)
    }
}

fn main() {
    let number_problem_str = std::env::args_os().nth(1).unwrap_or_default();
    if number_problem_str.is_empty() {
        panic!("usage: pollard-rho <problem number>")
    }

    let number_problem = number_problem_str.into_string().unwrap().parse::<u32>().unwrap();
    let (p, alpha, pub_a, pub_b) = parameters(number_problem, "desafios.txt".to_string());
    let q = (&p - 1u8) / 2u8;
    let n = &p - 1u8;
    let mut inv_r_option: Option<BigUint> = None;
    
    let mut x: BigUint;
    let mut a: BigUint = BigUint::ZERO;// compiler wouldnt compile if not initialized
    let mut b: BigUint;
    let mut x2: BigUint;
    let mut a2: BigUint = BigUint::ZERO;
    let mut b2: BigUint;

    for i in 1.. {
        a = i.to_biguint().unwrap();
        b = i.to_biguint().unwrap();
        x = alpha.modpow(&a, &q) * pub_a.modpow(&b, &q);

        x2 = x.clone();
        a2 = a.clone();
        b2 = b.clone();

        loop {
            (x, a, b) = step(x, a, b, &p, &pub_a, &alpha, &q);

            (x2, a2, b2) = step(x2, a2, b2, &p, &pub_a, &alpha, &q);
            (x2, a2, b2) = step(x2, a2, b2, &p, &pub_a, &alpha, &q);

            if x == x2 {
                break;
            }
        }

        let r = if b >= b2 {
            (&b - &b2) % &n
        } else {
            (&n + &b - &b2) % &n
        };
        if r == BigUint::ZERO {
            panic!("r == 0")
        }
        inv_r_option = r.modinv(&n);
        if inv_r_option == None {
            println!("r doesnt have an inverse");
            println!("r == {}", r);
            println!("x == {}, x2 == {}", x, x2);
            println!("a == {}, a2 == {}", a, a2);
            println!("b == {}, b2 == {}", b, b2);
            // panic!()
        } else {
            break;
        }
    }
    let inv_r = inv_r_option.unwrap();
    let delta_a = if a2 >= a {
        (&a2 - &a) % &n
    } else {
        (&n + &a2 - &a) % &n
    };
    let alice_private_key = (inv_r * delta_a) % &n;
    println!("k_ab = {}", pub_b.modpow(&alice_private_key, &p))
}
