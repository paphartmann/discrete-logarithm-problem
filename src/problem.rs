use num_bigint::*;

pub struct Problem {
    pub prime_modulus: BigUint,
    pub alpha_generator: BigUint,
    pub a_public_exp: BigUint,
    pub b_public_exp: BigUint,
}
