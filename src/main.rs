use std::fs;
use clap::{Parser, Subcommand};
use lazy_static::lazy_static;

// mod func;
// mod handlers;
mod subcommands;
// mod utils;

use crate::subcommands::prod_firewall;

lazy_static! {
    static ref DATA_PATH: String = {
        dirs::home_dir()
            .unwrap()
            .join(".local/share/fysm")
            .to_string_lossy()
            .into_owned()
    };
    static ref DB_PATH: String = {
        DATA_PATH.clone() + "/fysm.db"
    };
}

#[derive(Parser)]
#[command(name = "fysm")]
#[command(version = "0.1.0")]
#[command(about = "Four Years System Manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
pub enum Commands {
    // /// Manage background
    //Bg {
        //#[command(subcommand)]
        //command: background,
        //name: Option<String>,
    //},

    /// Add a blocked domain
    FwBDAdd {
        /// Domain of site to add a rule
        domain: String,
    },
    /// Remove a blocked domain
    FwBDRm {
        /// Domain of site to remove a rule
        domain: String,
    }
}

fn main() {
    if !fs::metadata(DATA_PATH.as_str()).is_ok() {
        fs::create_dir_all(DATA_PATH.as_str()).unwrap();
    }

    let cli = Cli::parse();

    match &cli.command {
        //Commands::Bg { action, name } => {
        //    background::init_daemon(action, name);
        //}
        Commands::FwBDAdd { domain } => {
            prod_firewall::set_rule("add", domain);
        }
        Commands::FwBDRm { domain } => {
            prod_firewall::set_rule("rm", domain);
        }
    }
}

