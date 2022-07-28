use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::{io, thread};
use std::thread::JoinHandle;
use solana_sdk::system_instruction;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signature::Signer;
use owo_colors::OwoColorize;

const LOGO: &str = r#"
 _   _             _ _            ___      _     _
| | | |           (_| |          / _ \    | |   | |
| | | | __ _ _ __  _| |_ _   _  / /_\ \ __| | __| |_ __ ___ ___ ___
| | | |/ _` | '_ \| | __| | | | |  _  |/ _` |/ _` | '__/ _ / __/ __|
\ \_/ | (_| | | | | | |_| |_| | | | | | (_| | (_| | | |  __\__ \__ \
 \___/ \__,_|_| |_|_|\__|\__, | \_| |_/\__,_|\__,_|_|  \___|___|___/
                          __/ |
                         |___/
                                                                    "#;

fn main() {
    println!("{}", LOGO.bright_blue());

    println!("{}", ("Please enter the desired beginning for your wallet (press Enter to leave blank).").bright_blue());
    let mut beginning = String::new();
    io::stdin()
        .read_line(&mut beginning)
        .expect("Failed to read");
    if beginning.ends_with('\n') {
        beginning.pop();
    }

    println!("{}", ("Please enter the desired ending for your wallet (press Enter to leave blank).").bright_blue());
    let mut ending = String::new();
    io::stdin()
        .read_line(&mut ending)
        .expect("Failed to read");
    if ending.ends_with('\n') {
        ending.pop();
    }

    // println!("{}", "Does register matter? (yes/no)".bright_blue());
    // let mut register = String::new();
    // io::stdin()
    //     .read_line(&mut register)
    //     .expect("Failed to read");
    // if register.ends_with('\n') {
    //     register.pop();
    // }

    println!("{}", "Generating your wallet".bright_blue());
    loop {
        let keypair = Keypair::new();

        if &keypair.pubkey().to_string().to_lowercase()[0..beginning.len()] == beginning.to_lowercase() && &keypair.pubkey().to_string().to_lowercase()[keypair.pubkey().to_string().len() - ending.len()..keypair.pubkey().to_string().len()] == ending.to_lowercase() {
            println!("{}", format!("Generated wallet! {:?} [{:?}]", keypair.pubkey().to_string(), keypair.to_base58_string()).green());
            break;
            // if &keypair.pubkey().to_string()[0..beginning.len()] == beginning && &keypair.pubkey().to_string()[keypair.pubkey().to_string().len() - ending.len()..keypair.pubkey().to_string().len()] == ending {
            //     println!("{}", format!("Generated wallet! {:?} [{:?}]", keypair.pubkey().to_string(), keypair.to_base58_string()).green());
            //     break;
        }
    }
}