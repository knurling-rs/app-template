#!/bin/sh

set -ex

project="test-app"

echo "Installing necessary tools"
cargo install flip-link cargo-generate sd probe-run

echo "Cleaning up old project"
rm -rf "$project"

echo "Creating new project"
cargo generate -p . --name "$project"

echo "Storing current config so that the child project will compile."
mv Cargo.toml Cargo.toml.tmp
mv .cargo/config.toml .cargo/config.toml.tmp

cd "$project"

echo "Performing steps"

sd -s -- '--chip $CHIP' '--chip nRF52840_xxAA' .cargo/config.toml
sd -s '# target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU)' 'target = "thumbv7em-none-eabihf"' .cargo/config.toml
sd -s '$RTIC_BACKEND' 'thumbv7-backend' Cargo.toml
sd -s '# some-hal = "1.2.3"' 'nrf52840-hal = "0.16.0"' Cargo.toml
sd -s '// use some_hal as _; // memory layout' 'use nrf52840_hal as _;' src/lib.rs
sd -s 'some_hal::pac' 'nrf52840_hal::pac' src/bin/minimal.rs
sd -s 'FreeInterrupt1, ...' 'SWI0_EGU0' src/bin/minimal.rs

cargo bbr minimal

cd ..
echo "Cleaning up"
mv Cargo.toml.tmp Cargo.toml
mv .cargo/config.toml.tmp .cargo/config.toml