use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Paragraph},
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
        .constraints([Constraint::Length(16), Constraint::Min(15), Constraint::Length(5)])
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
    let line = if matches!(app.bj.state, PlayState::RoundOver) && app.bj.settled {
        let (title, accent) = round_banner(app, theme);
        Line::from(vec![
            Span::styled(format!(" {} ", title), Style::default().fg(accent).add_modifier(Modifier::BOLD)),
            Span::raw("  "),
            Span::styled(app.bj.message.as_str(), Style::default().fg(theme.text)),
            Span::raw("    "),
            Span::styled("[Space] Deal", Style::default().fg(theme.dim)),
        ])
    } else {
        Line::from(Span::styled(app.bj.message.as_str(), Style::default().fg(theme.text)))
    };

    frame.render_widget(
        Paragraph::new(line).block(theme.block("Status")),
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

#[allow(dead_code)]
fn banner_art(title: &str) -> &'static [&'static str] {
    match title {
        "BLACKJACK" => &[
            r"  ____  _        _    ____ _  __    _   _    ____ _  __ ",
            r" | __ )| |      / \  / ___| |/ /   | | / \  / ___| |/ / ",
            r" |  _ \| |     / _ \| |   | ' / _  | |/ _ \| |   | ' /  ",
            r" | |_) | |___ / ___ \ |___| . \| |_| / ___ \ |___| . \  ",
            r" |____/|_____/_/   \_\____|_|\_\\___/_/   \_\____|_|\_\ ",
        ],
        "YOU WIN" => &[
            r" __   __  ___  _   _  __        __ ___  _   _  ",
            r" \ \ / / / _ \| | | | \ \      / /|_ _|| \ | | ",
            r"  \ V / | | | | | | |  \ \ /\ / /  | | |  \| | ",
            r"   | |  | |_| | |_| |   \ V  V /   | | | |\  | ",
            r"   |_|   \___/ \___/     \_/\_/   |___||_| \_| ",
        ],
        "HOUSE WINS" => &[
            r" _   _  ___  _   _ ____  _____  __        __ ___  _   _  ____  ",
            r"| | | |/ _ \| | | / ___|| ____| \ \      / /|_ _|| \ | |/ ___| ",
            r"| |_| | | | | | | \___ \|  _|    \ \ /\ / /  | | |  \| |\___ \ ",
            r"|  _  | |_| | |_| |___) | |___    \ V  V /   | | | |\  | ___) |",
            r"|_| |_|\___/ \___/|____/|_____|    \_/\_/   |___||_| \_||____/ ",
        ],
        "PUSH" => &[
            r"  ____  _   _  ____  _   _  ",
            r" |  _ \| | | |/ ___|| | | | ",
            r" | |_) | | | |\___ \| |_| | ",
            r" |  __/| |_| | ___) |  _  | ",
            r" |_|    \___/ |____/|_| |_| ",
        ],
        _ => &[
            r"  ____   ___  _   _ _   _ ____   ",
            r" |  _ \ / _ \| | | | \ | |  _ \  ",
            r" | |_) | | | | | | |  \| | | | | ",
            r" |  _ <| |_| | |_| | |\  | |_| | ",
            r" |_| \_\\___/ \___/|_| \_|____/  ",
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

fn flash_color(app: &App, theme: &ProxTheme) -> Option<Color> {
    let anim = app.bj.anim.as_ref()?;
    if anim.flash == 0 || anim.flash % 8 < 4 {
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
    let border_fg = Color::Rgb(90, 80, 85);
    let sym = c.suit_char();
    let rank = card_value_single(c);
    let s = flash.map_or(
        Style::default().fg(suit_fg).add_modifier(Modifier::BOLD),
        |fg| Style::default().fg(fg).add_modifier(Modifier::BOLD),
    );
    let b = flash.map_or(
        Style::default().fg(border_fg).add_modifier(Modifier::BOLD),
        |fg| Style::default().fg(fg).add_modifier(Modifier::BOLD),
    );
    let big_suit = match c.suit {
        crate::games::blackjack::Suit::Spades => vec![
            Span::styled("   ◢◤◥◣   ", s),
            Span::styled("  ◢████◣  ", s),
            Span::styled(" ◢██████◣ ", s),
            Span::styled(" ◥██████◤ ", s),
            Span::styled("    ██    ", s),
        ],
        crate::games::blackjack::Suit::Hearts => vec![
            Span::styled("  ◢████◣  ", s),
            Span::styled(" ◢██████◣ ", s),
            Span::styled(" ◥██████◤ ", s),
            Span::styled("  ◥████◤  ", s),
            Span::styled("   ◥◤◥◤   ", s),
        ],
        crate::games::blackjack::Suit::Diamonds => vec![
            Span::styled("    ◢◣    ", s),
            Span::styled("   ◢██◣   ", s),
            Span::styled("  ◢████◣  ", s),
            Span::styled("   ◥██◤   ", s),
            Span::styled("    ◥◤    ", s),
        ],
        crate::games::blackjack::Suit::Clubs => vec![
            Span::styled("    ◢◣    ", s),
            Span::styled("  ◢████◣  ", s),
            Span::styled(" ◢██████◣ ", s),
            Span::styled(" ◥██████◤ ", s),
            Span::styled("    ██    ", s),
        ],
    };

    // Every row = 19 chars: ┃(1) + 17 inner + ┃(1)
    vec![
        Line::from(Span::styled("┏━━━━━━━━━━━━━━━━━┓", b)),
        // " K ♠             " = 1+2+1+1+12 = 17
        Line::from(vec![
            Span::styled("┃ ", b), Span::styled(format!("{:<2}", rank), s),
            Span::styled(" ", b), Span::styled(sym, s), Span::styled("            ┃", b),
        ]),
        Line::from(Span::styled("┃                 ┃", b)),
        Line::from(vec![Span::styled("┃   ", b), big_suit[0].clone(), Span::styled("    ┃", b)]),
        Line::from(vec![Span::styled("┃   ", b), big_suit[1].clone(), Span::styled("    ┃", b)]),
        Line::from(vec![Span::styled("┃   ", b), big_suit[2].clone(), Span::styled("    ┃", b)]),
        Line::from(vec![Span::styled("┃   ", b), big_suit[3].clone(), Span::styled("    ┃", b)]),
        Line::from(vec![Span::styled("┃   ", b), big_suit[4].clone(), Span::styled("    ┃", b)]),
        Line::from(Span::styled("┃                 ┃", b)),
        Line::from(vec![
            Span::styled("┃            ", b), Span::styled(sym, s),
            Span::styled(" ", b), Span::styled(format!("{:>2}", rank), s), Span::styled(" ┃", b),
        ]),
        Line::from(Span::styled("┗━━━━━━━━━━━━━━━━━┛", b)),
    ]
}

fn card_back(_theme: &ProxTheme, flash: Option<Color>) -> Vec<Line<'static>> {
    let fg = flash.unwrap_or(Color::Rgb(130, 25, 25));
    let c = Style::default().fg(fg).add_modifier(Modifier::BOLD);
    // Single color, 19 chars: ┃(1) + 17 inner + ┃(1)
    vec![
        Line::from(Span::styled("┏━━━━━━━━━━━━━━━━━┓", c)),
        Line::from(vec![Span::styled("┃", c), Span::styled("█▓█▓█▓█▓█▓█▓█▓█▓█", c), Span::styled("┃", c)]),
        Line::from(vec![Span::styled("┃", c), Span::styled("▓█▓█▓█▓█▓█▓█▓█▓█▓", c), Span::styled("┃", c)]),
        Line::from(vec![Span::styled("┃", c), Span::styled("█▓█▓█▓█▓█▓█▓█▓█▓█", c), Span::styled("┃", c)]),
        Line::from(vec![Span::styled("┃", c), Span::styled("▓█▓█▓█▓█▓█▓█▓█▓█▓", c), Span::styled("┃", c)]),
        Line::from(vec![Span::styled("┃", c), Span::styled("█▓█▓█▓█▓█▓█▓█▓█▓█", c), Span::styled("┃", c)]),
        Line::from(vec![Span::styled("┃", c), Span::styled("▓█▓█▓█▓█▓█▓█▓█▓█▓", c), Span::styled("┃", c)]),
        Line::from(vec![Span::styled("┃", c), Span::styled("█▓█▓█▓█▓█▓█▓█▓█▓█", c), Span::styled("┃", c)]),
        Line::from(vec![Span::styled("┃", c), Span::styled("▓█▓█▓█▓█▓█▓█▓█▓█▓", c), Span::styled("┃", c)]),
        Line::from(vec![Span::styled("┃", c), Span::styled("█▓█▓█▓█▓█▓█▓█▓█▓█", c), Span::styled("┃", c)]),
        Line::from(vec![Span::styled("┃", c), Span::styled("▓█▓█▓█▓█▓█▓█▓█▓█▓", c), Span::styled("┃", c)]),
        Line::from(Span::styled("┗━━━━━━━━━━━━━━━━━┛", c)),
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
                row.push(Span::raw(" "));
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

