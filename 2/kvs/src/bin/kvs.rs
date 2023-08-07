use clap::{Parser, Subcommand};

#[derive(Parser)] // requires `derive` feature
#[command(name = "kvs")]
#[command(about = "A simple key-value store", bin_name = "kvs", author, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Get {
        #[arg(value_name = "key")]
        key: String,
    },
    #[command(arg_required_else_help = true)]
    Set {
        #[arg(value_name = "key")]
        key: String,
        #[arg(value_name = "value")]
        value: String,
    },
    #[command(arg_required_else_help = true)]
    Rm {
        #[arg(value_name = "key")]
        key: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Get { .. } => {
            eprintln!("unimplemented");
            panic!()
        }
        Commands::Set { .. } => {
            eprintln!("unimplemented");
            panic!()
        }
        Commands::Rm { .. } => {
            eprintln!("unimplemented");
            panic!()
        }
    }
}
