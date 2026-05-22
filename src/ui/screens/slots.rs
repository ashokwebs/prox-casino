use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::{app::App, ui::theme::ProxTheme};

pub fn draw(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(10), Constraint::Length(5)])
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
        Span::styled("Bet ", theme.style_dim()),
        Span::styled(crate::utils::chip_format::format_chips(g.state.bet), theme.style_gold()),
        Span::raw("   "),
        Span::styled("Chips ", theme.style_dim()),
        Span::styled(crate::utils::chip_format::format_chips_long(app.data.player.chips), theme.style_gold()),
        Span::raw("   "),
        Span::styled("Mini ", theme.style_dim()),
        Span::styled(crate::utils::chip_format::format_chips(g.state.jackpots.mini), theme.style_green()),
        Span::raw("   "),
        Span::styled("Mega ", theme.style_dim()),
        Span::styled(crate::utils::chip_format::format_chips(g.state.jackpots.mega), theme.style_amber()),
        Span::raw("   "),
        Span::styled("Ultra ", theme.style_dim()),
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
    let face = Style::default().fg(accent).add_modifier(Modifier::BOLD);

    let mut lines = vec![Line::from(Span::styled(
        if g.state.spinning { "        REELS SPINNING        " } else { "         READY TO SPIN        " },
        if g.state.spinning { theme.style_crimson() } else { theme.style_dim() },
    ))];

    let reels = g.state.reels.iter().map(|s| s.icon()).collect::<Vec<_>>();
    
    // Enhanced reel drawing with better spacing and visuals
    let top = reels.iter().map(|_| "┏━━━━━━━┓").collect::<Vec<_>>().join("  ");
    let mid1 = reels.iter().map(|_| "┃       ┃").collect::<Vec<_>>().join("  ");
    let mid2 = reels.iter().map(|icon| {
        // Add pulsing effect for winning symbols
        if flash && win {
            format!("┃  <{:^1}>  ┃", icon)
        } else {
            format!("┃   {:^1}   ┃", icon)
        }
    }).collect::<Vec<_>>().join("  ");
    let mid3 = reels.iter().map(|_| "┃       ┃").collect::<Vec<_>>().join("  ");
    let bot = reels.iter().map(|_| "┗━━━━━━━┛").collect::<Vec<_>>().join("  ");

    lines.push(Line::from(Span::styled(top, border)));
    lines.push(Line::from(Span::styled(mid1, border)));
    lines.push(Line::from(Span::styled(mid2, face)));
    lines.push(Line::from(Span::styled(mid3, border)));
    lines.push(Line::from(Span::styled(bot, border)));

    // Enhanced spinning animation with more frames
    if g.state.spinning {
        let f = crate::ui::animations::slot_reel_frames();
        let idx = g.state.spin_frames_left as usize % f.len();
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(format!("          {}", f[idx]), theme.style_amber())));
        
        // Add spinning indicator
        let spin_indicator = match (g.state.spin_frames_left / 4) % 4 {
            0 => "◢◤",
            1 => "◣◥",
            2 => "◥◣",
            3 => "◤◢",
            _ => ""
        };
        lines.push(Line::from(Span::styled(format!("          {}", spin_indicator), theme.style_crimson())));
    }
    
    // Enhanced celebration for big wins
    if win && !g.state.spinning && g.state.flash_counter > 0 {
        lines.push(Line::from(""));
        let celebration = if g.state.last_mult >= 100 {
            "🎉 ULTRA JACKPOT! 🎉"
        } else if g.state.last_mult >= 50 {
            "💰 MEGA WIN! 💰"
        } else if g.state.last_mult >= 20 {
            "✨ BIG WIN! ✨"
        } else {
            "🎺 WIN! 🎺"
        };
        lines.push(Line::from(Span::styled(
            celebration,
            if g.state.last_mult >= 50 { theme.style_red() } else { theme.style_green() }
                .add_modifier(Modifier::BOLD)
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
        format!("Auto:{}", g.state.auto_spin_remaining)
    } else {
        "Auto:OFF".to_string()
    };

    let mult = g.state.last_mult;
    let msg_style = if mult >= 50 {
        theme.style_red().add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK)
    } else if mult >= 20 {
        theme.style_amber().add_modifier(Modifier::BOLD)
    } else if mult >= 10 {
        Style::default().fg(theme.amber).add_modifier(Modifier::BOLD)
    } else {
        theme.style_text()
    };

    let lines = vec![
        Line::from(Span::styled(g.state.message.as_str(), msg_style)),
        Line::from(vec![
            Span::styled(" [←/→] Bet ", theme.style_dim()),
            Span::styled(" [Space] Spin ", theme.style_dim()),
            Span::styled(" [A] Auto ", theme.style_dim()),
            Span::styled(" [M] Machine ", theme.style_dim()),
            Span::styled(" [R] Rules ", theme.style_dim()),
            Span::styled(" [Esc] Dashboard ", theme.style_dim()),
            Span::raw("   "),
            Span::styled(auto, theme.style_amber()),
        ]),
    ];

    frame.render_widget(
        Paragraph::new(lines)
            .block(theme.block("Slots Controls"))
            .alignment(Alignment::Center),
        area,
    );
}
