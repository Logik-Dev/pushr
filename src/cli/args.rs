use std::path::PathBuf;

use clap::{Args, Parser, ValueEnum};

#[derive(Parser)]
pub struct Cli {
    /// Title of the message
    #[arg(short, long)]
    pub title: Option<String>,

    /// Content of the message
    #[arg(short, long, value_name = "MESSAGE")]
    pub content: String,

    /// Destination device
    #[arg(short, long)]
    pub device: Option<String>,

    #[command(flatten)]
    pub token: Token,

    #[command(flatten)]
    pub user: User,

    #[arg(short, long, value_enum)]
    pub priority: Option<Priority>,
}

#[derive(ValueEnum, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Priority {
    // High priority notification that is repeated 3 times over 2 minutes
    Emergency,

    // Bypass user's quiet hours
    High,

    // Display an alert according to user's device settings
    Normal,

    // Will not generate sound nor vibrations
    Low,

    // No notification sent (badge may be incremented)
    Lowest,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct Token {
    /// Token used for authentication
    #[arg(short = 'k', long)]
    pub token: Option<String>,

    /// File containing the token used for authentication
    #[arg(short = 'K', long, value_name = "PATH", env = "PUSHOVER_TOKEN")]
    pub token_file: Option<PathBuf>,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct User {
    /// User id
    #[arg(short, long)]
    pub user: Option<String>,

    /// File containing the user id used for authentication
    #[arg(short = 'U', long, env = "PUSHOVER_USER")]
    pub user_file: Option<PathBuf>,
}
