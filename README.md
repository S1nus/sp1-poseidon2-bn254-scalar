# SP1 Accelerated Poseidon2 Hash Function (BN256)

This repository contains a Rust implementation of [Poseidon2](https://eprint.iacr.org/2023/323.pdf) over the Barreto–Naehrig curve with a 254-bit prime field (commonly referred to as `BN256`, also known as `BN254`, and `BN128`, or in Ethereum as `alt_bn128`).
It uses the [SP1-patched `crypto-bigint`](https://github.com/sp1-patches/RustCrypto-bigint) library to enable [high-performance](https://docs.succinct.xyz/docs/sp1/optimizing-programs/precompiles) hashing in the SP1 zkVM context.

## Acknowledgements

Built on the shoulders of Giants: <https://github.com/HorizenLabs/poseidon2> laid the foundation for this work.
