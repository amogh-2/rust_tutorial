/*
To encrypt:
cargo run -- --message "This is a secret message. You cannot read it." --encrypt --shift 10

To decrypt
cargo run -- --message "Drsc sc k combod wocckqo. Iye mkxxyb bokn sd." --decrypt --shift 10
*/


use ceaser_cipher::{decrypt,encrypt};
use clap::Parser;


#[derive(Parser, Debug)]
#[command(author,version,about,long_about= None)]

struct Args{
    //encrypt the message
    #[arg(short,long)]
    encrypt:bool,
    //decrypt the message
    #[arg(short,long)]
    decrypt:bool,
    //message to encrypt or decrypt
    #[arg(short,long)]
    message: String,
    //The shift to use for cypher
    #[arg(short,long,default_value="3")]
    shift:u8,
}

fn main(){
    let args = Args::parse();
    if args.encrypt{
        println!("{}",encrypt(&args.message,args.shift));
    }
    else if args.decrypt{
        println!("{}",decrypt(&args.message,args.shift));
    }
    else{
        println!("Please specify either --encrypt or --decrypt");
    }
}

