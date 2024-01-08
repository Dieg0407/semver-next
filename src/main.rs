use clap::Parser;
use semver_next::generate_version;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[arg(short, long)]
    commit_message: String,

    #[arg(short, long)]
    last_version: String,
}

fn main() {
    let Arguments {
        commit_message,
        last_version,
    } = Arguments::parse();
    let new_version = generate_version(commit_message, last_version);
    println!("{new_version}");
}
