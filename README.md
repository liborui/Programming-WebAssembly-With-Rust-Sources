# Programming-WebAssembly-With-Rust-Sources
Source codes of book "Programming WebAssembly With Rust"

# Preparation
```bash
sudo apt-get install build-essential
sudo apt-get install cmake
git clone --recursive https://github.com/WebAssembly/wabt
cd wabt
mkdir build
cd build
cmake ..
cmake --build .
```

```
alias wat2wasm="~/wabt/bin/wat2wasm"
```

# Version
rustc 1.41.0 (5e1a79984 2020-01-27)
npm install webpack -g

# To Run

## Champ.6
```bash
cargo run
```

# Attention/Bug
In Ubuntu 18.04, using ```apt``` to install npm (Webapp depends on) and libssl-dev (some Rust crates depends on) is conflict to each other, which most probably is the confliction between libssl1.0-dev (npm depends on) and libssl-dev.
Installing libssl-dev will automatically remove npm.
