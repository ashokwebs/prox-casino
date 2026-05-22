use ratatui::{
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::ui::theme::ProxTheme;

pub fn draw_notes(frame: &mut Frame, area: Rect, notes: &[String], theme: &ProxTheme) {
    let lines: Vec<Line> = notes.iter().rev().take(4).rev()
        .map(|n| Line::from(format!(" ● {n}")))
        .collect();
    frame.render_widget(
        Paragraph::new(lines)
            .block(Block::default()
                .title(" Feed ")
                .title_style(theme.style_crimson())
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border)))
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(theme.text)),
        area,
    );
}
