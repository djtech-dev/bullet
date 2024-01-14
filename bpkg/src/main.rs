pub mod package;

use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Args, Debug)]
struct PSBFree {
    #[clap(long, short, action)]
    /// Rebuild all packages that use the updated packages as dependencies
    mode: bool,
}
#[derive(Debug, Subcommand)]
enum SourcesCmd {
    /// List all sources
    List,
    /// Add source(s)
    Add,
    /// Remove source(s)
    Rm,
    /// Edit the source list using a text editor
    Edit,
    /// Add the official sources for your selected Channel
    Official,
}
#[derive(Debug, Subcommand)]
enum LockCmd {
    /// List all locks
    List,
    /// Add a lock
    Add,
    /// Remove a lock
    Rm,
    /// Enable a disabled lock
    Enable,
    /// Disable an enabled lock
    Disable,
}
#[derive(Debug, Subcommand)]
enum SwitchCmd {
    /// Replace all Lab channels with Stable channels.
    Stable,
    /// Replace all Stable channels with Lab channels.
    Lab,
}
#[derive(Subcommand, Debug)]
enum Commands {
    /// Build a package
    Build,
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
    Lock {
        #[clap(subcommand)]
        l: LockCmd
    },
    /// Manage sources (archives and repos)
    Sources {
        #[clap(subcommand)]
        s: SourcesCmd
    },
    /// Easily switch from Stable to Lab
    Switch {
        #[clap(subcommand)]
        s: SwitchCmd
    },
    /// Toggle on/off the PSB-free option. (Better stability for Lab branches)
    PSB_free(PSBFree),
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        _ => { println!("{:?}", cli.command)}
    }
}
