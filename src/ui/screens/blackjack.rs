use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Clear, Paragraph},
    Frame,
};

use crate::{
    app::App,
    games::blackjack::{hand_value, Card, Outcome, PlayState, Suit},
    ui::theme::ProxTheme,
};

pub fn draw(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(20), Constraint::Length(3)])
        .split(area);

    draw_top(frame, chunks[0], app, theme);
    draw_table(frame, chunks[1], app, theme);
    draw_cmd(frame, chunks[2], app, theme);
    
    // Draw centered overlay for blackjack results when game is over
    if matches!(app.bj.state, PlayState::RoundOver) && app.bj.settled {
        draw_centered_result_overlay(frame, area, app, theme);
    }
}

fn draw_top(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    let bj = &app.bj;
    let chips = app.data.player.chips;

    let state_tag = match bj.state {
        PlayState::Idle | PlayState::RoundOver => "IDLE",
        PlayState::Dealing { .. } => "DEAL",
        PlayState::PlayerTurn => "PLAY",
        PlayState::SplitTurn { .. } => "SPLIT",
        PlayState::DealerTurn => "HOUSE",
    };

    let tag_style = match bj.state {
        PlayState::PlayerTurn | PlayState::SplitTurn { .. } => theme.style_green(),
        PlayState::DealerTurn => theme.style_amber(),
        PlayState::RoundOver => theme.style_crimson(),
        _ => theme.style_dim(),
    };

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(format!(" {} ", state_tag), tag_style),
            Span::raw("  "),
            Span::styled("Bet:", theme.style_dim()),
            Span::raw(" "),
            Span::styled(crate::utils::chip_format::format_chips(bj.bet), Style::default().fg(theme.text).add_modifier(Modifier::BOLD)),
            Span::raw("  "),
            Span::styled("Chips:", theme.style_dim()),
            Span::raw(" "),
            Span::styled(crate::utils::chip_format::format_chips_long(chips), theme.style_gold()),
        ]))
        .block(theme.block("Blackjack Table")),
        area,
    );
}

fn draw_table(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(12), Constraint::Min(11), Constraint::Length(8)])
        .split(area);

    draw_house(frame, chunks[0], app, theme);
    draw_player(frame, chunks[1], app, theme);
    draw_bottom_msg(frame, chunks[2], app, theme);
}

fn draw_house(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    let bj = &app.bj;
    let mut lines: Vec<Line> = Vec::new();

    if bj.dealer.is_empty() {
        lines.push(Line::from(Span::styled(" Waiting for deal...", theme.style_dim())));
    } else {
        let dv = dealer_value_display(&bj.dealer, bj.reveal);
        lines.push(Line::from(Span::styled(dv, theme.style_dim())));

        let cards = bj
            .dealer
            .iter()
            .enumerate()
            .map(|(i, c)| {
                if !bj.reveal && i == 1 {
                    card_back(theme, flash_color(app, theme))
                } else {
                    card_front(c, theme, flash_color(app, theme))
                }
            })
            .collect::<Vec<_>>();
        merge_cards(&mut lines, &cards);
    }

    frame.render_widget(
        Paragraph::new(lines)
            .block(theme.block("House"))
            .style(Style::default().fg(theme.text)),
        area,
    );
}

fn dealer_value_display(dealer: &[Card], reveal: bool) -> String {
    if reveal {
        let v = hand_value(dealer);
        if v > 21 { format!("  HOUSE: {}  BUST", v) }
        else if v == 21 && dealer.len() == 2 { format!("  HOUSE: {}  BLACKJACK", v) }
        else { format!("  HOUSE: {}", v) }
    } else {
        let v = card_value_single(&dealer[0]);
        format!("  SHOWING: {}", v)
    }
}

fn card_value_single(c: &Card) -> String {
    match c.rank {
        1 => "A".into(),
        11 => "J".into(),
        12 => "Q".into(),
        13 => "K".into(),
        10 => "10".into(),
        n => n.to_string(),
    }
}

fn draw_player(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    let bj = &app.bj;
    let mut lines: Vec<Line> = Vec::new();

    if bj.hands.is_empty() || bj.hands.iter().all(|h| h.cards.is_empty()) {
        lines.push(Line::from(Span::styled(" Place your bet and press [Space]", theme.style_dim())));
        frame.render_widget(
            Paragraph::new(lines)
                .block(theme.block("Player"))
                .style(Style::default().fg(theme.text)),
            area,
        );
        return;
    }

    for (i, hand) in bj.hands.iter().enumerate() {
        let active = i == bj.active && !hand.done;
        let val = hand.value();
        let label = if bj.hands.len() > 1 {
            format!("Hand {}", i + 1)
        } else {
            "You".to_string()
        };

        let tag = match hand.outcome {
            Some(Outcome::Blackjack) => Some((" BLACKJACK ", theme.style_gold())),
            Some(Outcome::Win) => Some((" WIN ", theme.style_green())),
            Some(Outcome::Lose) => Some((" LOSE ", theme.style_red())),
            Some(Outcome::Push) => Some((" PUSH ", Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD))),
            None => None,
        };

        let val_style = if val > 21 {
            theme.style_red()
        } else if val == 21 {
            theme.style_gold()
        } else if val >= 17 {
            theme.style_green()
        } else {
            Style::default().fg(theme.text).add_modifier(Modifier::BOLD)
        };

        let mut parts = vec![
            Span::styled(if active { "► " } else { "  " }, theme.style_amber()),
            Span::styled(label, if active { theme.style_amber() } else { theme.style_crimson() }),
            Span::raw("  "),
            Span::styled(format!("[{}]", val), val_style),
            Span::raw("  "),
            Span::styled(crate::utils::chip_format::format_chips(hand.bet), theme.style_dim()),
        ];
        if let Some((text, style)) = tag {
            parts.push(Span::raw(" "));
            parts.push(Span::styled(text, style));
        }
        if hand.doubled {
            parts.push(Span::raw(" "));
            parts.push(Span::styled("DOUBLE", theme.style_amber()));
        }
        lines.push(Line::from(parts));

        if !hand.cards.is_empty() {
            let cards = hand
                .cards
                .iter()
                .map(|c| card_front(c, theme, flash_color(app, theme)))
                .collect::<Vec<_>>();
            merge_cards(&mut lines, &cards);
        }
        lines.push(Line::from(""));
    }

    frame.render_widget(
        Paragraph::new(lines)
            .block(theme.block("Player"))
            .style(Style::default().fg(theme.text)),
        area,
    );
}

fn draw_bottom_msg(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    let msg = app.bj.message.clone();

    frame.render_widget(
        Paragraph::new(Span::styled(msg.as_str(), Style::default().fg(theme.text).add_modifier(Modifier::BOLD)))
            .block(theme.block("Status")),
        area,
    );
}

fn round_banner(app: &App, theme: &ProxTheme) -> (&'static str, Color) {
    let has_bj = any_outcome(&app.bj.hands, Outcome::Blackjack);
    let has_win = any_outcome(&app.bj.hands, Outcome::Win);
    let has_lose = any_outcome(&app.bj.hands, Outcome::Lose);
    let has_push = any_outcome(&app.bj.hands, Outcome::Push);

    if has_bj {
        ("BLACKJACK", theme.gold)
    } else if has_win && has_lose {
        ("MIXED ROUND", theme.amber)
    } else if has_win {
        ("YOU WIN", theme.green)
    } else if has_lose {
        ("HOUSE WINS", theme.red)
    } else if has_push {
        ("PUSH", theme.cyan)
    } else {
        ("ROUND OVER", theme.amber)
    }
}

fn banner_art(title: &str) -> &'static [&'static str] {
    match title {
        "BLACKJACK" => &[
            r" ██████ █          █████ ██████ █   █          ████ █████ ██████ █   █",
            r" █   █ █          █   █ █      █  █            █   █   █ █      █  █ ",
            r" █████ █          █████ █      ███             █   █████ █      ███  ",
            r" █   █ █          █   █ █      █  █        █   █   █   █ █      █  █ ",
            r" █████ ███████    █   █ ██████ █   █       ███    █   █ ██████ █   █",
        ],
        "YOU WIN" => &[
            r" ███   ███   █   █       █   █ ███ █   █ ",
            r" █ █  █   █  █   █       █   █  █  ██  █ ",
            r"  █   █   █  █   █       █ █ █  █  █ █ █ ",
            r"  █   █   █  █   █       ██ ██  █  █  ██ ",
            r"  █    ███    ███        █   █ ███ █   █ ",
        ],
        "HOUSE WINS" => &[
            r" █   █ ███   █   █ ████ ████      █   █ ███ █   █ ████ ",
            r" █   █ █   █ █   █ █    █         █   █  █  ██  █ █    ",
            r" █████ █   █ █   █ ███  ███       █ █ █  █  █ █ █ ███  ",
            r" █   █ █   █ █   █    █ █         ██ ██  █  █  ██ █    ",
            r" █   █ ███   ███  ████ ████      █   █ ███ █   █ ████ ",
        ],
        "PUSH" => &[
            r" █████ █   █ ████ █   █ ",
            r" █   █ █   █ █    █   █ ",
            r" █████ █   █ ███  █████ ",
            r" █     █   █    █ █   █ ",
            r" █      ███  ████ █   █ ",
        ],
        _ => &[
            r" ██   ██ ███ █   █ ████ ████ ",
            r" █ █ █ █  █   █ █  █    █   █",
            r" █  █  █  █    █   ███  █   █",
            r" █     █  █   █ █  █    █   █",
            r" █     █ ███ █   █ ████ ████ ",
        ],
    }
}

fn any_outcome(hands: &[crate::games::blackjack::Hand], outcome: Outcome) -> bool {
    hands.iter().any(|h| h.outcome == Some(outcome))
}

fn draw_cmd(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    let txt = match app.bj.state {
        PlayState::Idle | PlayState::RoundOver => " [←/→] Bet  [↑/↓] +10K  [Space] Deal  [R] Rules  [Esc] Dashboard ",
        PlayState::Dealing { .. } => " Dealing... ",
        PlayState::PlayerTurn => " [H] Hit  [S] Stand  [D] Double  [P] Split  [R] Rules  [Esc] Dashboard ",
        PlayState::SplitTurn { .. } => " [H] Hit  [S] Stand  [D] Double  [R] Rules  [Esc] Dashboard ",
        PlayState::DealerTurn => " House drawing... ",
    };

    frame.render_widget(
        Paragraph::new(Span::styled(txt, Style::default().fg(theme.text)))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(theme.border)),
            )
            .style(Style::default().fg(theme.text)),
        area,
    );
}

fn draw_centered_result_overlay(frame: &mut Frame, area: Rect, app: &App, theme: &ProxTheme) {
    if app.bj.anim.as_ref().map_or(0, |a| a.flash) == 0 {
        return;
    }
    
    let (title, accent) = round_banner(app, theme);
    let pulse = app.bj.anim.as_ref().map(|a| a.flash > 0 && a.flash % 2 == 0).unwrap_or(false);
    let overlay_area = centered_rect(area, 62, 45);
    
    frame.render_widget(Clear, overlay_area);
    
    let art = banner_art(title);
    let max_art_width = art.iter().map(|line| line.len()).max().unwrap_or(24);
    let art_width = max_art_width + 4;
    
    let block = Block::default()
        .title(format!(" {} ", title))
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(accent))
        .style(if pulse {
            Style::default().bg(accent)
        } else {
            Style::default().bg(Color::Black)
        });
    
    let mut lines = Vec::new();
    for row in art {
        let padded = format!("{:^art_width$}", row, art_width = art_width);
        lines.push(Line::from(Span::styled(
            padded,
            if pulse {
                Style::default().fg(theme.surface).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(accent).add_modifier(Modifier::BOLD)
            },
        )));
    }
    
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!("{:^art_width$}", app.bj.message, art_width = art_width),
        if pulse {
            Style::default().fg(theme.surface).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.text).add_modifier(Modifier::BOLD)
        },
    )));
    
    frame.render_widget(
        Paragraph::new(lines)
            .block(block)
            .alignment(Alignment::Center),
        overlay_area,
    );
}

fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_x = r.width * percent_x / 100;
    let popup_y = r.height * percent_y / 100;
    let x = r.x + (r.width - popup_x) / 2;
    let y = r.y + (r.height - popup_y) / 2;
    Rect {
        x,
        y,
        width: popup_x.max(20), // Ensure minimum width
        height: popup_y.max(10), // Ensure minimum height
    }
}

fn flash_color(app: &App, theme: &ProxTheme) -> Option<Color> {
    let anim = app.bj.anim.as_ref()?;
    if anim.flash == 0 || anim.flash % 2 != 0 {
        return None;
    }
    Some(match anim.kind {
        Outcome::Blackjack => theme.gold,
        Outcome::Win => theme.green,
        Outcome::Lose => theme.red,
        Outcome::Push => theme.cyan,
    })
}

fn card_front(c: &Card, theme: &ProxTheme, flash: Option<Color>) -> Vec<Line<'static>> {
    let is_red = matches!(c.suit, Suit::Hearts | Suit::Diamonds);
    let suit_fg = if is_red { theme.card_red } else { theme.card_black };
    let border_fg = Color::Rgb(160, 150, 155);
    let sym = c.suit_char();
    let rank = card_value_single(c);
    let face_style = flash.map_or(
        Style::default().fg(suit_fg).add_modifier(Modifier::BOLD),
        |fg| Style::default().fg(fg).add_modifier(Modifier::BOLD),
    );
    let border_style = flash.map_or(
        Style::default().fg(border_fg).add_modifier(Modifier::BOLD),
        |fg| Style::default().fg(fg).add_modifier(Modifier::BOLD),
    );
    let suit_row = format!("{} {} {}", sym, sym, sym);

    vec![
        Line::from(Span::styled("┏━━━━━━━━━━━┓", border_style)),
        Line::from(vec![
            Span::styled("┃", border_style),
            Span::styled(format!(" {:<2}", rank), face_style),
            Span::styled("        ┃", border_style),
        ]),
        Line::from(Span::styled("┃           ┃", border_style)),
        Line::from(vec![
            Span::styled("┃   ", border_style),
            Span::styled(sym, face_style),
            Span::styled("   ", border_style),
            Span::styled(sym, face_style),
            Span::styled("   ┃", border_style),
        ]),
        Line::from(vec![
            Span::styled("┃    ", border_style),
            Span::styled(sym, face_style),
            Span::styled("    ┃", border_style),
        ]),
        Line::from(vec![
            Span::styled("┃  ", border_style),
            Span::styled(suit_row, face_style),
            Span::styled("  ┃", border_style),
        ]),
        Line::from(vec![
            Span::styled("┃    ", border_style),
            Span::styled(sym, face_style),
            Span::styled("    ┃", border_style),
        ]),
        Line::from(vec![
            Span::styled("┃   ", border_style),
            Span::styled(sym, face_style),
            Span::styled("   ", border_style),
            Span::styled(sym, face_style),
            Span::styled("   ┃", border_style),
        ]),
        Line::from(vec![
            Span::styled("┃        ", border_style),
            Span::styled(format!("{:>2} ", rank), face_style),
            Span::styled("┃", border_style),
        ]),
        Line::from(Span::styled("┗━━━━━━━━━━━┛", border_style)),
    ]
}

fn card_back(theme: &ProxTheme, flash: Option<Color>) -> Vec<Line<'static>> {
    let fg = flash.unwrap_or(theme.crimson);
    let style = Style::default().fg(fg).add_modifier(Modifier::BOLD);
    vec![
        Line::from(Span::styled("┏━━━━━━━━━━━┓", style)),
        Line::from(Span::styled("┃▓▓▓▓▓▓▓▓▓▓▓┃", style)),
        Line::from(Span::styled("┃▓▒▓▒▓▒▓▒▓▒▓┃", style)),
        Line::from(Span::styled("┃▒▓▒▓▒▓▒▓▒▓▒┃", style)),
        Line::from(Span::styled("┃▓▒▓▒▓▒▓▒▓▒▓┃", style)),
        Line::from(Span::styled("┃▒▓▒▓▒▓▒▓▒▓▒┃", style)),
        Line::from(Span::styled("┃▓▓▓▓▓▓▓▓▓▓▓┃", style)),
        Line::from(Span::styled("┗━━━━━━━━━━━┛", style)),
    ]
}

fn merge_cards(lines: &mut Vec<Line>, card_lines: &[Vec<Line<'static>>]) {
    if card_lines.is_empty() {
        return;
    }
    let rows = card_lines[0].len();
    for row_idx in 0..rows {
        let mut row: Vec<Span> = Vec::new();
        for (ci, card) in card_lines.iter().enumerate() {
            if ci > 0 {
                row.push(Span::raw("  "));
            }
            if let Some(line) = card.get(row_idx) {
                for span in &line.spans {
                    row.push(span.clone());
                }
            }
        }
        lines.push(Line::from(row));
    }
}
