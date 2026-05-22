use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SlotSymbol {
    Cherry,
    Lemon,
    Bell,
    Seven,
    Diamond,
    Wild,
    Scatter,
    Crown,
    Flame,
    Cyber,
    // Classic Vegas & Expansion
    Bar,
    DoubleBar,
    TripleBar,
    Horseshoe,
    Coin,
    Heart,
    Star,
    GoldBar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

impl SlotSymbol {
    pub fn is_wild(self) -> bool {
        matches!(self, SlotSymbol::Wild)
    }

    pub fn is_scatter(self) -> bool {
        matches!(self, SlotSymbol::Scatter)
    }

    pub fn rarity(self) -> Rarity {
        match self {
            SlotSymbol::Cherry | SlotSymbol::Lemon | SlotSymbol::Bell | SlotSymbol::Heart | SlotSymbol::Coin => Rarity::Common,
            SlotSymbol::Flame | SlotSymbol::Cyber | SlotSymbol::Star | SlotSymbol::Bar => Rarity::Rare,
            SlotSymbol::Seven | SlotSymbol::DoubleBar | SlotSymbol::Horseshoe | SlotSymbol::Crown => Rarity::Epic,
            SlotSymbol::Diamond | SlotSymbol::TripleBar | SlotSymbol::GoldBar | SlotSymbol::Wild | SlotSymbol::Scatter => Rarity::Legendary,
        }
    }

    pub fn big_symbol(self) -> [&'static str; 5] {
        match self {
            SlotSymbol::Cherry => [
                "  ▄▄ ▄▄ ",
                " ██████ ",
                " ▀████▀ ",
                "   ▀▀   ",
                "   🍒   ",
            ],
            SlotSymbol::Lemon => [
                "   ▄██▄ ",
                "  ██████",
                "  ▀████▀",
                "   ▀██▀ ",
                "   🍋   ",
            ],
            SlotSymbol::Bell => [
                "   ▄▄   ",
                "  ████  ",
                " ██████ ",
                " ▀████▀ ",
                "   🔔   ",
            ],
            SlotSymbol::Seven => [
                " ▀▀▀▀▀██",
                "     ██ ",
                "    ██  ",
                "   ██   ",
                "  ██    ",
            ],
            SlotSymbol::Diamond => [
                "   ▄▄   ",
                " ▄████▄ ",
                " ██████ ",
                " ▀████▀ ",
                "   💎   ",
            ],
            SlotSymbol::Wild => [
                "   ⭐   ",
                "  ⭐⭐  ",
                " ⭐⭐⭐ ",
                "  ⭐⭐  ",
                "   ⭐   ",
            ],
            SlotSymbol::Scatter => [
                "  🎰🎰  ",
                " 🎰🎰🎰 ",
                " 🎰🎰🎰 ",
                "  🎰🎰  ",
                "   🎰   ",
            ],
            SlotSymbol::Crown => [
                " 👑👑👑 ",
                " ██████ ",
                " ██████ ",
                "  ▀▀▀▀  ",
                "   👑   ",
            ],
            SlotSymbol::Flame => [
                "   🔥   ",
                "  🔥🔥  ",
                " 🔥🔥🔥 ",
                "  🔥🔥  ",
                "   🔥   ",
            ],
            SlotSymbol::Cyber => [
                "  💠💠  ",
                " 💠💠💠 ",
                " 💠💠💠 ",
                "  💠💠  ",
                "   💠   ",
            ],
            SlotSymbol::Bar => [
                " ▄████▄ ",
                " ██████ ",
                " ██████ ",
                " ▀████▀ ",
                "  BAR   ",
            ],
            SlotSymbol::DoubleBar => [
                " ██████ ",
                "  BAR   ",
                "        ",
                " ██████ ",
                "  BAR   ",
            ],
            SlotSymbol::TripleBar => [
                " ██████ ",
                "  BAR   ",
                " ██████ ",
                "  BAR   ",
                " ██████ ",
            ],
            SlotSymbol::Horseshoe => [
                "  ▄██▄  ",
                " ██  ██ ",
                " ██  ██ ",
                " ▀█  █▀ ",
                "   🧲   ",
            ],
            SlotSymbol::Coin => [
                "   ▄▄   ",
                "  ████  ",
                "  ████  ",
                "   ▀▀   ",
                "   🪙   ",
            ],
            SlotSymbol::Heart => [
                "  ▄▄ ▄▄ ",
                " ██████ ",
                "  ████  ",
                "   ██   ",
                "   ❤️   ",
            ],
            SlotSymbol::Star => [
                "   ██   ",
                " ██████ ",
                "  ████  ",
                " ██  ██ ",
                "   ⭐   ",
            ],
            SlotSymbol::GoldBar => [
                "        ",
                " ▄████▄ ",
                " ██████ ",
                " ▀████▀ ",
                "  GOLD  ",
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineType {
    Classic,
    Cyber,
    Retro,
    Neon,
    Hacker,
    Elite,
    Midnight,
    DiamondRush,
    Lucky7,
    Inferno,
    Monochrome,
}

impl MachineType {
    pub fn name(&self) -> &str {
        match self {
            MachineType::Classic => "Classic Vegas",
            MachineType::Cyber => "Cyber Slots",
            MachineType::Retro => "Retro Casino",
            MachineType::Neon => "Neon Jackpot",
            MachineType::Hacker => "Hacker Machine",
            MachineType::Elite => "Elite VIP",
            MachineType::Midnight => "Midnight Machine",
            MachineType::DiamondRush => "Diamond Rush",
            MachineType::Lucky7 => "Lucky 7 Machine",
            MachineType::Inferno => "Inferno Slots",
            MachineType::Monochrome => "Monochrome",
        }
    }

    pub fn symbol_pool(self) -> &'static [(SlotSymbol, u32, i64)] {
        match self {
            MachineType::Classic => &[
                (SlotSymbol::Cherry, 40, 3),
                (SlotSymbol::Lemon, 30, 4),
                (SlotSymbol::Bell, 18, 7),
                (SlotSymbol::Seven, 8, 12),
                (SlotSymbol::Diamond, 4, 25),
            ],
            MachineType::Cyber => &[
                (SlotSymbol::Cherry, 30, 2),
                (SlotSymbol::Lemon, 25, 3),
                (SlotSymbol::Cyber, 20, 5),
                (SlotSymbol::Seven, 10, 10),
                (SlotSymbol::Wild, 8, 0),
                (SlotSymbol::Diamond, 5, 30),
                (SlotSymbol::Scatter, 2, 0),
            ],
            MachineType::Retro => &[
                (SlotSymbol::Cherry, 35, 3),
                (SlotSymbol::Lemon, 25, 5),
                (SlotSymbol::Bell, 15, 8),
                (SlotSymbol::Crown, 10, 15),
                (SlotSymbol::Seven, 8, 20),
                (SlotSymbol::Diamond, 5, 50),
                (SlotSymbol::Wild, 2, 0),
            ],
            MachineType::Neon => &[
                (SlotSymbol::Cherry, 35, 2),
                (SlotSymbol::Flame, 25, 4),
                (SlotSymbol::Bell, 15, 6),
                (SlotSymbol::Seven, 10, 10),
                (SlotSymbol::Diamond, 8, 20),
                (SlotSymbol::Wild, 5, 0),
                (SlotSymbol::Scatter, 2, 0),
            ],
            MachineType::Hacker => &[
                (SlotSymbol::Cyber, 30, 3),
                (SlotSymbol::Flame, 20, 5),
                (SlotSymbol::Seven, 15, 8),
                (SlotSymbol::Crown, 10, 12),
                (SlotSymbol::Diamond, 8, 25),
                (SlotSymbol::Wild, 7, 0),
                (SlotSymbol::Scatter, 10, 0),
            ],
            MachineType::Elite => &[
                (SlotSymbol::Diamond, 35, 15),
                (SlotSymbol::Crown, 25, 20),
                (SlotSymbol::Seven, 15, 30),
                (SlotSymbol::Wild, 15, 0),
                (SlotSymbol::Scatter, 10, 0),
            ],
            MachineType::Midnight => &[
                (SlotSymbol::Star, 35, 4),
                (SlotSymbol::Cyber, 25, 6),
                (SlotSymbol::DoubleBar, 15, 10),
                (SlotSymbol::TripleBar, 10, 20),
                (SlotSymbol::Wild, 10, 0),
                (SlotSymbol::Scatter, 5, 0),
            ],
            MachineType::DiamondRush => &[
                (SlotSymbol::Diamond, 50, 10),
                (SlotSymbol::GoldBar, 20, 15),
                (SlotSymbol::Crown, 15, 20),
                (SlotSymbol::Wild, 10, 0),
                (SlotSymbol::Scatter, 5, 0),
            ],
            MachineType::Lucky7 => &[
                (SlotSymbol::Seven, 40, 15),
                (SlotSymbol::Cherry, 30, 2),
                (SlotSymbol::Horseshoe, 15, 10),
                (SlotSymbol::Wild, 10, 0),
                (SlotSymbol::Scatter, 5, 0),
            ],
            MachineType::Inferno => &[
                (SlotSymbol::Flame, 40, 5),
                (SlotSymbol::Seven, 20, 15),
                (SlotSymbol::TripleBar, 15, 20),
                (SlotSymbol::Heart, 10, 10),
                (SlotSymbol::Wild, 10, 0),
                (SlotSymbol::Scatter, 5, 0),
            ],
            MachineType::Monochrome => &[
                (SlotSymbol::Bar, 40, 3),
                (SlotSymbol::DoubleBar, 30, 6),
                (SlotSymbol::TripleBar, 15, 12),
                (SlotSymbol::Diamond, 10, 25),
                (SlotSymbol::Scatter, 5, 0),
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub struct JackpotPool {
    pub mini: i64,
    pub mega: i64,
    pub ultra: i64,
}

impl JackpotPool {
    pub fn new() -> Self {
        Self {
            mini: 50_000,
            mega: 500_000,
            ultra: 5_000_000,
        }
    }

    pub fn contribute(&mut self, bet: i64) {
        let contrib = bet / 20;
        self.mini += contrib;
        self.mega += contrib / 2;
        self.ultra += contrib / 4;
    }
}

#[derive(Debug, Clone)]
pub struct SlotsState {
    pub bet: i64,
    pub reels: Vec<Vec<SlotSymbol>>, // 2D grid: [column][row]
    pub reel_count: usize,
    pub visible_rows: usize, // e.g. 3
    pub machine: MachineType,
    pub spinning: bool,
    pub spin_frames_left: u8,
    pub reel_spin_frames: Vec<u8>, // Individual spin frames for each reel
    pub message: String,
    pub last_payout: i64,
    pub last_mult: i64,
    pub flash_counter: u8,
    pub jackpots: JackpotPool,
    pub auto_spin_remaining: u32,
    pub auto_spin_total: u32,
    pub display_chips: i64,
}

impl Default for SlotsState {
    fn default() -> Self {
        Self {
            bet: 5_000,
            reels: vec![
                vec![SlotSymbol::Cherry, SlotSymbol::Cherry, SlotSymbol::Cherry],
                vec![SlotSymbol::Lemon, SlotSymbol::Lemon, SlotSymbol::Lemon],
                vec![SlotSymbol::Bell, SlotSymbol::Bell, SlotSymbol::Bell]
            ],
            reel_count: 3,
            visible_rows: 3,
            machine: MachineType::Classic,
            spinning: false,
            spin_frames_left: 0,
            reel_spin_frames: vec![0, 0, 0], // Initialize with zeros
            message: "Adjust bet and press [Space] to spin".to_string(),
            last_payout: 0,
            last_mult: 0,
            flash_counter: 0,
            jackpots: JackpotPool::new(),
            auto_spin_remaining: 0,
            auto_spin_total: 0,
            display_chips: 100_000,
        }
    }
}

#[derive(Default)]
pub struct SlotsGame {
    pub state: SlotsState,
}

impl SlotsGame {
    pub fn adjust_bet(&mut self, delta: i64, max_chips: i64) {
        if self.state.spinning || self.state.auto_spin_remaining > 0 {
            return;
        }
        let max = max_chips.clamp(1_000, 10_000_000);
        self.state.bet = (self.state.bet + delta).clamp(1_000, max);
    }

    pub fn set_machine(&mut self, machine: MachineType) {
        if self.state.spinning || self.state.auto_spin_remaining > 0 {
            return;
        }
        self.state.machine = machine;
        self.state.reel_count = if matches!(machine, MachineType::Cyber | MachineType::Hacker | MachineType::Elite) {
            5
        } else {
            3
        };
        self.state.reels = vec![
            vec![self.state.machine.symbol_pool()[0].0; self.state.visible_rows];
            self.state.reel_count
        ];
        // Initialize reel spin frames for the new reel count
        self.state.reel_spin_frames = vec![0; self.state.reel_count];
    }

    pub fn begin_spin(&mut self, chips: i64) -> bool {
        if self.state.spinning {
            return false;
        }
        if self.state.bet > chips {
            self.state.message = "Insufficient chips for bet".to_string();
            return false;
        }
        self.state.spinning = true;
        self.state.spin_frames_left = crate::core::SLOTS_SPIN_FRAMES;
        // Initialize individual reel spin frames with staggered start times
        self.state.reel_spin_frames = (0..self.state.reel_count)
            .map(|i| crate::core::SLOTS_SPIN_FRAMES + ((i as u8) * crate::core::SLOTS_REEL_STAGGER))
            .collect();
        self.state.message = "Spinning...".to_string();
        self.state.flash_counter = 0;
        self.state.last_mult = 0;
        self.state.last_payout = 0;
        self.state.jackpots.contribute(self.state.bet);
        true
    }

    pub fn tick_spin(&mut self) -> Option<(i64, i64, bool)> {
        if !self.state.spinning {
            return None;
        }

        // Decrease global spin frames for animation
        if self.state.spin_frames_left > 0 {
            self.state.spin_frames_left = self.state.spin_frames_left.saturating_sub(1);
        }
        
        // Decrease individual reel spin frames
        let mut all_done = true;
        for frame in &mut self.state.reel_spin_frames {
            if *frame > 0 {
                *frame -= 1;
                all_done = false;
            }
        }
        
        // If all reels have finished spinning, determine the outcome
        if all_done {
            self.state.spinning = false;

            let pool = self.state.machine.symbol_pool();
            let mut reels = Vec::with_capacity(self.state.reel_count);
            for _ in 0..self.state.reel_count {
                let mut column = Vec::with_capacity(self.state.visible_rows);
                for _ in 0..self.state.visible_rows {
                    column.push(draw_symbol_weighted(pool));
                }
                reels.push(column);
            }
            self.state.reels = reels;

            let mut has_wild = false;
            let mut scatter_count = 0;
            for col in &self.state.reels {
                for s in col {
                    if s.is_wild() { has_wild = true; }
                    if s.is_scatter() { scatter_count += 1; }
                }
            }

            let mult = calculate_multiplier(&self.state.reels, pool);
            let mut payout = self.state.bet * mult;

            if has_wild && mult > 0 {
                let wild_bonus = mult / 2;
                payout += wild_bonus;
            }

            self.state.last_payout = payout;
            self.state.last_mult = mult;
            self.state.flash_counter = if mult >= 50 { 40 } else if mult >= 20 { 28 } else { 16 };

            if mult >= 100 {
                self.state.jackpots.ultra = (self.state.jackpots.ultra as f64 * 0.5) as i64 + 5_000_000;
                self.state.message = format!("★★★★★ ULTRA JACKPOT x{mult}! ★★★★★");
            } else if mult >= 50 {
                self.state.jackpots.mega = (self.state.jackpots.mega as f64 * 0.5) as i64 + 500_000;
                self.state.message = format!("★★★★ MEGA JACKPOT x{mult}! ★★★★");
            } else if mult >= 20 {
                self.state.message = format!("★★★ JACKPOT x{mult}! ★★★");
            } else if mult >= 10 {
                self.state.message = format!("★ BIG WIN x{mult} ★");
            } else if mult > 0 {
                self.state.message = format!("Win x{mult}!");
            } else if scatter_count >= 3 {
                self.state.message = "SCATTER BONUS! +3x payout!".to_string();
                payout = self.state.bet * 3;
            } else {
                self.state.message = "No match. Better luck next spin.".to_string();
            }

            Some((payout, mult, scatter_count >= 3))
        } else {
            // Still spinning - generate intermediate reel states
            let pool = self.state.machine.symbol_pool();
            let mut reels = Vec::with_capacity(self.state.reel_count);
            for i in 0..self.state.reel_count {
                // Only spin reels that haven't finished their countdown
                if self.state.reel_spin_frames[i] > 0 {
                    let mut column = Vec::with_capacity(self.state.visible_rows);
                    for _ in 0..self.state.visible_rows {
                        column.push(draw_symbol_weighted(pool));
                    }
                    reels.push(column);
                } else {
                    // Keep the last spun symbol for finished reels
                    if !self.state.reels.is_empty() && i < self.state.reels.len() {
                        reels.push(self.state.reels[i].clone());
                    } else {
                        let mut column = Vec::with_capacity(self.state.visible_rows);
                        for _ in 0..self.state.visible_rows {
                            column.push(draw_symbol_weighted(pool));
                        }
                        reels.push(column);
                    }
                }
            }
            self.state.reels = reels;
            None
        }
    }

    pub fn tick_animation(&mut self, target_chips: i64) {
        if self.state.flash_counter > 0 {
            self.state.flash_counter -= 1;
        }

        // Chip count-up animation
        if self.state.display_chips < target_chips {
            let diff = target_chips - self.state.display_chips;
            let step = (diff / 10).max(1);
            self.state.display_chips += step;
        } else if self.state.display_chips > target_chips {
            let diff = self.state.display_chips - target_chips;
            let step = (diff / 10).max(1);
            self.state.display_chips -= step;
        }
    }
}

fn draw_symbol_weighted(pool: &[(SlotSymbol, u32, i64)]) -> SlotSymbol {
    let total_weight: u32 = pool.iter().map(|s| s.1).sum();
    let mut pick = thread_rng().gen_range(0..total_weight);
    for (symbol, weight, _) in pool {
        if pick < *weight {
            return *symbol;
        }
        pick -= weight;
    }
    pool[0].0
}

fn calculate_multiplier(reels: &[Vec<SlotSymbol>], pool: &[(SlotSymbol, u32, i64)]) -> i64 {
    let mut total_mult = 0;
    let reel_count = reels.len();
    if reel_count == 0 { return 0; }
    let visible_rows = reels[0].len();

    // Define paylines based on rows
    // For 3 visible rows: 
    // Line 0: middle row
    // Line 1: top row
    // Line 2: bottom row
    // Line 3: diagonal top-left to bottom-right
    // Line 4: diagonal bottom-left to top-right
    
    let mut paylines: Vec<Vec<SlotSymbol>> = Vec::new();
    
    // Horizontal lines
    for r in 0..visible_rows {
        let mut line = Vec::new();
        for col in 0..reel_count {
            line.push(reels[col][r]);
        }
        paylines.push(line);
    }
    
    // Diagonal lines (only if visible_rows >= 3)
    if visible_rows >= 3 {
        // Diagonal 1
        let mut d1 = Vec::new();
        for col in 0..reel_count {
            let r = col % visible_rows;
            d1.push(reels[col][r]);
        }
        paylines.push(d1);
        
        // Diagonal 2
        let mut d2 = Vec::new();
        for col in 0..reel_count {
            let r = (visible_rows - 1) - (col % visible_rows);
            d2.push(reels[col][r]);
        }
        paylines.push(d2);
    }

    for line in paylines {
        total_mult += evaluate_line(&line, pool);
    }

    total_mult
}

fn evaluate_line(line: &[SlotSymbol], pool: &[(SlotSymbol, u32, i64)]) -> i64 {
    let mut counts = std::collections::HashMap::new();
    let mut wild_count = 0;

    for s in line {
        if s.is_wild() {
            wild_count += 1;
        } else {
            *counts.entry(*s).or_insert(0) += 1;
        }
    }

    let n = line.len();

    if wild_count == n {
        return if n >= 5 { 60 } else { 30 };
    }

    if let Some((sym, count)) = counts.iter().max_by_key(|(_, c)| **c) {
        let effective = count + wild_count;
        if effective >= n {
            for (symbol, _, payout) in pool {
                if *symbol == *sym {
                    return if n >= 5 { *payout } else { *payout * 2 };
                }
            }
            return if n >= 5 { 12 } else { 8 };
        }
        if effective >= n.saturating_sub(1) && n >= 4 {
            for (symbol, _, payout) in pool {
                if *symbol == *sym {
                    return (*payout / 2).max(4);
                }
            }
            return 4;
        }
        if n == 3 && effective >= 3 {
            return 3;
        }
    }

    0
}
