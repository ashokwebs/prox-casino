mod app;
mod core;
mod games;
mod models;
mod network;
mod security;
mod services;
mod storage;
mod ui;
mod utils;

use std::{io, time::Duration};

use anyhow::Result;
use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::time::sleep;
use tracing_subscriber::EnvFilter;

use crate::app::App;
use crate::core::TICK_RATE_MS;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .compact()
        .init();

    let mut terminal = setup_terminal()?;
    let mut app = App::new()?;

    let tick_rate = Duration::from_millis(TICK_RATE_MS);

    while app.running {
        terminal.draw(|f| ui::draw(f, &app))?;
        app.handle_events()?;
        app.tick();
        sleep(tick_rate).await;
    }

    app.persist()?;
    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}
