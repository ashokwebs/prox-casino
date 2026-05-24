use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::ui::theme::ProxTheme;

pub fn draw(frame: &mut Frame, area: Rect, theme: &ProxTheme) {
    let lines = vec![
        Line::from(Span::styled(" ONLINE MODE | PLACEHOLDER ", theme.style_crimson())),
        Line::from(""),
        Line::from(Span::styled(" Architecture note:", Style::default().fg(theme.amber))),
        Line::from(" Online mode is scaffolded for future server-side integration."),
        Line::from(" The client is never authoritative."),
        Line::from(""),
        Line::from(Span::styled(" Planned:", Style::default().fg(theme.amber))),
        Line::from(" - Accounts and session tokens"),
        Line::from(" - Authoritative chip validation"),
        Line::from(" - Leaderboards and tournaments"),
        Line::from(" - Multiplayer tables"),
        Line::from(""),
        Line::from(Span::styled(" [R] Info  [1-4] Views  [o] Offline  [q] Exit ", theme.style_dim())),
    ];

    frame.render_widget(
        Paragraph::new(lines)
            .block(theme.block("Online"))
            .alignment(Alignment::Left)
            .style(Style::default().fg(theme.text)),
        area,
    );
}
