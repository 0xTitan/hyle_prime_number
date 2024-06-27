# Hylé example RISC Zero smart contract

This repository provides an example smart contract for Hylé, implementing prime number check.

## Installing RISC Zero

Please refer to [RiscZero's installation guide](https://dev.risczero.com/api/zkvm/install)

## Reproducible builds

RISC Zero provides using a docker setup. Simply run
```bash
cargo risczero build --manifest-path methods/guest/Cargo.toml
```
to build the smart contract.

The reproducible Image ID of this smart contract is currently `0xe3ad45dc2d868a97efa556e0f6223726a260fca2f4f61200b0107c78f6e3114c`


## Running the smart contract

```bash
cargo run next X # Generate a proof of the resumt for prime number check where X is the number you want to check
# Or reproducibly
cargo run -- -r next X
```

Expected output for number 7 with command ```bash cargo run next 7 ``` :

```
...
Method ID: Digest(e3ad45dc2d868a97efa556e0f6223726a260fca2f4f61200b0107c78f6e3114c) (hex)
proof.json written, transition from AAAABw== (7) to AAAAAQ== (1)
...
```

### Verifying locally

Install the [Hylé RISC Zero verifier](https://github.com/Hyle-org/hyle-risc-zero-verifier).
You can then verify proofs using:
```sh
# The verifier currently expects no `0x` prefix. Pass data as base64 values.
cargo run -p risc0-verifier e3ad45dc2d868a97efa556e0f6223726a260fca2f4f61200b0107c78f6e3114c [path_to_proof] [initial_state] [final_state]
```
If the proof is malformed, or doesn't respect the rules of the smart contract, the verifier will return an error.

## Verifying on Hylé

Once you [installed the CLI](https://docs.hyle.eu/getting-started/hyled-install-instructions/) and got [connected to devnet](https://docs.hyle.eu/getting-started/connect-to-devnet/), you should be able to [_register_ and _execute_ for your contract](https://docs.hyle.eu/getting-started/your-first-smart-contract/).
