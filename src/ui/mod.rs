pub mod theme;
pub mod animations;
pub mod components;
pub mod screens;

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

use crate::app::{App, View};
use crate::ui::theme::ProxTheme;

pub fn draw(frame: &mut Frame, app: &App) {
    let size = frame.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(6),
            Constraint::Length(4),
            Constraint::Length(1),
        ])
        .split(size);

    let theme = &app.theme;
    components::header::draw_header(frame, chunks[0], app.view, app.mode, theme);
    draw_screen(frame, chunks[1], app, theme);
    components::notifications::draw_notes(frame, chunks[2], &app.notes, theme);
    components::footer::draw_footer(frame, chunks[3], app.mode, theme);
    app.help_modal.draw(frame, size, theme);
    app.result_modal.draw(frame, size, theme);
}

fn draw_screen(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    match app.view {
        View::Dashboard => screens::dashboard::draw(frame, area, app, theme),
        View::Blackjack => screens::blackjack::draw(frame, area, app, theme),
        View::Slots => screens::slots::draw(frame, area, app, theme),
        View::Online => screens::online::draw(frame, area, theme),
    }
}
