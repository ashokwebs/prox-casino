use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerStats {
    pub games_played: u64,
    pub total_won: i64,
    pub total_bet: i64,

    #[serde(default)]
    pub biggest_win: i64,
    #[serde(default)]
    pub highest_bet: i64,

    // Blackjack
    pub blackjack_wins: u64,
    pub blackjack_losses: u64,
    pub blackjack_pushes: u64,

    #[serde(default)]
    pub blackjack_count: u64,
    #[serde(default)]
    pub bust_count: u64,
    #[serde(default)]
    pub dealer_bust_count: u64,

    pub blackjack_streak: i64,
    pub blackjack_best_streak: i64,

    #[serde(default)]
    pub average_bet: i64,

    // Detailed Blackjack Stats
    #[serde(default)]
    pub blackjack_win_streak: u64,
    #[serde(default)]
    pub blackjack_loss_streak: u64,
    #[serde(default)]
    pub blackjack_push_streak: u64,
    #[serde(default)]
    pub perfect_blackjacks: u64, // Natural 21 on initial deal
    #[serde(default)]
    pub double_down_wins: u64,
    #[serde(default)]
    pub double_down_losses: u64,
    #[serde(default)]
    pub split_wins: u64,
    #[serde(default)]
    pub split_losses: u64,
    #[serde(default)]
    pub surrender_count: u64,
    #[serde(default)]
    pub insurance_wins: u64,
    #[serde(default)]
    pub insurance_losses: u64,

    // Slots
    pub slots_spins: u64,
    pub slots_biggest_win: i64,

    #[serde(default)]
    pub jackpot_count_mini: u64,
    #[serde(default)]
    pub jackpot_count_mega: u64,
    #[serde(default)]
    pub jackpot_count_ultra: u64,

    // Detailed Slots Stats
    #[serde(default)]
    pub slots_win_streak: u64,
    #[serde(default)]
    pub slots_loss_streak: u64,
    #[serde(default)]
    pub cherry_matches: u64,
    #[serde(default)]
    pub lemon_matches: u64,
    #[serde(default)]
    pub bell_matches: u64,
    #[serde(default)]
    pub seven_matches: u64,
    #[serde(default)]
    pub diamond_matches: u64,
    #[serde(default)]
    pub wild_matches: u64,
    #[serde(default)]
    pub scatter_matches: u64,

    // Session History (last 10 sessions)
    #[serde(default)]
    pub session_history: Vec<SessionSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionSummary {
    pub date: String,
    pub duration_seconds: u64,
    pub starting_chips: i64,
    pub ending_chips: i64,
    pub net_change: i64,
    pub blackjack_wins: u64,
    pub blackjack_losses: u64,
    pub slots_spins: u64,
    pub biggest_win: i64,
    pub biggest_loss: i64,
}
