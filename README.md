# Discrete Logarithm Problem (dlp)

Rust implementation of algorithms to solve the discrete logarithm problem (DLP) using Pohlig–Hellman and Pollard's Rho for logarithms. The repository includes challenge parameters, a solution file, and source code to compute shared keys for given parameters.

Author: Pedro Arthur Pamplona Hartmann

## What this does

Given a scenario index (from desafios.txt) this program recovers the private exponent a such that A = alpha^a (mod p) using Pohlig–Hellman (factoring p-1 and solving subproblems) and Pollard's Rho to solve the smaller discrete logarithm subproblems. It then computes the shared key K_ab = B^a (mod p) and prints the result and runtime.

## Stack
- Language: Rust (edition 2024)
- Notable crates: fxhash, num-bigint, num-prime, num-traits, rayon

## Repository layout

```
Cargo.toml         # Rust manifest
src/                # Rust source
  main.rs           # CLI & high-level algorithm (Pohlig–Hellman orchestration)
  pollard_rho_log.rs# Pollard's Rho implementation (parallel)
  file.rs           # input parser for desafios.txt
desafios.txt        # challenge parameters (scenarios C0..C22)
solucao.txt         # expected output file for K_ab values (empty / exercise)
```

## Requirements

- Rust toolchain (rustc + cargo). Recommended: stable toolchain supporting edition 2024.
- Optional: GNU `factor` command in PATH (used by the program to factor p-1 more quickly). If `factor` is not available, the code falls back to the Rust `num-prime` factorization.
- A multicore CPU helps: the Pollard Rho implementation uses rayon for parallelism.

## Build

From the repository root:

```bash
# build optimized binary
cargo build --release
```

## Run

The program expects a single numeric argument selecting the scenario number (the index used in desafios.txt). Example to run scenario C1:

```bash
# run from cargo (convenient during development)
cargo run --release -- 1

# or run the built binary directly
./target/release/dlp 1
```

Output (example):

```
p	= 3008149519
alpha	= 223722476
A	= 1957790020
B	= 1354383354
k_ab	= <computed value>
It took X.XXXX seconds
```

Notes:
- The program reads desafios.txt in the repository root to load parameters for the chosen scenario index.
- The program performs a factorization of p-1; if `factor` is available it will call it, otherwise it will use the bundled Rust factorization routine.
- For some scenarios (large p or unfavourable factorization of p-1) execution can be very slow or infeasible on commodity hardware.

## Tests / Data

- Challenge parameters: `desafios.txt` contains scenarios C0..C22 (C0 is a small worked example with provided a, b and K_ab for sanity checking).
- `solucao.txt` is intended to collect the computed K_ab values.

## Design notes

- The main code uses Pohlig–Hellman to reduce the discrete logarithm to smaller moduli (factors of p-1) and solves each subproblem using a parallel Pollard's Rho implementation.
- The Pollard implementation follows the van Oorschot–Wiener idea of storing distinguished points in a B-Tree and uses rayon to parallelize search across starting seeds.
- For small orders (<= 16 bits) the algorithm tries a direct brute-force.

## Dependencies

See `Cargo.toml`. Key crates:
- fxhash — fast hashing used for seed generation and distinguished-point selection
- num-bigint, num-traits — big-integer arithmetic
- num-prime — factorization fallback
- rayon — parallel iteration

## Caveats & Security

This implementation is for educational and experimental use only. It demonstrates cryptanalytic techniques against Diffie–Hellman groups with weak parameter choices (p where p-1 has small prime factors). Do not use these techniques to attack systems you do not own. Use responsibly.
