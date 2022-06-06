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

> **Requires [Rust 1.61.0](https://blog.rust-lang.org/2022/05/19/Rust-1.61.0.html)**

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
wena = "0.1.0"
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

                Ok(0)
            })])
        .run();
}
```

Finally, compile and run the with `cargo run`.

```
cargo run -q --
```

## Application

As you may have noticed, Wena applications may be created using the struct `Application` that gets exported at the root level of the crate `wena`. And, the `new` static method allows you to create an instance of the struct `Application`:

```rust
use wena::Application;

let app = Application::new("your-application-name");
```

The struct `Application` represents a command-line application, and as such, it contains a name, description, version, list of commands, an input implementation, and an output implementation.

You may run the application at any time using the method `run()`:

```rust
use wena::Application;

let app = Application::new("your-application-name")
    .run();
```

### Description

Having a description is not required. You can optionally define a description using the `description()` method:

```rust
use wena::Application

let app = Application::new("application-name")
    .description("Application description");
```

### Version

By default, the application version is `1.0.0`. You can optionally define a version using the `version()` method:

```rust
use wena::Application

let app = Application::new("application-name")
    .version("0.1.0");
```

### Commands

You may run your application without any arguments to view all available commands in the application. Commands are defined using the `commands` method:

```rust
use wena::{Application, Command, Output};

let app = Application::new("application-name")
    .commands([
        Command::new("command-name").handler(|app| {
            // Command code...

            Ok(0)
        })
    ]);
```

#### Command description

Having a description is not required. You can optionally define a description using the `description()` method:

```rust
use wena::Command

let command = Command::new("command-name")
    .description("Command description");
```

#### Command handle

The command's `handle()` method receives a closure that contains the logic you want the command to execute.

```rust
use wena::Command

let command = Command::new("command-name")
    .handler(|app| {
        // Command code...

        Ok(0)
    });
```

The closure's first argument is an `Application` instance. As such, you have access to the application's `Input` and `Output` at any time in your command's code:

```rust
use wena::Command

let command = Command::new("command-name")
    .handler(|app| {
        let input = &app.input;
        let output = &app.output;

        Ok(0)
    });
```

#### Command input

The command's input may be defined using the command's `definition` method:

```rust
use wena::{Argument, Command};

let command = Command::new("command-name")
    .definition([
        Argument::new("argument-name").required(true),
    ]);
```

When defined, input arguments may be accessed using the method `argument` in your command's code:

```rust
use wena::{Argument, Command, Input};

let command = Command::new("command-name")
    .definition([
        Argument::new("argument-name").required(true),
    ]).handler(|app| {
        let value = app.input.argument::<String>("argument-name");

        Ok(0)
    });
```

The trait `Input` is required when using methods of the struct `Input`.

#### Command output

When necessary, you may write messages to the console using the command's output `write` or `writeln` methods:

```rust
use wena::{Command, Output};

let command = Command::new("command-name")
    .handler(|app| {
        // Outputs the given message...
        app.output.write("My message");

        // Outputs the a new line...
        app.output.new_line();

        // Outputs the given message and a new line...
        app.output.writeln("My message");

        Ok(0)
    });
```

The trait `Output` is required when using methods of the struct `Output`.

## Components

Wena gives you access the beautifully designed output components that give you everything you need to build CLI applications.

### Alert

Alerts provide contextual feedback messages for typical user actions.

```rust
use wena::{Alert, Command, Output};

let command = Command::new("command-name")
    .handler(|app| {
        app.output.writeln(Alert::error("This is a error — check it out!"));
        app.output.writeln(Alert::info("This is a info — check it out!"));
        app.output.writeln(Alert::warn("This is a warning — check it out!"));

        Ok(0)
    );
```

---

## License

Wena is an open-source software licensed under the MIT license.
