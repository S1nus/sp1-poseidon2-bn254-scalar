# SP1 Accelerated Poseidon2 Hash Function (bn256)

This repository contains the Rust implementation of [Poseidon2](https://eprint.iacr.org/2023/323.pdf) over the bn256 curve using the [SP1 patched `crypto-bigint`](https://github.com/sp1-patches/RustCrypto-bigint) to enable [highly performant](https://docs.succinct.xyz/docs/sp1/optimizing-programs/precompiles) hashing in the SP1 zkVM context.

## Acknowledgements

Built on the shoulders of Giants: <https://github.com/HorizenLabs/poseidon2> laid the foundation for this work.
