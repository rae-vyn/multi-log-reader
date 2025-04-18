use colored::Colorize;
use multi_log_reader::args;
use clap::Parser;
use std::fs::File;
use std::io::{self, Read};
use multi_log_reader::message::{Message, Sender};
use multi_log_reader::message_interpret::interpret_message;
use std::process::exit;
fn main() -> io::Result<()> {
    let used_args = args::Args::parse();
    let mut file = match File::open(&used_args.file_path) {
        Ok(res) => res,
        Err(error) => {
            eprintln!("{}", error);
            exit(2)
        }
    };

    let mut contents = String::new();

    if let Err(error) = file.read_to_string(&mut contents) {
        eprintln!("{}", error);
        exit(2)
    }

    let messages = Message::messages_from_string(contents);

    for message in messages {
        //println!("{}) {:#?}", message.clone().time, message);
        //println!("{}) {:#?}", message.clone().time, Action::from_message(message.clone()));
        let out = interpret_message(message.clone()).to_string();
        if message.clone().sender == Sender::Client {
            println!("{}) {}", message.time, out.green())
        } else {
            println!("{}) {}", message.time, out.red())
        }
    }

    Ok(())
}

