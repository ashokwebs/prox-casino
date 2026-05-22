use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::{
    core::DAILY_BONUS,
    games::blackjack::{hand_value, BlackjackEngine, Outcome, PlayState},
    games::slots::{MachineType, SlotSymbol, SlotsGame},
    models::mode::Mode,
    models::stats::SessionSummary,
    network::client::OnlineClient,
    services::save::SaveService,
    storage::local::SaveData,
    ui::components::modal::Modal,
    ui::theme::ProxTheme,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Dashboard,
    Blackjack,
    Slots,
    Online,
}

pub struct App {
    pub running: bool,
    pub mode: Mode,
    pub view: View,
    pub dashboard_cursor: usize,
    pub editing_name: bool,
    pub name_input: String,
    pub notes: Vec<String>,
    pub data: SaveData,
    pub bj: BlackjackEngine,
    pub slots: SlotsGame,
    pub theme: ProxTheme,
    pub help_modal: Modal,
    pub result_modal: Modal,
    pub save_pending: bool,
    pub save_ct: u8,
    #[allow(dead_code)]
    pub online: OnlineClient,
    pub session_start_chips: i64,
    pub session_start_time: std::time::Instant,
    save_svc: SaveService,
}

impl App {
    const DASHBOARD_VIEWS: [View; 3] = [View::Blackjack, View::Slots, View::Online];

    pub fn new() -> Result<Self> {
        let svc = SaveService::new()?;
        let d = svc.load()?;
        Ok(Self {
            running: true,
            mode: Mode::Offline,
            view: View::Dashboard,
            dashboard_cursor: 0,
            editing_name: false,
            name_input: d.player.name.clone(),
            notes: vec![format!("Loaded: {}", svc.path())],
            data: d.clone(),
            bj: BlackjackEngine::new(),
            slots: SlotsGame::default(),
            theme: ProxTheme::dark_crimson(),
            help_modal: Modal::new(String::new(), String::new()),
            result_modal: Modal::new(String::new(), String::new()),
            save_pending: false,
            save_ct: 0,
            online: OnlineClient::default(),
            session_start_chips: d.player.chips,
            session_start_time: std::time::Instant::now(),
            save_svc: svc,
        })
    }

    pub fn push_note(&mut self, msg: impl Into<String>) {
        self.notes.push(msg.into());
        if self.notes.len() > 6 { self.notes.remove(0); }
    }

    pub fn dashboard_target(&self) -> View {
        Self::DASHBOARD_VIEWS
            .get(self.dashboard_cursor)
            .copied()
            .unwrap_or(View::Blackjack)
    }

    pub fn persist(&self) -> Result<()> {
        // Update session history before saving
        let mut data = self.data.clone();
        let session_duration = self.session_start_time.elapsed().as_secs();
        let net_change = data.player.chips - self.session_start_chips;
        
        // Create session summary
        let session = SessionSummary {
            date: chrono::Local::now().format("%Y-%m-%d %H:%M").to_string(),
            duration_seconds: session_duration,
            starting_chips: self.session_start_chips,
            ending_chips: data.player.chips,
            net_change,
            blackjack_wins: data.stats.blackjack_wins,
            blackjack_losses: data.stats.blackjack_losses,
            slots_spins: data.stats.slots_spins,
            biggest_win: data.stats.biggest_win,
            biggest_loss: if net_change < 0 { net_change.abs() } else { 0 },
        };
        
        // Add to history (keep last 10 sessions)
        data.stats.session_history.push(session);
        if data.stats.session_history.len() > 10 {
            data.stats.session_history.remove(0);
        }
        
        self.save_svc.save(&data)
    }

    fn request_save(&mut self) { self.save_pending = true; self.save_ct = 8; }

    fn move_dashboard_cursor(&mut self, delta: i32) {
        let len = Self::DASHBOARD_VIEWS.len() as i32;
        let next = (self.dashboard_cursor as i32 + delta).rem_euclid(len);
        self.dashboard_cursor = next as usize;
    }

    fn open_dashboard_target(&mut self) {
        let target = self.dashboard_target();
        self.view = target;
        self.push_note(match target {
            View::Blackjack => "Blackjack",
            View::Slots => "Slots",
            View::Online => "Online",
            View::Dashboard => "Dashboard",
        });
    }

    fn record_highest_bet(&mut self, bet: i64) {
        if bet > self.data.stats.highest_bet {
            self.data.stats.highest_bet = bet;
        }
    }

    fn show_help(&mut self, title: &str, body: &str) {
        self.help_modal.show(title.to_string(), body.to_string());
    }

    fn show_rules_for_current_view(&mut self) {
        match self.view {
            View::Dashboard => self.show_help(
                "Developer Note",
                "WELCOME TO THE OFFLINE FLOOR\n\nThis build is tuned to feel punchy, dramatic, and local-first.\nNo network drama. No account friction. Just chips, stats, jackpots, and fast rounds.\n\nCurrent focus:\n- Blackjack should feel premium and readable\n- Slots should feel heavier, slower, and rarer on wins\n- Dashboard should feel like your private casino profile\n\nHotkeys:\n- [E] edit your offline name\n- [D] claim daily chips\n- [↑/↓] move between floors\n- [Enter] jump in\n\nDeveloper vibe:\nPlay loud. Chase streaks. Break your own best stats.",
            ),
            View::Blackjack => self.show_help(
                "Blackjack Rules",
                "OBJECTIVE\nBeat the house without going over 21.\n\nFLOW\n- You start with 2 cards\n- Dealer shows 1 card and hides 1 card\n- Face cards = 10\n- Aces = 1 or 11\n\nACTIONS\n- [H] Hit: take 1 card\n- [S] Stand: hold your total\n- [D] Double: double bet, take exactly 1 card, then stand\n- [P] Split: if both opening cards match, split into 2 hands\n\nHOUSE\nDealer reveals and draws until at least 17.\nSoft 17 is hit. Hard 17 stands.\n\nPAYOUTS\n- Win: +1x bet\n- Blackjack: +1.5x bet\n- Push: 0\n- Loss: -1x bet",
            ),
            View::Slots => self.show_help(
                "Slots Rules",
                "OBJECTIVE\nSpin matching symbols for payouts. Wins are intentionally rarer now.\n\nFLOW\n- Set bet with arrows\n- [Space] spins once\n- [A] toggles auto x10\n- [M] changes machine\n\nMACHINES\n- Classic / Retro / Neon use 3 reels\n- Cyber / Hacker use 5 reels\n\nPAYOUT LOGIC\n- Full matches pay best\n- Near-full matches mostly matter on 5-reel machines\n- Wilds help complete lines\n- Scatter now needs 3+ symbols for bonus\n\nJACKPOTS\nEach bet feeds mini, mega, and ultra pots.",
            ),
            View::Online => self.show_help(
                "Online Lounge",
                "Online mode is still a placeholder.\n\nPlanned later:\n- accounts\n- authoritative balance checks\n- leaderboards\n- multiplayer rooms\n\nFor now, the real experience is the offline floor.",
            ),
        }
    }

    fn begin_name_edit(&mut self) {
        self.editing_name = true;
        self.name_input = self.data.player.name.clone();
        self.push_note("Editing offline name");
    }

    fn commit_name_edit(&mut self) {
        let trimmed = self.name_input.trim();
        if trimmed.is_empty() {
            self.name_input = self.data.player.name.clone();
            self.editing_name = false;
            self.push_note("Name unchanged");
            return;
        }

        self.data.player.name = trimmed.chars().take(18).collect();
        self.name_input = self.data.player.name.clone();
        self.editing_name = false;
        self.request_save();
        self.push_note(format!("Name: {}", self.data.player.name));
    }

    fn cancel_name_edit(&mut self) {
        self.editing_name = false;
        self.name_input = self.data.player.name.clone();
        self.push_note("Name edit cancelled");
    }

    fn on_name_input(&mut self, code: KeyCode) {
        match code {
            KeyCode::Esc => self.cancel_name_edit(),
            KeyCode::Enter => self.commit_name_edit(),
            KeyCode::Backspace => {
                self.name_input.pop();
            }
            KeyCode::Char(c) if self.name_input.chars().count() < 18 && !c.is_control() => {
                self.name_input.push(c);
            }
            _ => {}
        }
    }

    pub fn tick(&mut self) {
        self.bj.tick();
        self.slots.tick_animation(self.data.player.chips);

        if let Some((payout, mult, scatter)) = self.slots.tick_spin() {
            // Chips were already deducted at spin start, so just add the payout
            self.data.player.chips = self.data.player.chips.saturating_add(payout);
            self.data.stats.games_played += 1;
            self.data.stats.slots_spins += 1;
            let net = payout - self.slots.state.bet;
            if payout > 0 {
                self.data.stats.total_won = self.data.stats.total_won.saturating_add(payout);
                if payout > self.data.stats.slots_biggest_win {
                    self.data.stats.slots_biggest_win = payout;
                }
                
                // Update slots win streak
                self.data.stats.slots_win_streak += 1;
                self.data.stats.slots_loss_streak = 0;
            } else {
                // Update slots loss streak
                self.data.stats.slots_loss_streak += 1;
                self.data.stats.slots_win_streak = 0;
            }
            
            // Count symbol appearances on reels
            for col in &self.slots.state.reels {
                for symbol in col {
                    match symbol {
                        SlotSymbol::Cherry => self.data.stats.cherry_matches += 1,
                        SlotSymbol::Lemon => self.data.stats.lemon_matches += 1,
                        SlotSymbol::Bell => self.data.stats.bell_matches += 1,
                        SlotSymbol::Seven => self.data.stats.seven_matches += 1,
                        SlotSymbol::Diamond => self.data.stats.diamond_matches += 1,
                        SlotSymbol::Wild => self.data.stats.wild_matches += 1,
                        SlotSymbol::Scatter => self.data.stats.scatter_matches += 1,
                        _ => {}
                    }
                }
            }
            
            if mult >= 100 { 
                self.data.stats.jackpot_count_ultra += 1; 
                self.data.stats.jackpot_count_mini += 1; // Ultra jackpot feeds mini too
            } else if mult >= 50 { 
                self.data.stats.jackpot_count_mega += 1; 
                self.data.stats.jackpot_count_mini += 1; // Mega jackpot feeds mini too
            } else if mult >= 20 {
                self.data.stats.jackpot_count_mini += 1; // Regular jackpot feeds mini
            }
            
            let note = if mult >= 100 { format!("██ ULTRA x{mult}! +{}", net) }
            else if mult >= 50 { format!("▓▓ MEGA x{mult}! +{}", net) }
            else if mult >= 20 { format!("▒▒ JACKPOT x{mult}! +{}", net) }
            else if net > 0 { format!("Win +{}", net) }
            else if scatter { format!("Scatter +{}", payout) }
            else { format!("Spin: {}", net) };
            self.push_note(note);
            self.request_save();
        }

        if self.bj.state == PlayState::RoundOver && !self.bj.settled {
            if hand_value(&self.bj.dealer) > 21 {
                self.data.stats.dealer_bust_count += 1;
            }
            let results = self.bj.settle();
            for (outcome, delta) in &results {
                self.apply_bj(outcome, *delta);
            }
            self.bj.settled = true;
            self.bj.anim = Some(crate::games::blackjack::AnimState {
                flash: crate::core::BJ_FLASH_FRAMES, kind: results.first().map(|r| r.0).unwrap_or(Outcome::Push),
            });
        }

        if self.slots.state.auto_spin_remaining > 0 && !self.slots.state.spinning {
            if self.slots.begin_spin(self.data.player.chips) {
                let bet = self.slots.state.bet;
                self.data.player.chips = self.data.player.chips.saturating_sub(bet);
                self.data.stats.total_bet += bet;
                self.slots.state.auto_spin_remaining -= 1;
            } else { self.slots.state.auto_spin_remaining = 0; }
        }

        if self.save_pending {
            if self.save_ct > 0 { self.save_ct -= 1; }
            else {
                self.save_pending = false;
                if let Err(e) = self.persist() { self.push_note(format!("Save: {e}")); }
            }
        }
    }

    fn apply_bj(&mut self, outcome: &Outcome, delta: i64) {
        let msg = match outcome {
            Outcome::Blackjack => {
                self.data.stats.blackjack_wins += 1;
                self.data.stats.blackjack_win_streak += 1;
                self.data.stats.blackjack_loss_streak = 0;
                self.data.stats.blackjack_push_streak = 0;
                if (self.data.stats.blackjack_win_streak as i64) > self.data.stats.blackjack_best_streak {
                    self.data.stats.blackjack_best_streak = self.data.stats.blackjack_win_streak as i64;
                }
                self.data.stats.blackjack_count += 1;
                self.data.player.chips = self.data.player.chips.saturating_add(delta);
                self.data.stats.total_won = self.data.stats.total_won.saturating_add(delta);
                if delta > self.data.stats.biggest_win { self.data.stats.biggest_win = delta; }
                format!("■■ BJ! +{}", crate::utils::chip_format::format_chips(delta))
            }
            Outcome::Win => {
                self.data.stats.blackjack_wins += 1;
                self.data.stats.blackjack_win_streak += 1;
                self.data.stats.blackjack_loss_streak = 0;
                self.data.stats.blackjack_push_streak = 0;
                if (self.data.stats.blackjack_win_streak as i64) > self.data.stats.blackjack_best_streak {
                    self.data.stats.blackjack_best_streak = self.data.stats.blackjack_win_streak as i64;
                }
                self.data.player.chips = self.data.player.chips.saturating_add(delta);
                self.data.stats.total_won = self.data.stats.total_won.saturating_add(delta);
                if delta > self.data.stats.biggest_win { self.data.stats.biggest_win = delta; }
                format!("Win +{}", crate::utils::chip_format::format_chips(delta))
            }
            Outcome::Lose => {
                self.data.stats.blackjack_losses += 1;
                self.data.stats.blackjack_loss_streak += 1;
                self.data.stats.blackjack_win_streak = 0;
                self.data.stats.blackjack_push_streak = 0;
                // Only count as bust if any player hand actually busted
                for h in &self.bj.hands {
                    if h.value() > 21 {
                        self.data.stats.bust_count += 1;
                        break;
                    }
                }
                self.data.player.chips = self.data.player.chips.saturating_add(delta);
                format!("Lose {}", crate::utils::chip_format::format_chips(delta.abs()))
            }
            Outcome::Push => {
                self.data.stats.blackjack_pushes += 1;
                self.data.stats.blackjack_push_streak += 1;
                self.data.stats.blackjack_win_streak = 0;
                self.data.stats.blackjack_loss_streak = 0;
                "Push".to_string()
            }
        };
        self.data.stats.games_played += 1;
        self.push_note(msg);
        self.request_save();
    }

    pub fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press { self.on_key(key.code)?; }
            }
        }
        Ok(())
    }

    fn on_key(&mut self, code: KeyCode) -> Result<()> {
        if self.editing_name {
            self.on_name_input(code);
            return Ok(());
        }

        if self.help_modal.visible {
            match code {
                KeyCode::Esc | KeyCode::Enter | KeyCode::Char('r') | KeyCode::Char('R') | KeyCode::Char(' ') => {
                    self.help_modal.hide();
                }
                _ => {}
            }
            return Ok(());
        }

        match code {
            KeyCode::Char('q') => self.running = false,
            KeyCode::Char('1') => self.view = View::Dashboard,
            KeyCode::Char('2') => { self.dashboard_cursor = 0; self.view = View::Blackjack; }
            KeyCode::Char('3') => { self.dashboard_cursor = 1; self.view = View::Slots; }
            KeyCode::Char('4') => { self.dashboard_cursor = 2; self.view = View::Online; }
            KeyCode::Char('o') | KeyCode::Char('O') => { self.mode = Mode::Offline; self.push_note("Offline"); }
            KeyCode::Char('n') | KeyCode::Char('N') => { self.mode = Mode::Online; self.push_note("Online"); }
            KeyCode::Char('r') | KeyCode::Char('R') => self.show_rules_for_current_view(),
            _ => self.on_view(code)?,
        }
        Ok(())
    }

    fn on_view(&mut self, code: KeyCode) -> Result<()> {
        match self.view {
            View::Dashboard => {
                match code {
                    KeyCode::Up => self.move_dashboard_cursor(-1),
                    KeyCode::Down => self.move_dashboard_cursor(1),
                    KeyCode::Enter | KeyCode::Char(' ') => self.open_dashboard_target(),
                    KeyCode::Char('e') | KeyCode::Char('E') => self.begin_name_edit(),
                    KeyCode::Char('d') | KeyCode::Char('D') => {
                        if self.data.player.claim_daily_bonus() {
                            self.push_note(format!("Daily +{}", crate::utils::chip_format::format_chips(DAILY_BONUS)));
                            self.request_save();
                        } else { self.push_note("Daily done"); }
                    }
                    _ => {}
                }
            }
            View::Blackjack => self.on_bj(code),
            View::Slots => self.on_slots(code),
            View::Online => {}
        }
        Ok(())
    }

    fn on_bj(&mut self, code: KeyCode) {
        match code {
            KeyCode::Left => self.bj.adjust_bet(-1_000, self.data.player.chips),
            KeyCode::Right => self.bj.adjust_bet(1_000, self.data.player.chips),
            KeyCode::Up => self.bj.adjust_bet(10_000, self.data.player.chips),
            KeyCode::Down => self.bj.adjust_bet(-10_000, self.data.player.chips),
            KeyCode::Enter | KeyCode::Char(' ') if self.bj.deal(self.data.player.chips) => {
                self.data.stats.total_bet += self.bj.bet;
                self.record_highest_bet(self.bj.bet);
                self.push_note(format!("BJ: {}", crate::utils::chip_format::format_chips(self.bj.bet)));
            }
            KeyCode::Char('h') | KeyCode::Char('H') => self.bj.hit(),
            KeyCode::Char('s') | KeyCode::Char('S') => self.bj.stand(),
            KeyCode::Char('d') | KeyCode::Char('D') => {
                let extra = self.bj.double_down(self.data.player.chips);
                if extra > 0 {
                    self.data.player.chips = self.data.player.chips.saturating_sub(extra);
                    self.data.stats.total_bet += extra;
                    self.record_highest_bet(self.bj.bet + extra);
                }
            }
            KeyCode::Char('p') | KeyCode::Char('P') => {
                let extra = self.bj.split(self.data.player.chips);
                if extra > 0 {
                    self.data.player.chips = self.data.player.chips.saturating_sub(extra);
                    self.data.stats.total_bet += extra;
                    self.record_highest_bet(extra);
                }
            }
            KeyCode::Esc => {
                self.view = View::Dashboard;
                self.push_note("Dashboard");
            }
            _ => {}
        }
    }

    fn on_slots(&mut self, code: KeyCode) {
        match code {
            KeyCode::Left => self.slots.adjust_bet(-1_000, self.data.player.chips),
            KeyCode::Right => self.slots.adjust_bet(1_000, self.data.player.chips),
            KeyCode::Up => self.slots.adjust_bet(10_000, self.data.player.chips),
            KeyCode::Down => self.slots.adjust_bet(-10_000, self.data.player.chips),
            KeyCode::Char(' ') => {
                if self.slots.state.auto_spin_remaining > 0 {
                    self.slots.state.auto_spin_remaining = 0;
                    self.push_note("Auto stop");
                } else if self.slots.begin_spin(self.data.player.chips) {
                    // Deduct chips at spin start to prevent double-spending
                    let bet = self.slots.state.bet;
                    self.data.player.chips = self.data.player.chips.saturating_sub(bet);
                    self.data.stats.total_bet += bet;
                    self.record_highest_bet(bet);
                    self.push_note(format!("Spin @ {}", crate::utils::chip_format::format_chips(bet)));
                }
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                if self.slots.state.auto_spin_remaining > 0 {
                    self.slots.state.auto_spin_remaining = 0;
                    self.slots.state.auto_spin_total = 0;
                    self.push_note("Auto stop");
                } else {
                    self.slots.state.auto_spin_remaining = 10;
                    self.slots.state.auto_spin_total = 10;
                    self.push_note("Auto: 10");
                    if !self.slots.state.spinning && self.slots.begin_spin(self.data.player.chips) {
                        let bet = self.slots.state.bet;
                        self.data.player.chips = self.data.player.chips.saturating_sub(bet);
                        self.data.stats.total_bet += bet;
                        self.record_highest_bet(bet);
                        self.slots.state.auto_spin_remaining = self.slots.state.auto_spin_remaining.saturating_sub(1);
                    }
                }
            }
            KeyCode::Char('m') | KeyCode::Char('M') => {
                if self.slots.state.spinning || self.slots.state.auto_spin_remaining > 0 {
                    self.push_note("Stop spin/auto before machine swap");
                } else {
                    let next = match self.slots.state.machine {
                        MachineType::Classic => MachineType::Cyber,
                        MachineType::Cyber => MachineType::Retro,
                        MachineType::Retro => MachineType::Neon,
                        MachineType::Neon => MachineType::Hacker,
                        MachineType::Hacker => MachineType::Elite,
                        MachineType::Elite => MachineType::Midnight,
                        MachineType::Midnight => MachineType::DiamondRush,
                        MachineType::DiamondRush => MachineType::Lucky7,
                        MachineType::Lucky7 => MachineType::Inferno,
                        MachineType::Inferno => MachineType::Monochrome,
                        MachineType::Monochrome => MachineType::Classic,
                    };
                    self.slots.set_machine(next);
                    self.push_note(format!("Machine: {}", next.name()));
                }
            }
            KeyCode::Esc => {
                self.view = View::Dashboard;
                self.push_note("Dashboard");
            }
            _ => {}
        }
    }
}
