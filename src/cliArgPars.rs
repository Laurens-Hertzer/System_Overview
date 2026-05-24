use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sysover")]
pub struct Cli {
    #[arg(short, long, default_value_t = 2)]
    pub interval: u64,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Disk,
    Procs {
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },
}