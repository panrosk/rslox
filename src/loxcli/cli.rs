use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Start an interactive session
    #[command(name = "repl")]
    Interactive,
}

pub fn run_command() -> Result<()> {
    let cli = Cli::parse();

    if let Some(loxfile) = &cli.file {
        println!("{:?}", loxfile);
        return Ok(()); // Termina la función aquí si existe el archivo
    };

    if let Some(command) = &cli.command {
        match command {
            Commands::Interactive => interactive_session(),
        }
    } else {
        // Si no se proporcionó ningún subcomando, mostrar un mensaje o hacer otra cosa
        println!("No subcommand provided.");
    }

    Ok(())
}

pub fn interactive_session() {
    println!("Entering interactive mode. Type '#quit' to exit.");
    loop {
        // Display a prompt
        print!("> ");
        io::stdout().flush().unwrap();

        // Read user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim(); // Remove any trailing newline or spaces

        // Check if the user typed "#quit"
        if input == "#quit" {
            println!("Exiting interactive mode.");
            break;
        }

        // Echo back the input
        println!("{}", input);
    }
}
