pub mod args;
pub mod message;
pub mod message_interpret;

pub fn parse_lines(text: &String) -> Vec<String> {
    return text.split('\n').map(|x| {x.to_string()}).collect()
}