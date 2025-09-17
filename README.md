# PineTime Rust Firmware

This repository is a minimal Rust firmware template for the PineTime smartwatch (nRF52832).

## Features

- Cortex-M4F (nRF52832) support
- No-std, embedded Rust
- Easily extensible for display, BLE, sensors

## Getting Started

1. Install Rust and add the ARM target:
   ```sh
   rustup target add thumbv7em-none-eabihf
   ```

2. Build firmware:
   ```sh
   cargo build --release
   ```

3. Flash using SWD programmer.

## License

MIT
