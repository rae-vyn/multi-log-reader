use crate::parse_lines;
use num_bigint::BigInt;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Sender {
    Client,
    Enemy,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Err,
    EmptyOption,
    Connected,
    Username {
        username: String,
        mod_hash: String,
    },
    Version,
    CreateLobby {
        game_mode: String,
    },
    JoinedLobby {
        code: String,
        game_type: String,
    },
    SyncClient {
        is_cached: bool,
    },
    LobbyInfo {
        host_cached: bool,
        is_host: bool,
        host_name: String,
        guest_name: String,
        guest_cached: bool,
    },
    LobbyOptions {
        different_decks: bool,
        death_on_round_loss: bool,
        pvp_start_round: i8,
        stake: i8,
        no_gold_on_round_loss: bool,
        different_seeds: bool,
        showdown_starting_antes: i8,
        back: String,
        gold_on_life_loss: bool,
        ruleset: String,
        sleeve: String,
        starting_lives: i8,
        multiplayer_jokers: bool,
    },
    LeaveLobby,
    JoinLobby {
        code: String,
    },
    StartGame {
        deck: String,
    },
    SetAnte {
        ante: i8,
    },
    PlayerInfo {
        lives: i8,
    },
    PlayHand {
        hands_left: i8,
        score: BigInt,
    },
    NewRound,
    SetLocation {
        location: PlayerLocation,
        user: Sender,
    },
    MoneyMoved {
        amount: i8,
    },
    SpentLastShop {
        amount: i8,
    },
    EndPvP {
        lost: bool,
    },
    SoldCard {
        card: String,
    },
    SoldJoker,
    Skip {
        skips: i8,
    },
    EnemyInfo {
        lives: i8,
        skips: i8,
        hands_left: i8,
        score: BigInt,
    },
    ReadyBlind,
    Speedrun,
    UsedCard {
        card: String,
    },
    StartBlind,
    LoseGame,
    WinGame,
    GetEndGameJokers,
    RecieveEndGameJokers {
        keys: Vec<String>,
    },
    StopGame,
    Asteroid,
    Magnet,
    MagnetResponse {
        key: String,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerLocation {
    SmallBlind,
    BigBlind,
    BossBlind,
    PvP,
    Shop,
    Selecting,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub sender: Sender,
    pub action: String,
    pub other: HashMap<String, String>,
    pub time: String,
}

impl Action {
    pub fn from_message(message: Message) -> Self {
        match message.action.as_str() {
            "createLobby" =>
                Self::CreateLobby {
                    game_mode: message.other.get("gameMode").unwrap().to_string(),
                },
            "moneyMoved" =>
                Self::MoneyMoved {
                    amount: message.other.get("amount").unwrap().parse::<i8>().unwrap(),
                },
            "leaveLobby" => Self::LeaveLobby,
            "spentLastShop" =>
                Self::SpentLastShop {
                    amount: message.other.get("amount").unwrap().parse::<i8>().unwrap(),
                },
            "joinLobby" =>
                Self::JoinLobby {
                    code: message.other.get("code").unwrap().to_string(),
                },
            "endPvP" =>
                Self::EndPvP { lost: message.other.get("lost").unwrap().parse::<bool>().unwrap() },
            "magnetResponse" =>
                Self::MagnetResponse { key: message.other.get("key").unwrap().to_string() },
            "connected" => Self::Connected,
            "stopGame" => Self::StopGame,
            "lobbyOptions" => {
                if message.other.is_empty() {
                    return Self::EmptyOption
                }
                Self::LobbyOptions {
                    different_decks: message.other
                        .get("different_decks")
                        .unwrap()
                        .parse::<bool>()
                        .unwrap(),
                    death_on_round_loss: message.other
                        .get("death_on_round_loss")
                        .unwrap()
                        .parse::<bool>()
                        .unwrap(),
                    pvp_start_round: match message.other
                        .get("pvp_start_round")
                        {
                            Some(num) => num.parse::<i8>().unwrap(),
                            None => 0
                        },
                    stake: message.other.get("stake").unwrap().parse::<i8>().unwrap(),
                    no_gold_on_round_loss: message.other
                        .get("no_gold_on_round_loss")
                        .unwrap()
                        .parse::<bool>()
                        .unwrap(),
                    different_seeds: message.other
                        .get("different_seeds")
                        .unwrap()
                        .parse::<bool>()
                        .unwrap(),
                    showdown_starting_antes: message.other
                        .get("showdown_starting_antes")
                        .unwrap()
                        .parse::<i8>()
                        .unwrap(),
                    back: message.other.get("back").unwrap().to_string(),
                    gold_on_life_loss: message.other
                        .get("gold_on_life_loss")
                        .unwrap()
                        .parse::<bool>()
                        .unwrap(),
                    ruleset: message.other.get("ruleset").unwrap().to_string(),
                    sleeve: match message.other
                    .get("sleeve")
                    {
                        Some(num) => num.to_string(),
                        None => String::new(),
                    },
                    starting_lives: message.other
                        .get("starting_lives")
                        .unwrap()
                        .parse::<i8>()
                        .unwrap(),
                    multiplayer_jokers: message.other
                        .get("multiplayer_jokers")
                        .unwrap()
                        .parse::<bool>()
                        .unwrap(),
                }
            }
            "syncClient" =>
                Self::SyncClient {
                    is_cached: message.other.get("isCached").unwrap().parse::<bool>().unwrap(),
                },
            "username" =>
                Self::Username {
                    username: message.other.get("username").unwrap().to_string(),
                    mod_hash: message.other.get("modHash").unwrap().to_string(),
                },
            "usedCard" => Self::UsedCard { card: message.other.get("card").unwrap().to_string() },
            "soldCard" => Self::SoldCard { card: message.other.get("card").unwrap().to_string() },
            "startGame" => Self::StartGame { deck: message.other.get("deck").unwrap().to_string() },
            "speedrun" => Self::Speedrun,
            "playerInfo" =>
                Self::PlayerInfo {
                    lives: message.other.get("lives").unwrap().parse::<i8>().unwrap(),
                },
            "version" => Self::Version,
            "getEndGameJokers" => Self::GetEndGameJokers,
            "joinedLobby" =>
                Self::JoinedLobby {
                    code: message.other.get("code").unwrap().to_string(),
                    game_type: message.other.get("type").unwrap().to_string(),
                },
            "newRound" => Self::NewRound,
            "enemyLocation" =>
                Self::SetLocation {
                    location: match message.other.get("location").unwrap().as_str() {
                        "loc_playing-bl_small" => PlayerLocation::SmallBlind,
                        "loc_playing-bl_big" => PlayerLocation::BigBlind,
                        "loc_playing-mp_nemesis" => PlayerLocation::PvP,
                        "loc_shop" => PlayerLocation::Shop,
                        "loc_selecting" => PlayerLocation::Selecting,
                        _ => PlayerLocation::BossBlind,
                    },
                    user: Sender::Enemy,
                },
            "lobbyInfo" => {
                if message.other.is_empty() {
                    return Self::EmptyOption
                }
                Self::LobbyInfo {
                    host_cached: message.other.get("hostCached").unwrap().parse::<bool>().unwrap(),
                    guest_cached: match message.other.get("guestCached") {
                        None => String::from("false").parse::<bool>().unwrap(),
                        Some(st) => st.parse::<bool>().unwrap()
                    },
                    is_host: message.other.get("isHost").unwrap().parse::<bool>().unwrap(),
                    host_name: message.other.get("host").unwrap().to_string(),
                    guest_name: match message.other.get("guest") {
                        None => String::new(),
                        Some(st) => st.to_string()
                    },
                }
            }
            "playHand" =>
                Self::PlayHand {
                    hands_left: message.other.get("handsLeft").unwrap().parse::<i8>().unwrap(),
                    score: message.other.get("score").unwrap().parse::<BigInt>().unwrap(),
                },
            "soldJoker" => Self::SoldJoker,
            "enemyInfo" =>
                Self::EnemyInfo {
                    lives: message.other.get("lives").unwrap().parse::<i8>().unwrap(),
                    skips: message.other.get("skips").unwrap().parse::<i8>().unwrap(),
                    hands_left: message.other.get("handsLeft").unwrap().parse::<i8>().unwrap(),
                    score: message.other.get("score").unwrap().parse::<BigInt>().unwrap(),
                },
            "readyBlind" => Self::ReadyBlind,
            "loseGame" => Self::LoseGame,
            "winGame" => Self::WinGame,
            "startBlind" => Self::StartBlind,
            "recieveEndGameJokers" =>
                Self::RecieveEndGameJokers {
                    keys: message.other
                        .get("keys")
                        .unwrap()
                        .split(";")
                        .filter(|x| !x.is_empty())
                        .map(|x| x.to_string())
                        .collect(),
                },
            "skip" =>
                Self::Skip { skips: message.other.get("skips").unwrap().parse::<i8>().unwrap() },
            "asteroid" => Self::Asteroid,
            "magnet" => Self::Magnet,
            "setLocation" =>
                Self::SetLocation {
                    location: match message.other.get("location").unwrap().as_str() {
                        "loc_playing-bl_small" => PlayerLocation::SmallBlind,
                        "loc_playing-bl_big" => PlayerLocation::BigBlind,
                        "loc_playing-mp_nemesis" => PlayerLocation::PvP,
                        "loc_shop" => PlayerLocation::Shop,
                        "loc_selecting" => PlayerLocation::Selecting,
                        _ => PlayerLocation::BossBlind,
                    },
                    user: Sender::Client,
                },
            "setAnte" =>
                Self::SetAnte { ante: message.other.get("ante").unwrap().parse::<i8>().unwrap() },
            _ => Self::Err,
        }
    }
}

impl Message {
    pub fn messages_from_string(contents: String) -> Vec<Message> {
        // Regex blocks
        let enemy_re = Regex::new(
            r"(?<timestamp>\d+:\d+:\d+).*Client got (?<message>\w+) message:\s+(?<contents>.*)"
        ).unwrap();
        let client_re = Regex::new(
            r"(?<timestamp>\d+:\d+:\d+).*Client sent message:\s+(?<contents>.*)"
        ).unwrap();
        let message_content_re = Regex::new(r"\((?<key>\w+): (?<value>[-\w\s;]+)\)").unwrap();
        let client_content_re = Regex::new(r"(?<key>\w+):(?<value>[-\w\s;]+)").unwrap();
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
                        end_message.action = part["value"].trim().to_string();
                    } else {
                        let _ = end_message.other.insert(
                            part["key"].to_string(),
                            part["value"].trim().to_string()
                        );
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
                            end_message.action = part["value"].trim().to_string();
                        } else {
                            let _ = end_message.other.insert(
                                part["key"].to_string(),
                                part["value"].trim().to_string()
                            );
                        }
                    }
                }
            }
            if !end_message.action.is_empty() {
                messages.push(end_message);
            }
        }
        return messages;
    }
}
