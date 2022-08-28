# Caesium
## Rust physics simulation game

Caesium is an attempt at making a physics simulation game that is as accurate as possible.
At the moment it's more of a challenge project for myself to improve my knowledge of Rust and physics simulations.

## Possible roadmap features (not yet confirmed)

- N-body simulation (Classical Newtonian gravity)
- Chaos simulation
- Electrostatic interation simulation
- Radiation 

The idea of the project isn't very concrete at the moment, and this README will change many times I'm sure.
My main focus is to make sure the simulations are accurate through extensive automated testing, and then expanding upon these models to create a featureful game-like experience.

## Tech
I'm writing this project in Rust as I've come to love working with it recently. Due to this project being very calculation/resource heavy, it's imperative to use a lower-level language with as little bloat as possible.

At the moment the project uses [Piston2D](https://github.com/pistondevelopers/graphics) for graphics. I intend to re-write the renderer myself at a later stage once I understand more about the optimisations I could make to it for this specific use case.

## Installation

First you'll need to install the [Rust toolchain](https://www.rust-lang.org/tools/install).

Then clone the repo and build the project with Cargo.

```sh
git clone https://github.com/ZakFarmer/Caesium.git
cd Caesium
cargo build
```

Then you can run the binary from the target directory.

Alternatively, you can build and run the program all in one step.

```sh
cargo run --release
```
Running with the release flag will result in better performance generally.

## Testing

To run the test suite, use the below Cargo command.
```sh
cargo test
```
