use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Args, Debug)]
struct Repo {
    #[clap(long, short, action)]
    /// Rebuild all packages that use the updated packages as dependencies
    rebuild_lower_packages: bool,
    #[clap(long, short, action)]
    /// Test all updated packages
    test: bool,
    #[clap(long, short, action)]
    /// Check if any dependency collision can happen
    check_dependency_collision: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Build rootfs
    Rootfs,
    /// Build ISO image
    ISO,
    /// Build OCI image
    OCI,

    /// Build multiple packages
    Series,
    /// Repo managment command; builds updated packages and runs extra utility for those packages
    Repo(Repo),
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        _ => { println!("{:?}", cli.command)}
    }
}
