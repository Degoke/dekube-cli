#![allow(unused)]
use clap::{Parser, Subcommand};
use anyhow::{Context, Result};
use log::{info, warn};
use rusqlite::{Connection};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Logs in user
    Authenticate {
        #[arg(short, long)]
        email: String,

        #[arg(short, long)]
        password: String
    },

    Upload {
        #[arg(short, long)]
        path: std::path::PathBuf,
    }
}
 
 #[tokio::main]
pub async fn main() -> Result<()> {
    env_logger::init();
    info!("Starting up");
    let conn = Connection::open("dekube.db")?;

    match conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
            id INTEGER PRIMARY KEY,
            email TEXT NOT NULL UNIQUE,
            token TEXT
        )",
        (),
    ) {
        Ok(created) => info!("table {} created", created),
        Err(error) => warn!("Error initializing tables {}", error)
    }

    let cli = Cli::parse();

    match &cli.command {
        Commands::Authenticate { email, password } => {
            dekube::authenticate_user(email, password, &conn);
        },
        Commands::Upload { path } => {
            // let mut folder = path.clone();
            // println!("{:?} is dir {}", folder, folder.is_dir());
            // folder.push("Dockerfile");
            // println!("{:?}", folder);
            // println!("dockerfile exists {}", folder.exists());
            dekube::upload::handle_upload(path).await
        }
    }

    Ok(())
}