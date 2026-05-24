use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::app::{App, View};
use crate::ui::theme::ProxTheme;
use crate::utils::chip_format::{format_chips, format_chips_long};

pub fn draw(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(20), Constraint::Length(11)])
        .split(area);

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(35),
            Constraint::Percentage(35),
        ])
        .split(rows[0]);

    let middle = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Min(10)])
        .split(cols[1]);
    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Min(10)])
        .split(cols[2]);

    draw_account(frame, cols[0], app, theme);
    draw_ledger(frame, middle[0], app, theme);
    draw_blackjack(frame, middle[1], app, theme);
    draw_slots(frame, right[0], app, theme);
    draw_sessions(frame, right[1], app, theme);
    draw_floor_nav(frame, rows[1], app, theme);
}

fn draw_account(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    let player = &app.data.player;
    let net = app.data.stats.total_won - app.data.stats.total_bet;
    let (level, tier) = level_icon(player.chips);
    let session_net = player.chips - app.session_start_chips;
    let display_name = if app.editing_name {
        format!("> {}_", app.name_input)
    } else {
        player.name.clone()
    };

    let lines = vec![
        Line::from(Span::styled("  .------------------------.  ", theme.style_dim())),
        Line::from(Span::styled("  |      PROX CASINO      |  ", theme.style_crimson())),
        Line::from(Span::styled("  |     OFFLINE FLOOR     |  ", theme.style_gold())),
        Line::from(Span::styled("  '------------------------'  ", theme.style_dim())),
        Line::from(""),
        Line::from(vec![
            Span::styled("  PLAYER   ", theme.style_dim()),
            Span::styled(display_name, theme.style_gold().add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  LEVEL    ", theme.style_dim()),
            Span::styled(level, theme.style_amber()),
            Span::raw("  "),
            Span::styled(tier, theme.style_dim()),
        ]),
        Line::from(vec![
            Span::styled("  CHIPS    ", theme.style_dim()),
            Span::styled(format_chips_long(player.chips), theme.style_gold().add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  NET      ", theme.style_dim()),
            Span::styled(
                format_chips(net),
                if net >= 0 { theme.style_green() } else { theme.style_red() },
            ),
        ]),
        Line::from(vec![
            Span::styled("  SESSION  ", theme.style_dim()),
            Span::styled(
                format_chips(session_net),
                if session_net >= 0 { theme.style_green() } else { theme.style_red() },
            ),
            Span::raw("  "),
            Span::styled(format_duration(app.session_start_time.elapsed().as_secs()), theme.style_dim()),
        ]),
        Line::from(vec![
            Span::styled("  DAILY    ", theme.style_dim()),
            Span::styled(
                if player.can_claim_daily() { "READY [D]" } else { "CLAIMED" },
                if player.can_claim_daily() { theme.style_green() } else { theme.style_dim() },
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled("  [E] Edit name", theme.style_amber())]),
        Line::from(vec![Span::styled("  [R] Help", theme.style_amber())]),
        Line::from(vec![Span::styled(
            if app.editing_name {
                "  [Enter] Save   [Esc] Cancel"
            } else {
                "  [Q] Save and quit"
            },
            theme.style_dim(),
        )]),
    ];

    frame.render_widget(
        Paragraph::new(lines)
            .block(theme.block("Account"))
            .style(Style::default().fg(theme.text)),
        area,
    );
}

fn draw_ledger(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    let stats = &app.data.stats;
    let net = stats.total_won - stats.total_bet;
    let session_net = app.data.player.chips - app.session_start_chips;

    let lines = vec![
        Line::from(vec![
            Span::styled(" Current bankroll ", theme.style_dim()),
            Span::styled(format_chips_long(app.data.player.chips), theme.style_gold()),
        ]),
        Line::from(vec![
            Span::styled(" Session move      ", theme.style_dim()),
            Span::styled(
                format_chips(session_net),
                if session_net >= 0 { theme.style_green() } else { theme.style_red() },
            ),
        ]),
        Line::from(vec![Span::styled(" Total bet         ", theme.style_dim()), Span::styled(format_chips(stats.total_bet), theme.style_red())]),
        Line::from(vec![Span::styled(" Total won         ", theme.style_dim()), Span::styled(format_chips(stats.total_won), theme.style_green())]),
        Line::from(vec![Span::styled(" Lifetime net      ", theme.style_dim()), Span::styled(format_chips(net), if net >= 0 { theme.style_green() } else { theme.style_red() })]),
        Line::from(vec![Span::styled(" Avg bet           ", theme.style_dim()), Span::styled(format_chips(stats.average_bet), theme.style_amber())]),
        Line::from(vec![Span::styled(" Highest bet       ", theme.style_dim()), Span::styled(format_chips(stats.highest_bet), theme.style_gold())]),
        Line::from(vec![Span::styled(" Biggest win       ", theme.style_dim()), Span::styled(format_chips(stats.biggest_win), theme.style_green())]),
    ];

    frame.render_widget(
        Paragraph::new(lines)
            .block(theme.block("Ledger"))
            .style(Style::default().fg(theme.text)),
        area,
    );
}

fn draw_blackjack(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    let stats = &app.data.stats;
    let total_bj = stats.blackjack_wins + stats.blackjack_losses + stats.blackjack_pushes;
    let win_rate = if total_bj > 0 {
        (stats.blackjack_wins as f64 / total_bj as f64 * 100.0) as u64
    } else {
        0
    };

    let lines = vec![
        Line::from(vec![Span::styled(" Wins          ", theme.style_dim()), Span::styled(stats.blackjack_wins.to_string(), theme.style_green())]),
        Line::from(vec![Span::styled(" Losses        ", theme.style_dim()), Span::styled(stats.blackjack_losses.to_string(), theme.style_red())]),
        Line::from(vec![Span::styled(" Pushes        ", theme.style_dim()), Span::styled(stats.blackjack_pushes.to_string(), theme.style_amber())]),
        Line::from(vec![Span::styled(" Win rate      ", theme.style_dim()), Span::styled(format!("{}%", win_rate), if win_rate >= 50 { theme.style_green() } else { theme.style_amber() })]),
        Line::from(vec![Span::styled(" Live streak   ", theme.style_dim()), Span::styled(stats.blackjack_streak.to_string(), if stats.blackjack_streak >= 0 { theme.style_green() } else { theme.style_red() })]),
        Line::from(vec![Span::styled(" Best streak   ", theme.style_dim()), Span::styled(stats.blackjack_best_streak.to_string(), theme.style_gold())]),
        Line::from(vec![Span::styled(" Perfect BJ    ", theme.style_dim()), Span::styled(stats.perfect_blackjacks.to_string(), theme.style_gold())]),
        Line::from(vec![Span::styled(" Busts         ", theme.style_dim()), Span::styled(stats.bust_count.to_string(), theme.style_red())]),
        Line::from(vec![Span::styled(" Dealer busts  ", theme.style_dim()), Span::styled(stats.dealer_bust_count.to_string(), theme.style_green())]),
        Line::from(vec![Span::styled(" Double W/L    ", theme.style_dim()), Span::styled(format!("{}/{}", stats.double_down_wins, stats.double_down_losses), theme.style_text())]),
        Line::from(vec![Span::styled(" Split W/L     ", theme.style_dim()), Span::styled(format!("{}/{}", stats.split_wins, stats.split_losses), theme.style_text())]),
    ];

    frame.render_widget(
        Paragraph::new(lines)
            .block(theme.block("Blackjack"))
            .style(Style::default().fg(theme.text)),
        area,
    );
}

fn draw_slots(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    let stats = &app.data.stats;

    let lines = vec![
        Line::from(vec![Span::styled(" Spins         ", theme.style_dim()), Span::styled(stats.slots_spins.to_string(), theme.style_text())]),
        Line::from(vec![Span::styled(" Biggest hit   ", theme.style_dim()), Span::styled(format_chips(stats.slots_biggest_win), theme.style_amber())]),
        Line::from(vec![Span::styled(" Mini / Mega   ", theme.style_dim()), Span::styled(format!("{} / {}", stats.jackpot_count_mini, stats.jackpot_count_mega), theme.style_green())]),
        Line::from(vec![Span::styled(" Ultra         ", theme.style_dim()), Span::styled(stats.jackpot_count_ultra.to_string(), theme.style_red())]),
        Line::from(vec![Span::styled(" Win streak    ", theme.style_dim()), Span::styled(stats.slots_win_streak.to_string(), theme.style_green())]),
        Line::from(vec![Span::styled(" Loss streak   ", theme.style_dim()), Span::styled(stats.slots_loss_streak.to_string(), theme.style_red())]),
        Line::from(vec![Span::styled(" CHRY / LMON   ", theme.style_dim()), Span::styled(format!("{} / {}", stats.cherry_matches, stats.lemon_matches), theme.style_text())]),
        Line::from(vec![Span::styled(" BELL / SEVN   ", theme.style_dim()), Span::styled(format!("{} / {}", stats.bell_matches, stats.seven_matches), theme.style_text())]),
        Line::from(vec![Span::styled(" DIAM / WILD   ", theme.style_dim()), Span::styled(format!("{} / {}", stats.diamond_matches, stats.wild_matches), theme.style_text())]),
        Line::from(vec![Span::styled(" SCATTER       ", theme.style_dim()), Span::styled(stats.scatter_matches.to_string(), theme.style_cyan())]),
    ];

    frame.render_widget(
        Paragraph::new(lines)
            .block(theme.block("Slots"))
            .style(Style::default().fg(theme.text)),
        area,
    );
}

fn draw_sessions(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    let stats = &app.data.stats;
    let session_net = app.data.player.chips - app.session_start_chips;
    let mut lines = vec![
        Line::from(vec![Span::styled(" Current run", theme.style_crimson())]),
        Line::from(vec![Span::styled("  Length  ", theme.style_dim()), Span::styled(format_duration(app.session_start_time.elapsed().as_secs()), theme.style_text())]),
        Line::from(vec![Span::styled("  Net     ", theme.style_dim()), Span::styled(format_chips(session_net), if session_net >= 0 { theme.style_green() } else { theme.style_red() })]),
        Line::from(vec![Span::styled("  Games   ", theme.style_dim()), Span::styled(stats.games_played.to_string(), theme.style_text())]),
        Line::from(""),
        Line::from(vec![Span::styled(" Recent saves", theme.style_crimson())]),
    ];

    if stats.session_history.is_empty() {
        lines.push(Line::from(Span::styled("  No saved sessions yet", theme.style_dim())));
    } else {
        for session in stats.session_history.iter().rev().take(3) {
            lines.push(Line::from(vec![
                Span::styled("  ", theme.style_dim()),
                Span::styled(session.date.as_str(), theme.style_text()),
            ]));
            lines.push(Line::from(vec![
                Span::styled("    ", theme.style_dim()),
                Span::styled(format_duration(session.duration_seconds), theme.style_dim()),
                Span::raw("  "),
                Span::styled(
                    format_chips(session.net_change),
                    if session.net_change >= 0 { theme.style_green() } else { theme.style_red() },
                ),
            ]));
        }
    }

    frame.render_widget(
        Paragraph::new(lines)
            .block(theme.block("Sessions"))
            .style(Style::default().fg(theme.text)),
        area,
    );
}

fn draw_floor_nav(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    let entries = [
        (View::Blackjack, "Blackjack Table", "Cards, doubles, splits, dealer pacing"),
        (View::Slots, "Slots Floor", "Reels, jackpots, machine rotation"),
        (View::Online, "Online Lounge", "Placeholder for later network play"),
    ];

    let mut lines = vec![Line::from(vec![
        Span::styled("  [Up/Down] ", theme.style_amber()),
        Span::styled("Move", theme.style_dim()),
        Span::raw("   "),
        Span::styled("[Enter] ", theme.style_amber()),
        Span::styled("Open", theme.style_dim()),
        Span::raw("   "),
        Span::styled("[1-4] ", theme.style_amber()),
        Span::styled("Jump", theme.style_dim()),
        Span::raw("   "),
        Span::styled("[R] ", theme.style_amber()),
        Span::styled("Guide", theme.style_dim()),
    ])];

    for (i, (_, title, subtitle)) in entries.iter().enumerate() {
        let selected = i == app.dashboard_cursor;
        let label_style = if selected {
            Style::default()
                .fg(theme.hl_fg)
                .bg(theme.hl_bg)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.text).add_modifier(Modifier::BOLD)
        };

        lines.push(Line::from(vec![
            Span::styled(if selected { " > " } else { "   " }, if selected { theme.style_crimson() } else { theme.style_dim() }),
            Span::styled(*title, label_style),
        ]));
        lines.push(Line::from(vec![
            Span::styled("    ", theme.style_dim()),
            Span::styled(*subtitle, if selected { theme.style_amber() } else { theme.style_dim() }),
        ]));
    }

    frame.render_widget(
        Paragraph::new(lines)
            .block(theme.block("Floor Select"))
            .style(Style::default().fg(theme.text)),
        area,
    );
}

fn level_icon(chips: i64) -> (&'static str, &'static str) {
    if chips >= 100_000_000 {
        ("LVL 15", "High Roller")
    } else if chips >= 10_000_000 {
        ("LVL 10", "Royal")
    } else if chips >= 1_000_000 {
        ("LVL 06", "Diamond")
    } else if chips >= 100_000 {
        ("LVL 03", "Gold")
    } else {
        ("LVL 01", "Bronze")
    }
}

fn format_duration(total_seconds: u64) -> String {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}", minutes, seconds)
    }
}
