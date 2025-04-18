use crate::message::*;

pub fn interpret_message(message: Message) -> String {
    
    match Action::from_message(message.clone()) {
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
        Action::Username { username, mod_hash: _ } => { format!("Set username to {}", username) }
        Action::JoinLobby { code } => { format!("Entered lobby code {}", code) }
        Action::Asteroid => { "Sent an asteroid!".to_string() }
        Action::StartGame { deck: _ } => { "Game started".to_string() }
        Action::Err => "Unsupported Action".to_string(),
        Action::EmptyOption => "Empty Options".to_string(),
        Action::Connected => "Connected to servers".to_string(),
        Action::Version => "Checked version".to_string(),
        Action::LeaveLobby => "Left lobby".to_string(),
        Action::SetAnte { ante } => format!("Ante is now {}", i8::max(ante, 1)),
        Action::PlayerInfo { lives } => format!("Client has {} lives", lives),
        Action::PlayHand { hands_left, score } =>
            format!("Hand played; {} hands left, {} current score", hands_left, score),
        Action::NewRound => "New Round Started".to_string(),
        Action::SetLocation { location, user } =>
            format!("{:?} is currently at {:?}", user, location),
        Action::MoneyMoved { amount } => format!("Money moved: ${}", amount),
        Action::SpentLastShop { amount } => format!("Enemy spent ${} last shop", amount),
        Action::EndPvP { lost } => format!("{} match", if lost { "lost" } else { "won" }),
        Action::SoldCard { card } => format!("Sold {}", card),
        Action::SoldJoker => "Sold a joker".to_string(),
        Action::Skip { skips } => format!("Skipped, curr skips is {}", skips),
        Action::EnemyInfo { lives, skips, hands_left, score } =>
            format!(
                "Enemy has {} lives, has used {} skips, has {} hands left, and has scored {}",
                lives,
                skips,
                hands_left,
                score
            ),
        Action::ReadyBlind => "Readied up for PvP".to_string(),
        Action::Speedrun => "Speedrun active!".to_string(),
        Action::UsedCard { card } => format!("Card used: {}", card),
        Action::StartBlind => "PvP Blind started".to_string(),
        Action::LoseGame => "You lost!".to_string(),
        Action::WinGame => "You won!".to_string(),
        Action::GetEndGameJokers => "Getting end-game jokers...".to_string(),
        Action::ReceiveEndGameJokers { keys , seed} => format!("Sent Jokers: {}{}", keys.join(" "), if seed.is_some() { format!(", Seed is {}", seed.unwrap())} else {"".to_string()}),
        Action::StopGame => "Game Stopped".to_string(),
        Action::Magnet => "Sold magnet".to_string(),
        Action::MagnetResponse { key } => format!("Sent card for magnet: {}", key),
    }
}
