pub mod package;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Install a package
    Install,
    /// Remove a package
    Remove,
    /// Clean a package / the entire system
    Clean,
    /// Test a package
    Test,
    /// Update the entire system
    Update,
    /// Manage locks
    Lock,
    /// Manage sources (archives and repos)
    Sources,
    /// Easily switch from Stable to Lab
    Switch,
    /// Toggle on/off the PSB-free option. (Better stability for Lab branches)
    PSB_free,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        _ => { println!("{:?}", cli.command)}
    }
}
