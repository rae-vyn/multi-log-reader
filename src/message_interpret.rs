use crate::message::*;

pub fn interpret_message(message: Message) -> String {
    let mut output = String::new();
    output = match Action::from_message(message.clone()) {
        Action::JoinedLobby { code, game_type } => {
            format!("Joined {} game with code {}", game_type, code)
        }
        Action::CreateLobby { game_mode } => { format!("Created {} game", game_mode) }
        Action::SyncClient { is_cached } => {
            format!("Client synced, {} cached.", if is_cached { "is" } else { "isn't" })
        }
        Action::LobbyInfo { .. } => { format!("Lobby info: {:#?}", Action::from_message(message)) }
        Action::LobbyOptions { .. } => {
            format!("Lobby config: {:#?}", Action::from_message(message))
        }
        Action::Username { username, mod_hash } => { format!("Set username to {}", username) }
        Action::JoinLobby { code } => { format!("Entered lobby code {}", code) }
        Action::Asteroid => { format!("Sent an asteroid!") }
        Action::StartGame { deck } => { format!("Game started") }
        Action::Err => format!("Unsupported Action"),
        Action::EmptyOption => format!("Empty Options"),
        Action::Connected => format!("Connected to servers"),
        Action::Version => format!("Checked version"),
        Action::LeaveLobby => format!("Left lobby"),
        Action::SetAnte { ante } => format!("Ante is now {}", i8::max(ante, 1)),
        Action::PlayerInfo { lives } => format!("Client has {} lives", lives),
        Action::PlayHand { hands_left, score } =>
            format!("Hand played; {} hands left, {} current score", hands_left, score),
        Action::NewRound => format!("New Round Started"),
        Action::SetLocation { location, user } =>
            format!("{:?} is currently at {:?}", user, location),
        Action::MoneyMoved { amount } => format!("Money moved: ${}", amount),
        Action::SpentLastShop { amount } => format!("Enemy spent ${} last shop", amount),
        Action::EndPvP { lost } => format!("{} match", if lost { "lost" } else { "won" }),
        Action::SoldCard { card } => format!("Sold {}", card),
        Action::SoldJoker => format!("Sold a joker"),
        Action::Skip { skips } => format!("Skipped, curr skips is {}", skips),
        Action::EnemyInfo { lives, skips, hands_left, score } =>
            format!(
                "Enemy has {} lives, has used {} skips, has {} hands left, and has scored {}",
                lives,
                skips,
                hands_left,
                score
            ),
        Action::ReadyBlind => format!("Readied up for PvP"),
        Action::Speedrun => format!("Speedrun active!"),
        Action::UsedCard { card } => format!("Card used: {}", card),
        Action::StartBlind => format!("PvP Blind started"),
        Action::LoseGame => format!("You lost!"),
        Action::WinGame => format!("You won!"),
        Action::GetEndGameJokers => format!("Getting end-game jokers..."),
        Action::ReceiveEndGameJokers { keys , seed} => format!("Sent Jokers: {}{}", keys.join(" "), if seed.is_some() { format!(", Seed is {}", seed.unwrap())} else {"".to_string()}),
        Action::StopGame => format!("Game Stopped"),
        Action::Magnet => format!("Sold magnet"),
        Action::MagnetResponse { key } => format!("Sent card for magnet: {}", key),
    };
    return output;
}
