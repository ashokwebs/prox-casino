use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::{app::App, games::slots::{MachineType, SlotSymbol}, ui::theme::ProxTheme};

pub fn draw(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(10), Constraint::Length(6)])
        .split(area);

    draw_top(frame, vert[0], app, theme);
    draw_reels(frame, vert[1], app, theme);
    draw_info(frame, vert[2], app, theme);
}

fn draw_top(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    let g = &app.slots;
    let line = Line::from(vec![
        Span::styled(" Machine ", theme.style_dim()),
        Span::styled(g.state.machine.name(), theme.style_crimson()),
        Span::raw("   "),
        Span::styled("Layout ", theme.style_dim()),
        Span::styled(format!("{}x{}", g.state.reel_count, g.state.visible_rows), theme.style_text()),
        Span::raw("   "),
        Span::styled("Bet ", theme.style_dim()),
        Span::styled(crate::utils::chip_format::format_chips(g.state.bet), theme.style_gold()),
        Span::raw("   "),
        Span::styled("Bankroll ", theme.style_dim()),
        Span::styled(crate::utils::chip_format::format_chips_long(g.state.display_chips), theme.style_gold()),
        Span::raw("   "),
        Span::styled("Mini/Mega/Ultra ", theme.style_dim()),
        Span::styled(crate::utils::chip_format::format_chips(g.state.jackpots.mini), theme.style_green()),
        Span::raw(" / "),
        Span::styled(crate::utils::chip_format::format_chips(g.state.jackpots.mega), theme.style_amber()),
        Span::raw(" / "),
        Span::styled(crate::utils::chip_format::format_chips(g.state.jackpots.ultra), theme.style_red()),
    ]);

    frame.render_widget(
        Paragraph::new(line)
            .block(theme.block("Slots Floor"))
            .alignment(Alignment::Center),
        area,
    );
}

fn draw_reels(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    let g = &app.slots;
    let flash = g.state.flash_counter > 0 && g.state.flash_counter.is_multiple_of(2);
    let win = g.state.last_mult > 0;
    let accent = if flash && win {
        if g.state.last_mult >= 50 { theme.red } else { theme.gold }
    } else if g.state.spinning {
        theme.crimson
    } else {
        theme.text
    };
    let border = Style::default().fg(accent).add_modifier(Modifier::BOLD);

    let mut lines = vec![Line::from(vec![
        Span::styled(" PAYLINES ", theme.style_dim()),
        Span::styled("MID  TOP  BOT  DIAG", theme.style_amber()),
        Span::raw("    "),
        Span::styled(if g.state.spinning { "REELS SPINNING" } else { "READY TO SPIN" }, if g.state.spinning { theme.style_crimson() } else { theme.style_dim() }),
    ])];
    lines.push(Line::from(""));

    let reel_count = g.state.reel_count;
    let visible_rows = g.state.visible_rows;
    let labels = [" TOP  ", " MID  ", " BOT  "];
    let top = (0..reel_count).map(|_| "┏━━━━━━━━━━┓").collect::<Vec<_>>().join("  ");
    let separator = (0..reel_count).map(|_| "┠──────────┨").collect::<Vec<_>>().join("  ");
    let bottom = (0..reel_count).map(|_| "┗━━━━━━━━━━┛").collect::<Vec<_>>().join("  ");

    for row in 0..visible_rows {
        lines.push(Line::from(vec![
            Span::styled(if row < labels.len() { labels[row] } else { "      " }, theme.style_dim()),
            Span::styled(top.clone(), border),
        ]));

        for art_row in 0..5 {
            let mut spans = vec![Span::styled(if art_row == 2 { "      " } else { "      " }, theme.style_dim())];
            for col in 0..reel_count {
                let symbol = g.state.reels[col][row];
                let art = symbol.big_symbol();
                let art_style = symbol_style(theme, symbol, flash && win);
                spans.push(Span::styled("┃ ", border));
                spans.push(Span::styled(art[art_row], art_style));
                spans.push(Span::styled(" ┃", border));
                if col + 1 < reel_count {
                    spans.push(Span::raw("  "));
                }
            }
            lines.push(Line::from(spans));
        }

        lines.push(Line::from(vec![
            Span::styled("      ", theme.style_dim()),
            Span::styled(bottom.clone(), border),
        ]));

        if row + 1 < visible_rows {
            lines.push(Line::from(vec![
                Span::styled("      ", theme.style_dim()),
                Span::styled(separator.clone(), border),
            ]));
        }
    }

    if g.state.spinning {
        let frames = crate::ui::animations::slot_reel_frames();
        let idx = g.state.spin_frames_left as usize % frames.len();
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(frames[idx], theme.style_amber())));
    } else if win && g.state.flash_counter > 0 {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            celebration_text(g.state.last_mult),
            if g.state.last_mult >= 50 { theme.style_red() } else { theme.style_green() },
        )));
    }

    frame.render_widget(
        Paragraph::new(lines)
            .block(theme.block(g.state.machine.name()))
            .alignment(Alignment::Center),
        area,
    );
}

fn draw_info(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    let g = &app.slots;
    let auto = if g.state.auto_spin_remaining > 0 {
        format!("AUTO:{}", g.state.auto_spin_remaining)
    } else {
        "AUTO:OFF".to_string()
    };
    let key_symbols = g
        .state
        .machine
        .symbol_pool()
        .iter()
        .take(5)
        .map(|(symbol, _, _)| symbol.short_name())
        .collect::<Vec<_>>()
        .join(" ");

    let lines = vec![
        Line::from(Span::styled(g.state.message.as_str(), message_style(theme, g.state.last_mult))),
        Line::from(vec![
            Span::styled(" [Left/Right] Bet ", theme.style_dim()),
            Span::styled(" [Space] Spin ", theme.style_dim()),
            Span::styled(" [A] Auto ", theme.style_dim()),
            Span::styled(" [M] Machine ", theme.style_dim()),
            Span::styled(" [R] Rules ", theme.style_dim()),
            Span::styled(" [Esc] Dashboard ", theme.style_dim()),
            Span::raw("   "),
            Span::styled(auto, theme.style_amber()),
        ]),
        Line::from(vec![
            Span::styled(" Profile ", theme.style_dim()),
            Span::styled(machine_profile(g.state.machine).0, theme.style_text()),
            Span::raw("   "),
            Span::styled(" Focus ", theme.style_dim()),
            Span::styled(machine_profile(g.state.machine).1, theme.style_text()),
            Span::raw("   "),
            Span::styled(" Last ", theme.style_dim()),
            Span::styled(
                crate::utils::chip_format::format_chips(g.state.last_payout),
                if g.state.last_payout > 0 { theme.style_green() } else { theme.style_dim() },
            ),
        ]),
        Line::from(vec![
            Span::styled(" Key ", theme.style_dim()),
            Span::styled(key_symbols, theme.style_text()),
        ]),
    ];

    frame.render_widget(
        Paragraph::new(lines)
            .block(theme.block("Slots Controls"))
            .alignment(Alignment::Center),
        area,
    );
}

fn symbol_style(theme: &ProxTheme, symbol: SlotSymbol, flashing: bool) -> Style {
    if flashing {
        return Style::default().fg(theme.white).add_modifier(Modifier::BOLD);
    }

    match (symbol, symbol.rarity()) {
        (SlotSymbol::Cherry | SlotSymbol::Heart | SlotSymbol::Flame, _) => theme.style_red(),
        (SlotSymbol::Lemon | SlotSymbol::GoldBar, _) => theme.style_gold(),
        (SlotSymbol::Bell | SlotSymbol::Coin | SlotSymbol::Star, _) => theme.style_amber(),
        (SlotSymbol::Seven | SlotSymbol::Crown, _) => theme.style_green(),
        (SlotSymbol::Diamond | SlotSymbol::Cyber, _) => theme.style_cyan(),
        (SlotSymbol::Wild | SlotSymbol::Scatter, _) => theme.style_crimson(),
        (SlotSymbol::Bar | SlotSymbol::DoubleBar | SlotSymbol::TripleBar | SlotSymbol::Horseshoe, _) => theme.style_text(),
    }
}

fn message_style(theme: &ProxTheme, mult: i64) -> Style {
    if mult >= 50 {
        theme.style_red().add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK)
    } else if mult >= 20 {
        theme.style_amber().add_modifier(Modifier::BOLD)
    } else if mult >= 10 {
        Style::default().fg(theme.amber).add_modifier(Modifier::BOLD)
    } else {
        theme.style_text()
    }
}

fn celebration_text(mult: i64) -> &'static str {
    if mult >= 100 {
        "=== ULTRA JACKPOT ==="
    } else if mult >= 50 {
        "=== MEGA WIN ==="
    } else if mult >= 20 {
        "=== JACKPOT HIT ==="
    } else {
        "=== WIN ==="
    }
}

fn machine_profile(machine: MachineType) -> (&'static str, &'static str) {
    match machine {
        MachineType::Classic => ("3 reels | calm spread", "steady low-volatility lines"),
        MachineType::Cyber => ("5 reels | wild-heavy", "longer lines and bonus chases"),
        MachineType::Retro => ("3 reels | classic pay", "balanced jackpots and wilds"),
        MachineType::Neon => ("3 reels | fast swing", "lighter hits with sharp spikes"),
        MachineType::Hacker => ("5 reels | scatter-rich", "chaotic reels and bonus pressure"),
        MachineType::Elite => ("5 reels | high risk", "premium symbols and huge swings"),
        MachineType::Midnight => ("3 reels | dark mix", "bars, stars, and streak play"),
        MachineType::DiamondRush => ("3 reels | gem table", "diamond-heavy jackpots"),
        MachineType::Lucky7 => ("3 reels | seven chase", "lean pool with sharp payouts"),
        MachineType::Inferno => ("3 reels | hot table", "flame-heavy volatility"),
        MachineType::Monochrome => ("3 reels | bar stack", "clean classic payout map"),
    }
}
