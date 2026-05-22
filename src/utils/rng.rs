#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RngPolicy {
    pub offline_note: &'static str,
    pub online_note: &'static str,
}

impl Default for RngPolicy {
    fn default() -> Self {
        Self {
            offline_note: "Offline RNG is local and user-modifiable.",
            online_note: "Online RNG must be authoritative and server-validated.",
        }
    }
}
