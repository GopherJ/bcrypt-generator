extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash, verify};
use clap::{value_t_or_exit};

mod init;

fn main() -> Result<(), Box<std::error::Error>>{
    match init::OPTS.subcommand() {
        ("generate", Some(matches)) => {
            let input = value_t_or_exit!(matches.value_of("input"), String); 
            let cost = value_t_or_exit!(matches.value_of("cost"), u32);
            
            println!("{}", hash(input, cost)?);
        },
        ("verify", Some(matches)) => {
            let input = value_t_or_exit!(matches.value_of("input"), String); 
            let hashed = value_t_or_exit!(matches.value_of("hashed"), String);
            
            if verify(input, &hashed)? {
                println!("{}", 1);
            } else {
                println!("{}", 0);
            }
        },
        _ => unreachable!()
    }

    Ok(())
}
