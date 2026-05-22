use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::app::{App, View};
use crate::ui::theme::ProxTheme;

pub fn draw(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(18), Constraint::Length(11)])
        .split(area);

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(40), Constraint::Percentage(30)])
        .split(rows[0]);

    draw_account(frame, cols[0], app, theme);
    draw_stats(frame, cols[1], app, theme);
    draw_settings(frame, cols[2], app, theme);
    draw_floor_nav(frame, rows[1], app, theme);
}

fn draw_account(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    let d = &app.data;
    let can = d.player.can_claim_daily();
    let level = level_icon(d.player.chips);
    let net = d.stats.total_won - d.stats.total_bet;
    let display_name = if app.editing_name {
        format!("> {}_", app.name_input)
    } else {
        d.player.name.clone()
    };

    let lines = vec![
        Line::from(Span::styled("  .----------------------------.  ", theme.style_dim())),
        Line::from(Span::styled("  |       PROX CASINO         |  ", theme.style_crimson())),
        Line::from(Span::styled("  |      OFFLINE ACCOUNT      |  ", theme.style_gold())),
        Line::from(Span::styled("  |    chips / stats / play   |  ", theme.style_dim())),
        Line::from(Span::styled("  '----------------------------'  ", theme.style_dim())),
        Line::from(""),
        Line::from(vec![
            Span::styled("  PROFILE  ", theme.style_crimson()),
            Span::styled("████", theme.style_gold()),
            Span::styled(format!(" {}  ", display_name), theme.style_gold().add_modifier(Modifier::BOLD)),
            Span::styled(format!("Lvl {}", level.1), theme.style_amber()),
        ]),
        Line::from(vec![
            Span::styled("  TIER   ", theme.style_dim()), 
            Span::styled(level.2, if level.1 >= 10 { theme.style_gold() } else if level.1 >= 5 { theme.style_amber() } else { theme.style_dim() })
        ]),
        Line::from(vec![
            Span::styled("  CHIPS  ", theme.style_dim()),
            Span::styled(crate::utils::chip_format::format_chips_long(d.player.chips), theme.style_gold().add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  NET    ", theme.style_dim()),
            Span::styled(
                crate::utils::chip_format::format_chips(net.abs()),
                if net >= 0 { theme.style_green() } else { theme.style_red() },
            ),
            Span::styled(if net >= 0 { " ▲" } else { " ▼" }, if net >= 0 { theme.style_green() } else { theme.style_red() }),
        ]),
        Line::from(vec![
            Span::styled("  DAILY  ", theme.style_dim()),
            Span::styled(if can { "READY [d]" } else { "claimed" }, if can { theme.style_green() } else { theme.style_dim() }),
            Span::styled(if can { " ◉" } else { " ○" }, if can { theme.style_green() } else { theme.style_dim() }),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ──────────────────────  ", theme.style_dim()),
        ]),
        Line::from(vec![Span::styled("  [E] Edit name", theme.style_amber())]),
        Line::from(vec![Span::styled(
            if app.editing_name {
                "  [Enter] Save   [Esc] Cancel"
            } else {
                "  [D] Daily claim   [R] Dev note"
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

fn draw_stats(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    let s = &app.data.stats;
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(vertical[0]);
    let bottom = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(vertical[1]);

    let total_bj = s.blackjack_wins + s.blackjack_losses + s.blackjack_pushes;
    let win_rate = if total_bj > 0 {
        (s.blackjack_wins as f64 / total_bj as f64 * 100.0) as u64
    } else {
        0
    };
    let avg_bet = if s.games_played > 0 { s.total_bet / s.games_played as i64 } else { 0 };

    frame.render_widget(
        Paragraph::new(vec![
            Line::from(Span::styled(" Ledger", theme.style_crimson())),
            Line::from(""),
            Line::from(vec![Span::styled("  Played   ", theme.style_dim()), Span::raw(s.games_played.to_string())]),
            Line::from(vec![Span::styled("  Win %    ", theme.style_dim()), Span::styled(format!("{}%", win_rate), if win_rate >= 50 { theme.style_green() } else { theme.style_amber() })]),
            Line::from(vec![Span::styled("  Total Bet", theme.style_dim()), Span::styled(crate::utils::chip_format::format_chips(s.total_bet), theme.style_red())]),
            Line::from(vec![Span::styled("  Total Won", theme.style_dim()), Span::styled(crate::utils::chip_format::format_chips(s.total_won), theme.style_green())]),
            Line::from(vec![Span::styled("  Avg Bet  ", theme.style_dim()), Span::styled(crate::utils::chip_format::format_chips(avg_bet), theme.style_amber())]),
            Line::from(vec![Span::styled("  Highest  ", theme.style_dim()), Span::styled(crate::utils::chip_format::format_chips(s.highest_bet), theme.style_gold())]),
        ])
        .block(theme.block("Ledger"))
        .style(Style::default().fg(theme.text)),
        top[0],
    );

    frame.render_widget(
        Paragraph::new(vec![
            Line::from(Span::styled(" Blackjack Record", theme.style_crimson())),
            Line::from(""),
            Line::from(vec![Span::styled("  Wins    ", theme.style_dim()), Span::styled(s.blackjack_wins.to_string(), theme.style_green())]),
            Line::from(vec![Span::styled("  Losses  ", theme.style_dim()), Span::styled(s.blackjack_losses.to_string(), theme.style_red())]),
            Line::from(vec![Span::styled("  Pushes  ", theme.style_dim()), Span::styled(s.blackjack_pushes.to_string(), theme.style_amber())]),
            Line::from(vec![Span::styled("  BJ Hits ", theme.style_dim()), Span::styled(s.blackjack_count.to_string(), theme.style_gold())]),
            Line::from(vec![Span::styled("  Best Stk", theme.style_dim()), Span::styled(s.blackjack_best_streak.to_string(), theme.style_gold())]),
            Line::from(vec![Span::styled("  Live Stk", theme.style_dim()), Span::styled(s.blackjack_streak.to_string(), if s.blackjack_streak >= 0 { theme.style_green() } else { theme.style_red() })]),
            Line::from(vec![Span::styled("  Win Streak", theme.style_dim()), Span::styled(s.blackjack_win_streak.to_string(), theme.style_green())]),
            Line::from(vec![Span::styled("  Loss Streak", theme.style_dim()), Span::styled(s.blackjack_loss_streak.to_string(), theme.style_red())]),
            Line::from(vec![Span::styled("  Push Streak", theme.style_dim()), Span::styled(s.blackjack_push_streak.to_string(), theme.style_amber())]),
            Line::from(vec![Span::styled("  Perfect BJ", theme.style_dim()), Span::styled(s.perfect_blackjacks.to_string(), theme.style_gold())]),
        ])
        .block(theme.block("Blackjack"))
        .style(Style::default().fg(theme.text)),
        top[1],
    );

    frame.render_widget(
        Paragraph::new(vec![
            Line::from(Span::styled(" Table Notes", theme.style_crimson())),
            Line::from(""),
            Line::from(vec![Span::styled("  Busts      ", theme.style_dim()), Span::raw(s.bust_count.to_string())]),
            Line::from(vec![Span::styled("  Dealer Bust", theme.style_dim()), Span::raw(s.dealer_bust_count.to_string())]),
            Line::from(vec![Span::styled("  Biggest Win", theme.style_dim()), Span::styled(crate::utils::chip_format::format_chips(s.biggest_win), theme.style_green())]),
            Line::from(vec![Span::styled("  Double Downs Won", theme.style_dim()), Span::styled(s.double_down_wins.to_string(), theme.style_green())]),
            Line::from(vec![Span::styled("  Double Downs Lost", theme.style_dim()), Span::styled(s.double_down_losses.to_string(), theme.style_red())]),
            Line::from(vec![Span::styled("  Splits Won", theme.style_dim()), Span::styled(s.split_wins.to_string(), theme.style_green())]),
            Line::from(vec![Span::styled("  Splits Lost", theme.style_dim()), Span::styled(s.split_losses.to_string(), theme.style_red())]),
            Line::from(vec![Span::styled("  Save Ver   ", theme.style_dim()), Span::raw(app.data.version.clone())]),
        ])
        .block(theme.block("Table Notes"))
        .style(Style::default().fg(theme.text)),
        bottom[0],
    );

    frame.render_widget(
        Paragraph::new(vec![
            Line::from(Span::styled(" Casino Floor", theme.style_crimson())),
            Line::from(""),
            Line::from(vec![Span::styled("  Slots Spins", theme.style_dim()), Span::raw(s.slots_spins.to_string())]),
            Line::from(vec![Span::styled("  Slot Best ", theme.style_dim()), Span::styled(crate::utils::chip_format::format_chips(s.slots_biggest_win), theme.style_amber())]),
            Line::from(vec![Span::styled("  Mega JP   ", theme.style_dim()), Span::styled(s.jackpot_count_mega.to_string(), theme.style_amber())]),
            Line::from(vec![Span::styled("  Ultra JP  ", theme.style_dim()), Span::styled(s.jackpot_count_ultra.to_string(), theme.style_red())]),
            Line::from(vec![Span::styled("  Mini JP   ", theme.style_dim()), Span::styled(s.jackpot_count_mini.to_string(), theme.style_green())]),
            Line::from(vec![Span::styled("  Win Streak", theme.style_dim()), Span::styled(s.slots_win_streak.to_string(), theme.style_green())]),
            Line::from(vec![Span::styled("  Loss Streak", theme.style_dim()), Span::styled(s.slots_loss_streak.to_string(), theme.style_red())]),
            Line::from(vec![Span::styled("  Cherry    ", theme.style_dim()), Span::styled(s.cherry_matches.to_string(), theme.style_red())]),
            Line::from(vec![Span::styled("  Lemon     ", theme.style_dim()), Span::styled(s.lemon_matches.to_string(), Style::default().fg(theme.yellow))]),
            Line::from(vec![Span::styled("  Bell      ", theme.style_dim()), Span::styled(s.bell_matches.to_string(), Style::default().fg(theme.gray))]),
            Line::from(vec![Span::styled("  Seven     ", theme.style_dim()), Span::styled(s.seven_matches.to_string(), theme.style_gold())]),
            Line::from(vec![Span::styled("  Diamond   ", theme.style_dim()), Span::styled(s.diamond_matches.to_string(), theme.style_cyan())]),
            Line::from(vec![Span::styled("  Wild      ", theme.style_dim()), Span::styled(s.wild_matches.to_string(), Style::default().fg(theme.white))]),
            Line::from(vec![Span::styled("  Scatter   ", theme.style_dim()), Span::styled(s.scatter_matches.to_string(), Style::default().fg(theme.magenta))]),
        ])
        .block(theme.block("Casino Floor"))
        .style(Style::default().fg(theme.text)),
        bottom[1],
    );
}

fn draw_settings(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    // Settings panel with profile information and options
    let d = &app.data;
    let net = d.stats.total_won - d.stats.total_bet;
    let level = level_icon(d.player.chips);
    
    let lines = vec![
        Line::from(""),
        Line::from(Span::styled("  SETTINGS PANEL  ", theme.style_crimson().add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Player:  ", theme.style_dim()),
            Span::styled(d.player.name.clone(), theme.style_gold().add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  Level:   ", theme.style_dim()),
            Span::styled(level.2, if level.1 >= 10 { theme.style_gold() } else if level.1 >= 5 { theme.style_amber() } else { theme.style_dim() }),
        ]),
        Line::from(vec![
            Span::styled("  Chips:   ", theme.style_dim()),
            Span::styled(crate::utils::chip_format::format_chips_long(d.player.chips), theme.style_gold().add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  Net:     ", theme.style_dim()),
            Span::styled(
                crate::utils::chip_format::format_chips(net.abs()),
                if net >= 0 { theme.style_green() } else { theme.style_red() },
            ),
            Span::styled(if net >= 0 { " ▲" } else { " ▼" }, if net >= 0 { theme.style_green() } else { theme.style_red() }),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ──────────────────────  ", theme.style_dim()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  OPTIONS:  ", theme.style_crimson()),
        ]),
        Line::from(vec![Span::styled("  [E] Edit Name    ", theme.style_amber())]),
        Line::from(vec![Span::styled("  [D] Daily Bonus  ", if d.player.can_claim_daily() { theme.style_green() } else { theme.style_dim() })]),
        Line::from(vec![Span::styled("  [R] Dev Note     ", theme.style_amber())]),
        Line::from(vec![Span::styled("  [1-3] Select Game", theme.style_amber())]),
        Line::from(vec![Span::styled("  [Q] Quit Game    ", theme.style_red())]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  VERSION: ", theme.style_dim()),
            Span::styled(env!("CARGO_PKG_VERSION"), theme.style_text()),
        ]),
        Line::from(""),
    ];

    frame.render_widget(
        Paragraph::new(lines)
            .block(theme.block("Settings"))
            .style(Style::default().fg(theme.text)),
        area,
    );
}

fn draw_floor_nav(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    let entries = [
        (View::Blackjack, "Blackjack Table", "Big cards, double, split, animated finish"),
        (View::Slots, "Slots Floor", "Jackpots, stronger reel panel, quick machine swap"),
        (View::Online, "Online Lounge", "Placeholder room while offline stays primary"),
    ];

    let mut lines = vec![Line::from(vec![
        Span::styled("  [↑/↓] ", theme.style_amber()),
        Span::styled("Move", theme.style_dim()),
        Span::raw("   "),
        Span::styled("[Enter] ", theme.style_amber()),
        Span::styled("Open", theme.style_dim()),
        Span::raw("   "),
        Span::styled("[R] ", theme.style_amber()),
        Span::styled("Note", theme.style_dim()),
        Span::raw("   "),
        Span::styled("[1-4] ", theme.style_amber()),
        Span::styled("Jump", theme.style_dim()),
    ])];

    for (i, (_, title, subtitle)) in entries.iter().enumerate() {
        let selected = i == app.dashboard_cursor;
        let accent = if selected { theme.style_crimson() } else { theme.style_dim() };
        let label = if selected {
            Style::default().fg(theme.hl_fg).bg(theme.hl_bg).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.text).add_modifier(Modifier::BOLD)
        };
        lines.push(Line::from(vec![
            Span::styled(if selected { " ► " } else { "   " }, accent),
            Span::styled(*title, label),
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

fn level_icon(chips: i64) -> (&'static str, u32, &'static str) {
    if chips >= 100_000_000 { ("CROWN", 15, "High Roller") }
    else if chips >= 10_000_000 { ("ROYAL", 10, "Royal") }
    else if chips >= 1_000_000 { ("DIAMOND", 6, "Diamond") }
    else if chips >= 100_000 { ("GOLD", 3, "Gold") }
    else { ("BRONZE", 1, "Bronze") }
}
