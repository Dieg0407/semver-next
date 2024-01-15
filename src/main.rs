use clap::{Parser, Subcommand};
use semver::generate_version;
use semver::validate_commit_message;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Next {
        #[arg(short, long)]
        commit_message: String,
        #[arg(short, long)]
        last_version: String,
    },
    Validate {
        #[arg(short, long)]
        commit_message: String,
    },
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Next {
            commit_message,
            last_version,
        }) => {
            let new_version = generate_version(commit_message, last_version)?;
            println!("{new_version}");
        }
        Some(Commands::Validate { commit_message }) => {
            let result = validate_commit_message(&commit_message);
            if result.is_err() {
                eprintln!("{}", result.err().unwrap());
                println!("invalid");
            } else {
                println!("valid");
            }
        }
        None => (),
    }

    Ok(())
}
