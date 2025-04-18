use colored::Colorize;
use multi_log_reader::args;
use clap::Parser;
use std::fs::File;
use std::io::{self, Read};
use multi_log_reader::message::{Action, Message, Sender};
use multi_log_reader::message_interpret::interpret_message;
fn main() -> io::Result<()> {
    let used_args = args::Args::parse();
    let mut file = match File::open(&used_args.file_path) {
        Ok(res) => res,
        Err(error) => panic!("{}", error)
    };

    let mut contents = String::new();

    if let Err(error) = file.read_to_string(&mut contents) {
        panic!("{}", error)
    }

    let messages = Message::messages_from_string(contents);

    for message in messages {
        //println!("{}) {:#?}", message.clone().time, message);
        //println!("{}) {:#?}", message.clone().time, Action::from_message(message.clone()));
        let out = format!("{}", interpret_message(message.clone()));
        if message.clone().sender == Sender::Client {
            println!("{}) {}", message.time, out.green())
        } else {
            println!("{}) {}", message.time, out.red())
        }
    }

    return Ok(());
}

