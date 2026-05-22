#[allow(dead_code)]
pub struct SecurityArchitecture {
    pub note: &'static str,
}

impl Default for SecurityArchitecture {
    fn default() -> Self {
        Self {
            note: "Online mode: all game logic, RNG, and balances must be server-authoritative. \
                   The client is never trusted. All actions must be validated server-side. \
                   Replay logs and session tokens enforce integrity."
        }
    }
}

#[allow(dead_code)]
pub enum SecurityLevel {
    Offline,
    Online,
}

impl SecurityLevel {
    pub fn is_authoritative(&self) -> bool {
        matches!(self, SecurityLevel::Online)
    }
}
