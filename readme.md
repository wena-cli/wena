<p align="center">
    <img title="Wena" width="100%" src="https://raw.githubusercontent.com/wena-cli/wena/main/art/logo.jpg" />
</p>

<p align="center">
  <a href="https://github.com/wena-cli/wena/actions"><img src="https://img.shields.io/github/workflow/status/wena-cli/wena/Tests.svg" alt="Build Status"></img></a>
  <a href="https://packagist.org/packages/wena-cli/wena"><img src="https://img.shields.io/packagist/l/wena-cli/wena.svg" alt="License"></a>
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
use wena;

fn main() {
    wena::app("my-application", "0.0.1", vec![
        wena::command("hello", "Displays hello", |app| {
            app.output.writeln("Hello, world!");
        }),
    ]).run();
}
```

Finally, compile and run the with `cargo run`. You should see the following:

```
cargo run -q --
```

## License

Wena is an open-source software licensed under the MIT license.
