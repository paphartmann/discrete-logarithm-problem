use dashmap::DashMap;
use fxhash::hash64;
use rayon::prelude::*;
use num_bigint::{BigUint, ToBigUint};
use std::{sync::{Arc, atomic::{AtomicBool, Ordering}}};
use num_traits::ToPrimitive;

fn step(
    x: &mut BigUint,
    a: &mut BigUint,
    b: &mut BigUint,
    p: &BigUint,
    beta: &BigUint,
    alpha: &BigUint,
    q: &BigUint) {
    let remainder = unsafe {(&*x % 3u8).to_u8().unwrap_unchecked()};

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
    debug_assert!(a < n && b < n);
    if a >= b {
        a - b
    } else {
        n - (b - a)
    }
}

fn pollard_rho(
    i: u32,
    p: &BigUint,
    alpha: &BigUint,
    beta: &BigUint,
    ord: &BigUint,
    stop_cond: Arc<AtomicBool>,
    shared_list: Arc<DashMap<BigUint, (BigUint, BigUint)>>
) -> Option<BigUint> {
    let mut ai = hash64(&i).to_biguint()? % ord;
    let mut bi = hash64(&(i+1)).to_biguint()? % ord;
    let mut xi = (alpha.modpow(&ai, p) * beta.modpow(&bi, p)) % p;

    let a2i: BigUint;
    let b2i: BigUint;
    let dp_bits = (ord.bits()/8 + 2) as u32;

    loop {
        step(&mut xi, &mut ai, &mut bi, p, beta, alpha, ord);

        if hash64(&xi).leading_zeros() >= dp_bits {
            if let Some(entry) = shared_list.get(&xi) {
                let (cand_a, cand_b) = entry.value();
                a2i = cand_a.clone();
                b2i = cand_b.clone();
                break
            } else {
                shared_list.insert(xi, (ai,bi));
                return None
            }
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
        eprintln!("Algorithm found wrong logarithm, continuing...");
        None
    }
}

pub fn find_log(
    p: &BigUint,
    alpha: &BigUint,
    beta: &BigUint,
    ord: &BigUint,
) -> BigUint {
    if ord.bits() <= 16 {
        let mut i = BigUint::ZERO;
        while i <= *ord {
            if alpha.modpow(&i, &p) == *beta {
                return i
            }
            i += 1u8;
        }
        unreachable!()
    } else {
        let stop = Arc::new(AtomicBool::new(false));
        let shared_list = Arc::new(DashMap::new());

        (0u32..u32::MAX).into_par_iter().find_map_any(|i| -> Option<BigUint> {
            pollard_rho(i, p, alpha, beta, ord, Arc::clone(&stop), Arc::clone(&shared_list))
        }).unwrap()
    }
}
