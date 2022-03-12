# Rust-Calculator

This repo hosts the code for a calculator I built using Rust.

## Set Up

1. Make sure you have [Rust with cargo installed][1].
2. Clone the repository.
3. Open the project directory, `cd ~/myRepos/rust-calculator`
4. In the root of the project, create a local release of the calculator.
  - `$ cargo build --release`
5. Run the binary with a value you'd like to calculate:
  - `$ target/release/rust-calculator 1080 * 48`
6. If you get results like below, then it worked!
  - `1080 * 48 = 51840`

[1]: https://doc.rust-lang.org/cargo/getting-started/installation.html
