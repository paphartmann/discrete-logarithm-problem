use num_bigint::*;

pub struct Problem {
    pub prime_modulus: BigInt,
    pub alpha_generator: BigInt,
    pub a_public_exp: BigInt,
    pub b_public_exp: BigInt,
}
