use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Tabs},
    Frame,
};

use crate::{app::View, core::APP_NAME, models::mode::Mode, ui::theme::ProxTheme};

pub fn draw_header(frame: &mut Frame, area: Rect, view: View, mode: Mode, theme: &ProxTheme) {
    let titles = [
        (View::Dashboard, " DASHBOARD "),
        (View::Blackjack, " BLACKJACK "),
        (View::Slots, " SLOTS "),
        (View::Online, " ONLINE "),
    ];

    let selected = titles.iter().position(|(v, _)| *v == view).unwrap_or(0);
    let tabs = Tabs::new(
        titles.iter().map(|(_, t)| Line::from(*t)).collect::<Vec<_>>(),
    )
    .block(
        Block::default()
            .title(format!(
                " {}  v{}  [{}] ",
                APP_NAME,
                crate::core::APP_VERSION,
                match mode { Mode::Offline => "OFFLINE", Mode::Online => "ONLINE" }
            ))
            .title_style(Style::default().fg(theme.crimson).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme.border)),
    )
    .highlight_style(Style::default().fg(theme.hl_fg).bg(theme.hl_bg).add_modifier(Modifier::BOLD))
    .select(selected)
    .divider(" │ ");

    frame.render_widget(tabs, area);
}
