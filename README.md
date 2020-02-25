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