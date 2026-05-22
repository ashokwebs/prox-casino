use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone)]
pub struct ProxTheme {
    pub bg: Color,
    pub surface: Color,
    pub text: Color,
    pub dim: Color,
    pub crimson: Color,
    pub gold: Color,
    pub green: Color,
    pub red: Color,
    pub amber: Color,
    pub cyan: Color,
    pub border: Color,
    pub card_red: Color,
    pub card_black: Color,
    pub hl_bg: Color,
    pub hl_fg: Color,
    pub yellow: Color,
    pub gray: Color,
    pub white: Color,
    pub magenta: Color,
}

impl ProxTheme {
    pub fn dark_crimson() -> Self {
        Self {
            bg: Color::Rgb(8, 2, 4),
            surface: Color::Rgb(16, 6, 10),
            text: Color::Rgb(215, 200, 205),
            dim: Color::Rgb(110, 90, 95),
            crimson: Color::Rgb(210, 50, 50),
            gold: Color::Rgb(210, 160, 40),
            green: Color::Rgb(50, 190, 90),
            red: Color::Rgb(220, 35, 35),
            amber: Color::Rgb(220, 140, 40),
            cyan: Color::Rgb(40, 170, 220),
            border: Color::Rgb(150, 30, 30),
            card_red: Color::Rgb(220, 60, 60),
            card_black: Color::Rgb(190, 185, 190),
            hl_bg: Color::Rgb(210, 50, 50),
            hl_fg: Color::Rgb(8, 2, 4),
            yellow: Color::Rgb(220, 220, 100),
            gray: Color::Rgb(150, 150, 150),
            white: Color::Rgb(255, 255, 255),
            magenta: Color::Rgb(220, 100, 220),
        }
    }

    pub fn style_text(&self) -> Style { Style::default().fg(self.text) }
    pub fn style_dim(&self) -> Style { Style::default().fg(self.dim) }
    pub fn style_crimson(&self) -> Style { Style::default().fg(self.crimson).add_modifier(Modifier::BOLD) }
    pub fn style_gold(&self) -> Style { Style::default().fg(self.gold).add_modifier(Modifier::BOLD) }
    pub fn style_green(&self) -> Style { Style::default().fg(self.green).add_modifier(Modifier::BOLD) }
    pub fn style_red(&self) -> Style { Style::default().fg(self.red).add_modifier(Modifier::BOLD) }
    pub fn style_amber(&self) -> Style { Style::default().fg(self.amber).add_modifier(Modifier::BOLD) }
    pub fn style_cyan(&self) -> Style { Style::default().fg(self.cyan).add_modifier(Modifier::BOLD) }

    pub fn block(&self, title: &str) -> ratatui::widgets::Block<'static> {
        ratatui::widgets::Block::default()
            .title(format!(" {} ", title))
            .title_style(Style::default().fg(self.crimson).add_modifier(Modifier::BOLD))
            .borders(ratatui::widgets::Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(self.border))
    }
}
