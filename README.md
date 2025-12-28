# Stepcount

A Rust implementation of the stepcount algorithm from the YouTube video [Code Optimisation via Memoization - Computerphile](https://www.youtube.com/watch?v=JXUOMsFBDXQ).

The stepcount module contains a naive implementation of the algorithm not using any memoization. Just don't run this with too big inputs. It's designed for educational purposes and may not be efficient for large inputs.

## Usage

```shell
cargo run -- [method] [stepcount] [steps]
```

For the naive stepcount algorithm:
```shell
cargo run -- stepcount 10 1,3,5
```