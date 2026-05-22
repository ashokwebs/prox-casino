use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::core::DAILY_BONUS;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflinePlayer {
    pub name: String,
    pub chips: i64,
    pub last_daily_claim: Option<DateTime<Utc>>,
}

impl Default for OfflinePlayer {
    fn default() -> Self {
        Self {
            name: "Player".to_string(),
            chips: crate::core::START_CHIPS,
            last_daily_claim: None,
        }
    }
}

impl OfflinePlayer {
    pub fn can_claim_daily(&self) -> bool {
        self.last_daily_claim
            .map(|ts| Utc::now().date_naive() > ts.date_naive())
            .unwrap_or(true)
    }

    pub fn claim_daily_bonus(&mut self) -> bool {
        if self.can_claim_daily() {
            self.chips += DAILY_BONUS;
            self.last_daily_claim = Some(Utc::now());
            true
        } else {
            false
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OnlinePlayer {
    pub account_id: String,
    pub display_name: String,
}
