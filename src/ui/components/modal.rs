use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::ui::theme::ProxTheme;

#[allow(dead_code)]
pub struct Modal {
    pub title: String,
    pub body: String,
    pub visible: bool,
}

#[allow(dead_code)]
impl Modal {
    pub fn new(title: String, body: String) -> Self {
        Self {
            title,
            body,
            visible: false,
        }
    }

    pub fn show(&mut self, title: String, body: String) {
        self.title = title;
        self.body = body;
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect, theme: &ProxTheme) {
        if !self.visible {
            return;
        }

        let modal_area = centered_rect(area, 72, 68);
        frame.render_widget(Clear, modal_area);

        let block = Block::default()
            .title(format!(" {} ", self.title))
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(theme.amber))
            .style(Style::default().bg(Color::Black));

        let paragraph = Paragraph::new(self.body.as_str())
            .block(block)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(theme.text));

        frame.render_widget(paragraph, modal_area);
    }
}

fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_x = r.width * percent_x / 100;
    let popup_y = r.height * percent_y / 100;
    let x = r.x + (r.width - popup_x) / 2;
    let y = r.y + (r.height - popup_y) / 2;
    Rect {
        x,
        y,
        width: popup_x,
        height: popup_y,
    }
}
