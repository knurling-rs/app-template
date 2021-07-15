### User Survey

> We love feedback! Please give us < 5 minutes of your time so we can get to know our users better.
> 
> Help us shape knurling by filling out [this short survey](https://forms.office.com/r/aMfHG79N9K).


# `app-template`

> Quickly set up a [`probe-run`] + [`defmt`] + [`flip-link`] embedded project

[`probe-run`]: https://crates.io/crates/probe-run
[`defmt`]: https://github.com/knurling-rs/defmt
[`flip-link`]: https://github.com/knurling-rs/flip-link

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

#### 3. [`cargo-generate`]:

``` console
$ cargo install cargo-generate
```

[`cargo-generate`]: https://crates.io/crates/cargo-generate

> *Note:* You can also just clone this repository instead of using `cargo-generate`, but this involves additional manual adjustments.

## Setup

#### 1. Initialize the project template

``` console
$ cargo generate \
    --git https://github.com/knurling-rs/app-template \
    --branch main \
    --name my-app
```

If you look into your new `my-app` folder, you'll find that there are a few `TODO`s in the files marking the properties you need to set.

Let's walk through them together now.

#### 2. Set `probe-run` chip

Pick a chip from `probe-run --list-chips` and enter it into `.cargo/config.toml`.

If, for example, you have a nRF52840 Development Kit from one of [our workshops], replace `{{chip}}` with `nRF52840_xxAA`.

[our workshops]: https://github.com/ferrous-systems/embedded-trainings-2020

``` diff
 # .cargo/config.toml
 [target.'cfg(all(target_arch = "arm", target_os = "none"))']
-runner = "probe-run --chip {{chip}}"
+runner = "probe-run --chip nRF52840_xxAA"
```

#### 3. Adjust the compilation target

In `.cargo/config.toml`, pick the right compilation target for your board.

``` diff
 # .cargo/config.toml
 [build]
-target = "thumbv6m-none-eabi"    # Cortex-M0 and Cortex-M0+
-# target = "thumbv7m-none-eabi"    # Cortex-M3
-# target = "thumbv7em-none-eabi"   # Cortex-M4 and Cortex-M7 (no FPU)
-# target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU)
+target = "thumbv7em-none-eabihf" # Cortex-M4F (with FPU)
```

Add the target with `rustup`.

``` console
$ rustup target add thumbv7em-none-eabihf
```

#### 4. Add a HAL as a dependency

In `Cargo.toml`, list the Hardware Abstraction Layer (HAL) for your board as a dependency.

For the nRF52840 you'll want to use the [`nrf52840-hal`].

[`nrf52840-hal`]: https://crates.io/crates/nrf52840-hal

``` diff
 # Cargo.toml
 [dependencies]
-# some-hal = "1.2.3"
+nrf52840-hal = "0.12.0"
```

#### 5. Import your HAL

Now that you have selected a HAL, fix the HAL import in `src/lib.rs`

``` diff
 // my-app/src/lib.rs
-// use some_hal as _; // memory layout
+use nrf52840_hal as _; // memory layout
```

#### (6. Get a linker script)

Some HAL crates require that you manually copy over a file called `memory.x` from the HAL to the root of your project. For nrf52840-hal, this is done automatically so no action is needed. For other HAL crates, you can get it from your local Cargo folder, the default location is under:

```
~/.cargo/registry/src/
```

Not all HALs provide a `memory.x` file, you may need to write it yourself. Check the documentation for the HAL you are using.


#### 7. Run!

You are now all set to `cargo-run` your first `defmt`-powered application!
There are some examples in the `src/bin` directory.

Start by `cargo run`-ning `my-app/src/bin/hello.rs`:

``` console
$ # `rb` is an alias for `run --bin`
$ cargo rb hello
    Finished dev [optimized + debuginfo] target(s) in 0.03s
flashing program ..
DONE
resetting device
0.000000 INFO Hello, world!
(..)

$ echo $?
0
```

#### (8. Set `rust-analyzer.linkedProjects`)

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

## Trying out the git version of defmt

This template is configured to use the latest crates.io release (the "stable" release) of the `defmt` framework.
To use the git version (the "development" version) of `defmt` follow these steps:

1. Install the *git* version of `probe-run`

``` console
$ cargo install --git https://github.com/knurling-rs/probe-run --branch main
```

2. Check which defmt version `probe-run` supports

``` console
$ probe-run --version
0.2.0 (aa585f2 2021-02-22)
supported defmt version: 60c6447f8ecbc4ff023378ba6905bcd0de1e679f
```

In the example output, the supported version is `60c6447f8ecbc4ff023378ba6905bcd0de1e679f`

3. Switch defmt dependencies to git: uncomment the last part of the root `Cargo.toml` and enter the hash reported by `probe-run --version`:

``` diff
-# [patch.crates-io]
-# defmt = { git = "https://github.com/knurling-rs/defmt", rev = "use defmt version reported by `probe-run --version`" }
-# defmt-rtt = { git = "https://github.com/knurling-rs/defmt", rev = "use defmt version reported by `probe-run --version`" }
-# defmt-test = { git = "https://github.com/knurling-rs/defmt", rev = "use defmt version reported by `probe-run --version`" }
-# panic-probe = { git = "https://github.com/knurling-rs/defmt", rev = "use defmt version reported by `probe-run --version`" }
+[patch.crates-io]
+defmt = { git = "https://github.com/knurling-rs/defmt", rev = "60c6447f8ecbc4ff023378ba6905bcd0de1e679f" }
+defmt-rtt = { git = "https://github.com/knurling-rs/defmt", rev = "60c6447f8ecbc4ff023378ba6905bcd0de1e679f" }
+defmt-test = { git = "https://github.com/knurling-rs/defmt", rev = "60c6447f8ecbc4ff023378ba6905bcd0de1e679f" }
+panic-probe = { git = "https://github.com/knurling-rs/defmt", rev = "60c6447f8ecbc4ff023378ba6905bcd0de1e679f" }
```

You are now using the git version of `defmt`!

**NOTE** there may have been breaking changes between the crates.io version and the git version; you'll need to fix those in the source code.

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
