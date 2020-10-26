#[macro_use]
extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("input miner address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_addr);

    print!("difficulty: ");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("difficulty must be an integer.");

    println!("generating genesis block...");
    let mut blockchain = blockchain::Blockchain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        print!("Enter your choice: ");
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
        println!("");

        match choice.trim().parse().unwrap() {
            0 => {
                print!("exiting program...");
                process::exit(0);
            },
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();
                let trans_res;

                print!("sender address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);
                print!("receiver address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut receiver);
                print!("amount: ");
                io::stdout().flush();
                io::stdin().read_line(&mut amount);
                
                trans_res = blockchain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap()
                );
            },
            _ => println!("invalid option..."),
        }
    }
}
