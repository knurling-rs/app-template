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
$ cargo install probe-run
```

#### 3. [`kickstart`]:

```
$ cargo install kickstart
```

[`kickstart`]: https://github.com/Keats/kickstart

> *Note:* You can also just clone this repository instead of using `kickstart`, but this involves additional manual adjustments.

## Setup

#### 1. Initialize the project template

``` console
$ kickstart \
    https://github.com/knurling-rs/app-template
```

Answer the questions (or use defaults) to generate your project.


#### 2. Get a linker script

Some HAL crates require that you manually copy over a file called `memory.x` from the HAL to the root of your project. For nrf52840-hal, this is done automatically so no action is needed. For other HAL crates, you can get it from your local Cargo folder, the default location is under:

```
~/.cargo/registry/src/
```

Not all HALs provide a memory.x file, you may need to write it yourself. Check the documentation for the HAL you are using.


#### 3. Run!

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
probe-run 0.1.4 (3521a42 2020-11-12)
supported defmt version: 3db6b41f08a5c866e6d6ed7103d01b0b0fe5a1f4
```

In the example output, the supported version is `3db6b41f08a5c866e6d6ed7103d01b0b0fe5a1f4`

3. Switch defmt dependencies to git: uncomment the last part of the root `Cargo.toml` and enter the hash reported by `probe-run --version`:

``` diff
-# [patch.crates-io]
-# defmt = { git = "https://github.com/knurling-rs/defmt", rev = "use defmt version reported by `probe-run --version`" }
-# defmt-rtt = { git = "https://github.com/knurling-rs/defmt", rev = "use defmt version reported by `probe-run --version`" }
-# defmt-test = { git = "https://github.com/knurling-rs/defmt", rev = "use defmt version reported by `probe-run --version`" }
-# panic-probe = { git = "https://github.com/knurling-rs/defmt", rev = "use defmt version reported by `probe-run --version`" }
+[patch.crates-io]
+defmt = { git = "https://github.com/knurling-rs/defmt", rev = "3db6b41f08a5c866e6d6ed7103d01b0b0fe5a1f4" }
+defmt-rtt = { git = "https://github.com/knurling-rs/defmt", rev = "3db6b41f08a5c866e6d6ed7103d01b0b0fe5a1f4" }
+defmt-test = { git = "https://github.com/knurling-rs/defmt", rev = "3db6b41f08a5c866e6d6ed7103d01b0b0fe5a1f4" }
+panic-probe = { git = "https://github.com/knurling-rs/defmt", rev = "3db6b41f08a5c866e6d6ed7103d01b0b0fe5a1f4" }
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
