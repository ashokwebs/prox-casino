#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ClientAction {
    Login { token: String },
    JoinLobby,
    QueueMatchmaking,
    PlaceBet { table_id: String, amount: u64 },
    GameMove { match_id: String, payload: String },
    Ping,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ServerEvent {
    Authenticated,
    LobbySnapshot { active_players: u32 },
    MatchFound { match_id: String },
    GameStateDelta { match_id: String, state: String },
    ValidationError { reason: String },
    SessionExpired,
}
