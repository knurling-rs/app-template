# `app-template`

> Quickly set up a [`probe-run`] + [`defmt`] + [`flip-link`] embedded project
> running on the [`RTIC`] scheduler

[`probe-run`]: https://crates.io/crates/probe-run
[`defmt`]: https://github.com/knurling-rs/defmt
[`flip-link`]: https://github.com/knurling-rs/flip-link
[`RTIC`]: https://rtic.rs/

Based on https://github.com/knurling-rs/app-template

## Dependencies

#### 1. `flip-link`:

```console
$ cargo install flip-link
```

#### 2. `probe-run`:

``` console
$ # make sure to install v0.2.0 or later
$ cargo install probe-run
```

## Setup

#### 1. Clone the project template

``` console
$ git clone https://github.com/rtic-rs/app-template test-app
```

If you look into your new `test-app` folder, you'll find that there are a few `TODO`s in the files marking the properties you need to set. The todo's are formatted as `TODO(n)`, where `n` is the number of the step in which the TODO is explained.

Let's walk through them together now.

#### 2. Set `probe-run` chip

Pick a chip from `probe-run --list-chips` and enter it into `.cargo/config.toml`.

If, for example, you have a nRF52840 Development Kit from one of [our workshops], replace `$CHIP` with `nRF52840_xxAA`.

[our workshops]: https://github.com/ferrous-systems/embedded-trainings-2020

```diff
# .cargo/config.toml
 [target.'cfg(all(target_arch = "arm", target_os = "none"))']
-runner = "probe-run --chip $CHIP"
+runner = "probe-run --chip nRF52840_xxAA"
```

#### 3. Adjust the compilation target

In `.cargo/config.toml`, pick the right compilation target for your board.

``` diff
 # .cargo/config.toml
 [build]
-# target = "thumbv6m-none-eabi"    # Cortex-M0 and Cortex-M0+
-# target = "thumbv7m-none-eabi"    # Cortex-M3
-# target = "thumbv7em-none-eabi"   # Cortex-M4 and Cortex-M7 (no FPU)
-# target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU)
+target = "thumbv7em-none-eabihf" # Cortex-M4F (with FPU)
```

Add the target with `rustup`.

``` console
$ rustup +nightly target add thumbv7em-none-eabihf
```

#### 4. Activate the correct `rtic` backend

In `Cargo.toml`, activate the correct `rtic` backend for your target by replacing `$RTIC_BACKEND` with one of `thumbv6-backend`, `thumbv7-backend`, `thumbv8base-backend`, or `thumbv8main-backend`, depending on the target you are compiling for.

```diff
# Cargo.toml
-rtic = { version = "2.0.0", features = [ "$RTIC_BACKEND" ] }
+rtic = { version = "2.0.0", features = [ "thumbv7-backend" ] }
```

#### 5. Add a HAL as a dependency

In `Cargo.toml`, list the Hardware Abstraction Layer (HAL) for your board as a dependency.

For the nRF52840 you'll want to use the [`nrf52840-hal`].

[`nrf52840-hal`]: https://crates.io/crates/nrf52840-hal

```diff
# Cargo.toml
 [dependencies]
-some-hal = "1.2.3"
+nrf52840-hal = "0.16.0"
```

⚠️ Note for RP2040 users ⚠️

You will need to not just specify the `rp-hal` HAL, but a BSP (board support crate) which includes a second stage bootloader. Please find a list of available BSPs [here](https://github.com/rp-rs/rp-hal-boards#packages).

#### 6. Import your HAL

Now that you have selected a HAL, fix the HAL import in `src/lib.rs`

``` diff
# my-app/src/lib.rs
-use some_hal as _; // memory layout
+use nrf52840_hal as _; // memory layout
```

#### 7. Configure the `rtic::app` macro.

In `src/bin/minimal.rs`, edit the `rtic::app` macro into a valid form.

``` diff
# my-app/src/bin/minimal.rs
\#[rtic::app(
-    // TODO: Replace `some_hal::pac` with the path to the PAC
-    device = some_hal::pac,
-    // TODO: Replace the `FreeInterrupt1, ...` with free interrupt vectors if software tasks are used
-    // You can usually find the names of the interrupt vectors in the some_hal::pac::interrupt enum.
-    dispatchers = [FreeInterrupt1, ...]
+    device = nrf52840_hal::pac,
+    dispatchers = [SWI0_EGU0]
)]
```

#### (8. Get a linker script)

Some HAL crates require that you manually copy over a file called `memory.x` from the HAL to the root of your project. For nrf52840-hal, this is done automatically so no action is needed. For other HAL crates, you can get it from your local Cargo folder, the default location is under:

```
~/.cargo/registry/src/
```

Not all HALs provide a `memory.x` file, you may need to write it yourself. Check the documentation for the HAL you are using.


#### 9. Run!

You are now all set to `cargo-run` your first `defmt`-powered application!
There are some examples in the `src/bin` directory.

Start by `cargo run`-ning `my-app/src/bin/minimal.rs`:

``` console
$ # `rb` is an alias for `run --bin`
$ DEFMT_LOG=trace cargo rb minimal
    Finished dev [optimized + debuginfo] target(s) in 0.03s
flashing program ..
DONE
resetting device
0.000000 INFO Hello, world!
(..)

$ echo $?
0
```

If you're running out of memory (`flip-link` bails with an overflow error), you can decrease the size of the device memory buffer by setting the `DEFMT_BRTT_BUFFER_SIZE` environment variable. The default value is 1024 bytes, and powers of two should be used for optimal performance:

``` console
$ DEFMT_BRTT_BUFFER_SIZE=64 cargo rb minimal
```

[RA docs]: https://rust-analyzer.github.io/manual.html#configuration
[rust-analyzer]: https://rust-analyzer.github.io/

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
