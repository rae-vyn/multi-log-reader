use multi_log_reader::args;
use clap::Parser;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Read};
use multi_log_reader::message::{Action, Message, Sender};
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
        println!("{}) {:#?}", message.clone().time, message);
        println!("{}) {:#?}", message.clone().time, Action::from_message(message));
    }

    return Ok(());
}



fn display_logs(messages: Vec<Message>) {
    let mut unique_keys: HashSet<String> = HashSet::new();
    for message in messages {
        unique_keys.insert(message.action);
    }
    println!("{:#?}", unique_keys);
}

