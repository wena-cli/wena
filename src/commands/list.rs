use crate::commands::*;

pub fn new<TInput: Input, TOutput: Output>() -> Command<TInput, TOutput> {
    crate::commands::new::<TInput, TOutput>("list")
        .description("Displays the application commands")
        .handler(|app| {
            let name = app.name.clone().bold().white();
            let version = app.version.clone().green().bold();

            app.output.writeln("");
            app.output.writeln(&format!("  {} : {}", name, version));

            let executable = std::env::current_exe().unwrap();
            let binary = executable.file_name().unwrap().to_str().unwrap();
            let usage = "USAGE:".bold().yellow();

            app.output.writeln("");
            app.output.writeln(&format!(
                "  {} {} <command> [options] [flags]",
                usage, binary
            ));

            for subcommand in &app.commands {
                let name = subcommand.name.clone().bold().white();
                // let description = subcommand.description.clone().white();

                app.output.writeln(&format!("         wena {}", name));
            }

            app.output.writeln("");
        })
}
