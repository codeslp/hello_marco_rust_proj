// a command line tool to play Marco-Polo

use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Brian Farish",
    about = "A simple Marco-Polo game"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(
        version = "1.0",
        author = "Brian Farish",
        about = "A simple Marco-Polo game"
    )]
    Play {
        #[clap(short, long)]
        //this String here maps to our lib.rs marco_polo fn
        name: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { name }) => {
            let result = hello_marco::marco_polo(&name);
            println!("{}", result);
        }
        None => println!("No command given"),
    }
}
