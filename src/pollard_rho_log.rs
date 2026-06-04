use rayon::prelude::*;
use num_bigint::{BigUint, ToBigUint};
use std::{hash::*, sync::{Arc, atomic::{AtomicBool, Ordering}}};
use num_traits::ToPrimitive;

fn step(
    x: &mut BigUint,
    a: &mut BigUint,
    b: &mut BigUint,
    p: &BigUint,
    beta: &BigUint,
    alpha: &BigUint,
    q: &BigUint) {
    let remainder = (&*x % 3u8).to_u8().unwrap();

    match remainder {
        0 => {
            *x *= beta;
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

fn sub_mod(a: &BigUint, b: &BigUint, n: &BigUint) -> BigUint {
    if a > b {
        a - b
    } else {
        n - (b - a)
    }
}

fn pollard_rho(
    i: u64,
    p: &BigUint,
    alpha: &BigUint,
    beta: &BigUint,
    ord: &BigUint,
    stop_cond: Arc<AtomicBool>
) -> Option<BigUint> {
    // println!("{i}");
    let mut hasher = DefaultHasher::new();
    hasher.write_u64(i);
    let mut ai = hasher.finish().to_biguint()? % ord;
    hasher.write_u64(1);
    let mut bi = hasher.finish().to_biguint()? % ord;
    let mut xi = (alpha.modpow(&ai, p) * beta.modpow(&bi, p)) % p;

    let mut x2i = xi.clone();
    let mut a2i = ai.clone();
    let mut b2i = bi.clone();

    loop {
        step(&mut xi, &mut ai, &mut bi, p, beta, alpha, ord);

        step(&mut x2i, &mut a2i, &mut b2i, p, beta, alpha, ord);
        step(&mut x2i, &mut a2i, &mut b2i, p, beta, alpha, ord);

        if xi == x2i {
            break;
        } else if stop_cond.load(Ordering::Relaxed) {
            return None
        }
    }

    let r = sub_mod(&bi, &b2i, ord);

    if r == BigUint::ZERO {
        return None;
    }

    let r_inv = r.modinv(ord)?;
    let result = (r_inv * sub_mod(&a2i, &ai, ord)) % ord;
    if alpha.modpow(&result, p) == *beta {
        stop_cond.store(true, Ordering::Relaxed);
        Some(result)
    } else {
        None
    }
}

pub fn find_log(
    p: &BigUint,
    alpha: &BigUint,
    beta: &BigUint,
    ord: &BigUint,
) -> BigUint {
    let stop = Arc::new(AtomicBool::new(false));

    (1u64..u64::MAX).into_par_iter().find_map_any(|i| -> Option<BigUint> {
        pollard_rho(i, p, alpha, beta, ord, stop.clone())
    }).unwrap()
}
