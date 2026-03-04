use clap::Parser;

#[derive(Parser)]
#[clap(version="1.0", author="Amogh", about = "A marco-polo game")]

struct Cli{
    #[clap(subcommand)]
    command:Option<Commands>,
}

#[derive(Parser)]
enum Commands{
    #[clap(version="1.0", author="Amogh", about = "A marco-polo game")]
    Play{
        #[clap(short, long)]
        input: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command{
        Some(Commands::Play{input}) => {
            // if input == "marco" {
            //     println!("Polo!");
            // }
            // else{
            //     println!("Dumbass enter Marco!");
            // }
            let result = marco_polo::marco(&input);
            println!("{}", result);
        }
        None => {
            println!("No command provided");
        }
    }
}   