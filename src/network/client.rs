use crate::network::protocol::{ClientAction, ServerEvent};

#[derive(Debug, Default)]
pub struct OnlineClient {
    connected: bool,
}

#[allow(dead_code)]
impl OnlineClient {
    pub async fn connect(&mut self, _endpoint: &str) -> anyhow::Result<()> {
        self.connected = false;
        Ok(())
    }

    pub async fn send_action(&self, _action: ClientAction) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn next_event(&self) -> anyhow::Result<Option<ServerEvent>> {
        Ok(None)
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }
}
