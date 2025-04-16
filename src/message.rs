use crate::parse_lines;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Sender {
    Client,
    Enemy,
}

#[derive(Debug)]
pub struct Message {
    pub sender: Sender,
    pub action: String,
    pub other: HashMap<String, String>,
    pub time: String,
}

impl Message {
    pub fn messages_from_string(contents: String) -> Vec<Message> {
        // Regex blocks
        let enemy_re = Regex::new(
            r"(?<timestamp>\d+:\d+:\d+).*Client got (?<message>\w+) message:\s+(?<contents>.*)",
        )
        .unwrap();
        let client_re =
            Regex::new(r"(?<timestamp>\d+:\d+:\d+).*Client sent message:\s+(?<contents>.*)")
                .unwrap();
        let message_content_re = Regex::new(r"\((?<key>\w+): (?<value>[\w\s;]+)\)").unwrap();
        let client_content_re = Regex::new(r"(?<key>\w+):(?<value>[\w\s;]+)").unwrap();
        let mut messages: Vec<Message> = vec![];
        for line in parse_lines(&contents) {
            let mut end_message = Message {
                sender: Sender::Client,
                action: "".to_string(),
                other: HashMap::new(),
                time: "".to_string(),
            };
            if let Some(caps) = enemy_re.captures(&line) {
                let (_full, [timestamp, _message, contents]) = caps.extract();
                end_message.time = timestamp.to_string();
                end_message.sender = Sender::Enemy;
                let content_caps = message_content_re.captures_iter(contents);
                for part in content_caps {
                    if &part["key"] == "action" {
                        end_message.action = part["value"].trim().to_string()
                    } else {
                        let _ = end_message
                            .other
                            .insert(part["key"].to_string(), part["value"].trim().to_string());
                    }
                }
            }
            if let Some(caps) = client_re.captures(&line) {
                let (_full, [timestamp, contents]) = caps.extract();
                end_message.time = timestamp.to_string();
                let used_contents: Vec<&str> = contents.split(",").collect();
                for segment in used_contents {
                    let content_caps = client_content_re.captures_iter(segment);
                    for part in content_caps {
                        if &part["key"] == "action" {
                            end_message.action = part["value"].trim().to_string()
                        } else {
                            let _ = end_message
                                .other
                                .insert(part["key"].to_string(), part["value"].trim().to_string());
                        }
                    }
                }
            }
            if !end_message.action.is_empty() {
                messages.push(end_message)
            };
        }
        return messages;
    }
}
