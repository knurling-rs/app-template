# `app-template`

> Quickly set up a [`probe-rs`] + [`defmt`] + [`flip-link`] embedded project

[`probe-rs`]: https://crates.io/crates/probe-rs
[`defmt`]: https://github.com/knurling-rs/defmt
[`flip-link`]: https://github.com/knurling-rs/flip-link

## Dependencies

### 1. `flip-link`:

```bash
cargo install flip-link
```

### 2. `probe-rs`:

Install probe-rs by following the instructions at <https://probe.rs/docs/getting-started/installation/>.

### 3. [`cargo-generate`]:

```bash
cargo install cargo-generate
```

[`cargo-generate`]: https://crates.io/crates/cargo-generate

> *Note:* You can also just clone this repository instead of using `cargo-generate`, but this involves additional manual adjustments.

## Setup

### 1. Initialize the project template

```bash
cargo generate \
    --git https://github.com/knurling-rs/app-template \
    --branch main \
    --name my-app
```

If you look into your new `my-app` folder, you'll find that there are a few `TODO`s in the files marking the properties you need to set.

Let's walk through them together now.

### 2. Set `probe-rs` chip

Pick a chip from ` probe-rs chip list` and enter it into `.cargo/config.toml`.

If, for example, you have a nRF52840 Development Kit as used in one of [our exercises], replace `{{chip}}` with `nRF52840_xxAA`.

[our workshops]: https://rust-exercises.ferrous-systems.com

```diff
 # .cargo/config.toml
-runner = ["probe-rs", "run", "--chip", "$CHIP", "--log-format=oneline"]
+runner = ["probe-rs", "run", "--chip", "nRF52840_xxAA", "--log-format=oneline"]
```

### 3. Adjust the compilation target

In `.cargo/config.toml`, pick the right compilation target for your board.

```diff
 # .cargo/config.toml
 [build]
-target = "thumbv6m-none-eabi"    # Cortex-M0 and Cortex-M0+
-# target = "thumbv7m-none-eabi"    # Cortex-M3
-# target = "thumbv7em-none-eabi"   # Cortex-M4 and Cortex-M7 (no FPU)
-# target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU)
+target = "thumbv7em-none-eabihf" # Cortex-M4F (with FPU)
```

Add the target with `rustup`.

```bash
rustup target add thumbv7em-none-eabihf
```

### 4. Add a HAL as a dependency

In `Cargo.toml`, list the Hardware Abstraction Layer (HAL) for your board as a dependency.

For the nRF52840 you'll want to use the [`nrf52840-hal`].

[`nrf52840-hal`]: https://crates.io/crates/nrf52840-hal

```diff
 # Cargo.toml
 [dependencies]
-# some-hal = "1.2.3"
+nrf52840-hal = "0.14.0"
```

⚠️ Note for RP2040 users ⚠️

You will need to not just specify the `rp-hal` HAL, but a BSP (board support crate) which includes a second stage bootloader. Please find a list of available BSPs [here](https://github.com/rp-rs/rp-hal-boards#packages).

### 5. Import your HAL

Now that you have selected a HAL, fix the HAL import in `src/lib.rs`

```diff
 // my-app/src/lib.rs
-// use some_hal as _; // memory layout
+use nrf52840_hal as _; // memory layout
```

### (6. Get a linker script)

Some HAL crates require that you manually copy over a file called `memory.x` from the HAL to the root of your project. For nrf52840-hal, this is done automatically so no action is needed. For other HAL crates, see their documentation on where to find an example file.

The `memory.x` file should look something like:

```text
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 1024K
  RAM   : ORIGIN = 0x20000000, LENGTH = 256K
}
```

The `memory.x` file is included in the `cortex-m-rt` linker script `link.x`, and so `link.x` is the one you should tell `rustc` to use (see the `.cargo/config.toml` file where we do that).

### 7. Run!

You are now all set to `cargo-run` your first `defmt`-powered application!
There are some examples in the `src/bin` directory.

Start by `cargo run`-ning `my-app/src/bin/hello.rs`:

```console
$ # `rb` is an alias for `run --bin`
$ cargo rb hello
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.01s
     Running `probe-rs run --chip nrf52840_xxaa --log-format=oneline target/thumbv6m-none-eabi/debug/hello`
      Erasing ✔ 100% [####################]   8.00 KiB @  15.79 KiB/s (took 1s)
  Programming ✔ 100% [####################]   8.00 KiB @  13.19 KiB/s (took 1s)                                                                                                                        Finished in 1.11s
Hello, world!

$ echo $?
0
```

If you're running out of memory (`flip-link` bails with an overflow error), you can decrease the size of the device memory buffer by setting the `DEFMT_RTT_BUFFER_SIZE` environment variable. The default value is 1024 bytes, and powers of two should be used for optimal performance:

```console
$ DEFMT_RTT_BUFFER_SIZE=64 cargo rb hello
```

### (8. Set `rust-analyzer.linkedProjects`)

If you are using [rust-analyzer] with VS Code for IDE-like features you can add following configuration to your `.vscode/settings.json` to make it work transparently across workspaces. Find the details of this option in the [RA docs].

```json
{
    "rust-analyzer.linkedProjects": [
        "Cargo.toml",
        "firmware/Cargo.toml",
    ]
}
```

[RA docs]: https://rust-analyzer.github.io/manual.html#configuration
[rust-analyzer]: https://rust-analyzer.github.io/

## Running tests

The template comes configured for running unit tests and integration tests on the target.

Unit tests reside in the library crate and can test private API; the initial set of unit tests are in `src/lib.rs`.
`cargo test --lib` will run those unit tests.

```console
$ cargo test --lib
   Compiling example v0.1.0 (./knurling-rs/example)
    Finished `test` profile [optimized + debuginfo] target(s) in 0.15s
     Running unittests src/lib.rs (target/thumbv6m-none-eabi/debug/deps/example-2b0d0e25d141bf57)
      Erasing ✔ 100% [####################]   8.00 KiB @  15.99 KiB/s (took 1s)
  Programming ✔ 100% [####################]   8.00 KiB @  13.33 KiB/s (took 1s)                                                                                                                        Finished in 1.10s
(1/1) running `it_works`...
all tests passed!
```

Integration tests reside in the `tests` directory; the initial set of integration tests are in `tests/integration.rs`.
`cargo test --test integration` will run those integration tests.
Note that the argument of the `--test` flag must match the name of the test file in the `tests` directory.

```console
$ cargo test --test integration
   Compiling example v0.1.0 (./knurling-rs/example)
    Finished `test` profile [optimized + debuginfo] target(s) in 0.10s
     Running tests/integration.rs (target/thumbv6m-none-eabi/debug/deps/integration-aaaff41151f6a722)
      Erasing ✔ 100% [####################]   8.00 KiB @  16.03 KiB/s (took 0s)
  Programming ✔ 100% [####################]   8.00 KiB @  13.19 KiB/s (took 1s)                                                                                                                        Finished in 1.11s
(1/1) running `it_works`...
all tests passed!
```

Note that to add a new test file to the `tests` directory you also need to add a new `[[test]]` section to `Cargo.toml`.

To run all the tests via `cargo test` the tests need to be explicitly disabled for all the existing binary targets.
See `Cargo.toml` for details on how to do this.

## Support

`app-template` is part of the [Knurling] project, [Ferrous Systems]' effort at
improving tooling used to develop for embedded systems.

If you think that our work is useful, consider sponsoring it via [GitHub
Sponsors].

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.

[Knurling]: https://knurling.ferrous-systems.com
[Ferrous Systems]: https://ferrous-systems.com/
[GitHub Sponsors]: https://github.com/sponsors/knurling-rs
