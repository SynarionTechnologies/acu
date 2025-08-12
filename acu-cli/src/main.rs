//! Command line interface for ACU.
//!
//! ```bash
//! cargo run -p acu-cli -- --help
//! ```

use clap::{Parser, Subcommand};

/// ACU command line application.
#[derive(Parser)]
#[command(name = "acu", version, about = "Autonomous Conscient Unit CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// Top-level commands.
#[derive(Subcommand)]
enum Command {
    /// Manage profiles.
    Profile {
        #[command(subcommand)]
        action: Profile,
    },
    /// Inspect modules.
    Modules {
        #[command(subcommand)]
        action: Modules,
    },
    /// Inspect memory usage.
    Memory {
        #[command(subcommand)]
        action: Memory,
    },
}

/// Profile commands.
#[derive(Subcommand)]
enum Profile {
    /// Create a profile.
    Create,
    /// Show profile information.
    Show,
}

/// Module commands.
#[derive(Subcommand)]
enum Modules {
    /// List loaded modules.
    List,
}

/// Memory commands.
#[derive(Subcommand)]
enum Memory {
    /// Display memory statistics.
    Stats,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Profile {
            action: Profile::Create,
        } => println!("profile created (stub)"),
        Command::Profile {
            action: Profile::Show,
        } => println!("profile: default (stub)"),
        Command::Modules {
            action: Modules::List,
        } => println!("modules: none (stub)"),
        Command::Memory {
            action: Memory::Stats,
        } => println!("memory stats: n/a (stub)"),
    }
}
