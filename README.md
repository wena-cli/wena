<p align="center">
    <img title="Wena" width="50%" src="https://raw.githubusercontent.com/wena-cli/wena/main/art/logo.png" />
</p>

<p align="center">
    <a href="https://crates.io/crates/wena"><img src="https://img.shields.io/crates/v/wena" alt="Build Status"></img></a>
    <a href="https://github.com/wena-cli/wena/actions"><img src="https://img.shields.io/github/workflow/status/wena-cli/wena/Tests.svg" alt="Build Status"></img></a>
    <a href="https://deps.rs/repo/github/wena-cli/wena"><img src="https://deps.rs/repo/github/wena-cli/wena/status.svg" alt="Dependency status"></a>
    <a href="https://github.com/wena-cli/wena/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-informational" alt="License"></a>
</p>

Wena was created by [Nuno Maduro](https://github.com/nunomaduro), and is a [Rust Lang](https://www.rust-lang.org) micro-framework that provides an elegant starting point for your console application.

> This project is a work-in-progress. Code and documentation are currently under development and are subject to change.

------

## Get Started

> **Requires [Rust 1.57.0](https://blog.rust-lang.org/2021/12/02/Rust-1.57.0.html)**

First, install a recent release of Rust via the [rustup](https://rustup.rs):

```sh
rustup default stable
```

Next, create a new binary-based [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) project:

```sh
cargo new my-cli-application --bin
```

Once the project is created, add `wena` as dependency in your `Cargo.yml`:

```toml
[dependencies]
wena = "0.0.1"
```

After, modify your `src/main.rs` file, and create your CLI application:

```rs
use wena::*;

fn main() {
    Application::new("calculator")
        .commands([Command::new("sum")
            .description("Add two numbers")
            .definition([
                Argument::new("first").required(true),
                Argument::new("second").required(true),
            ])
            .handler(|app| {
                let first = app.input.argument::<i32>("first").unwrap();
                let second = app.input.argument::<i32>("second").unwrap();

                app.output.writeln(format!("Total: {}", first + second));
            })])
        .run();
}
```

Finally, compile and run the with `cargo run`. You should see the following:

```
cargo run -q --
```

## License

Wena is an open-source software licensed under the MIT license.
