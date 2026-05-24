use ratatui::{layout::Rect, style::Style, text::Span, widgets::Paragraph, Frame};

use crate::{models::mode::Mode, ui::theme::ProxTheme};

pub fn draw_footer(frame: &mut Frame, area: Rect, mode: Mode, theme: &ProxTheme) {
    let (text, style) = match mode {
        Mode::Offline => (
            " OFFLINE | Virtual chips | Local saves | Sandbox ",
            theme.style_crimson(),
        ),
        Mode::Online => (
            " ONLINE | Placeholder | Server-authoritative later ",
            Style::default().fg(theme.amber).add_modifier(ratatui::style::Modifier::BOLD),
        ),
    };
    frame.render_widget(Paragraph::new(Span::styled(text, style)), area);
}
