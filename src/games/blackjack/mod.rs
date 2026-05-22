use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub rank: u8,
    pub suit: Suit,
}

impl Card {
    pub fn suit_char(&self) -> &'static str {
        match self.suit {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        }
    }
}

#[allow(dead_code)]
pub fn rank_char(r: u8) -> &'static str {
    match r {
        1 => "A",
        11 => "J",
        12 => "Q",
        13 => "K",
        n => ["2", "3", "4", "5", "6", "7", "8", "9", "10"][(n - 2) as usize],
    }
}

#[allow(dead_code)]
fn card_value(rank: u8) -> u8 {
    match rank {
        1 => 11,
        11..=13 => 10,
        n => n,
    }
}

pub fn hand_value(cards: &[Card]) -> u8 {
    let mut total = 0u8;
    let mut aces = 0u8;
    for c in cards {
        match c.rank {
            1 => { aces += 1; total += 11; }
            11..=13 => total += 10,
            n => total += n,
        }
    }
    while total > 21 && aces > 0 {
        total -= 10;
        aces -= 1;
    }
    total
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayState {
    Idle,
    #[allow(dead_code)]
    Dealing { phase: u8 },
    PlayerTurn,
    SplitTurn { hand_idx: usize },
    DealerTurn,
    RoundOver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Win,
    Lose,
    Push,
    Blackjack,
}

#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub bet: i64,
    pub done: bool,
    pub doubled: bool,
    pub outcome: Option<Outcome>,
}

impl Hand {
    pub fn new(bet: i64) -> Self {
        Self { cards: Vec::new(), bet, done: false, doubled: false, outcome: None }
    }

    pub fn value(&self) -> u8 { hand_value(&self.cards) }

    pub fn is_blackjack(&self) -> bool { self.cards.len() == 2 && self.value() == 21 }

    pub fn is_bust(&self) -> bool { self.value() > 21 }

    pub fn can_double(&self, chips: i64) -> bool {
        self.cards.len() == 2 && chips >= self.bet * 2 && !self.doubled
    }

    pub fn can_split(&self, chips: i64) -> bool {
        self.cards.len() == 2 && self.cards[0].rank == self.cards[1].rank && chips >= self.bet * 2
    }
}



#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AnimState {
    pub flash: u8,
    pub kind: Outcome,
}

#[derive(Debug, Clone)]
pub struct BlackjackEngine {
    pub state: PlayState,
    pub hands: Vec<Hand>,
    pub active: usize,
    pub dealer: Vec<Card>,
    pub bet: i64,
    pub message: String,
    pub reveal: bool,
    pub anim: Option<AnimState>,
    pub settled: bool,
    pub all_bust: bool,
    tick: u8,
    deck: Vec<Card>,
}

impl BlackjackEngine {
    pub fn new() -> Self {
        Self {
            state: PlayState::Idle,
            hands: Vec::new(),
            active: 0,
            dealer: Vec::new(),
            bet: 10_000,
            message: "Ready — [Space] Deal".to_string(),
            reveal: false,
            anim: None,
            settled: false,
            all_bust: false,
            tick: 0,
            deck: Self::fresh_deck(),
        }
    }

    fn fresh_deck() -> Vec<Card> {
        let mut d = Vec::with_capacity(52);
        for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in 1..=13 {
                d.push(Card { rank, suit });
            }
        }
        d.shuffle(&mut thread_rng());
        d
    }

    fn draw(&mut self) -> Card {
        if self.deck.is_empty() { self.deck = Self::fresh_deck(); }
        self.deck.pop().expect("deck not empty")
    }

    pub fn adjust_bet(&mut self, delta: i64, chips: i64) {
        let min = 1_000;
        let max = chips.min(10_000_000).max(min);
        self.bet = (self.bet + delta).clamp(min, max);
    }

    pub fn deal(&mut self, chips: i64) -> bool {
        if !matches!(self.state, PlayState::Idle | PlayState::RoundOver) { return false; }
        if self.bet > chips { self.message = "Not enough chips!".to_string(); return false; }
        let p1 = self.draw();
        let d1 = self.draw();
        let p2 = self.draw();
        let d2 = self.draw();
        self.hands = vec![Hand { cards: vec![p1, p2], bet: self.bet, done: false, doubled: false, outcome: None }];
        self.dealer = vec![d1, d2];
        self.active = 0;
        self.reveal = false;
        self.anim = None;
        self.settled = false;
        self.all_bust = false;
        self.tick = 0;
        let pv = self.hands[0].value();
        if pv == 21 {
            self.reveal = true;
            let dv = hand_value(&self.dealer);
            self.message = if dv == 21 { "Both Blackjack — Push".to_string() }
                else { "BLACKJACK!".to_string() };
            self.state = PlayState::RoundOver;
            self.evaluate();
        } else {
            let mut msg = format!("{}  [H] [S]", pv);
            if self.hands[0].can_double(chips) { msg.push_str(" [D]"); }
            if self.hands[0].can_split(chips) { msg.push_str(" [P]"); }
            self.message = msg;
            self.state = PlayState::PlayerTurn;
        }
        true
    }

    fn cur_idx(&self) -> usize {
        match self.state {
            PlayState::SplitTurn { hand_idx } => hand_idx,
            _ => self.active,
        }
    }

    pub fn hit(&mut self) {
        if !matches!(self.state, PlayState::PlayerTurn | PlayState::SplitTurn { .. }) { return; }
        let idx = self.cur_idx();
        if idx >= self.hands.len() || self.hands[idx].done { return; }
        let c = self.draw();
        self.hands[idx].cards.push(c);
        if self.hands[idx].is_bust() {
            self.hands[idx].done = true;
            self.message = format!("Hand {} bust!", idx + 1);
            self.advance();
        } else {
            self.message = format!("Hand {}: {}  [H] [S]", idx + 1, self.hands[idx].value());
        }
    }

    pub fn stand(&mut self) {
        if !matches!(self.state, PlayState::PlayerTurn | PlayState::SplitTurn { .. }) { return; }
        let idx = self.cur_idx();
        if idx >= self.hands.len() || self.hands[idx].done { return; }
        self.hands[idx].done = true;
        self.message = format!("Hand {} stand at {}", idx + 1, self.hands[idx].value());
        self.advance();
    }

    pub fn double_down(&mut self, chips: i64) -> i64 {
        if !matches!(self.state, PlayState::PlayerTurn | PlayState::SplitTurn { .. }) { return 0; }
        let idx = self.cur_idx();
        if idx >= self.hands.len() { return 0; }
        if !self.hands[idx].can_double(chips) { return 0; }
        let c = self.draw();
        let h = &mut self.hands[idx];
        let extra = h.bet;
        h.bet *= 2;
        h.doubled = true;
        h.cards.push(c);
        h.done = true;
        self.message = if h.is_bust() { "Double bust!".to_string() }
            else { format!("Double stand at {}", h.value()) };
        self.advance();
        extra
    }

    pub fn split(&mut self, chips: i64) -> i64 {
        if self.state != PlayState::PlayerTurn { return 0; }
        if self.active >= self.hands.len() { return 0; }
        if !self.hands[self.active].can_split(chips) { return 0; }
        let card = self.hands[self.active].cards[1];
        let bet = self.hands[self.active].bet;
        self.hands[self.active].cards.truncate(1);
        let c1 = self.draw();
        self.hands[self.active].cards.push(c1);
        let mut nh = Hand::new(bet);
        nh.cards.push(card);
        let c2 = self.draw();
        nh.cards.push(c2);
        self.hands.push(nh);
        self.state = PlayState::SplitTurn { hand_idx: 0 };
        self.message = format!("Hand 1: {}  [H] [S] [D]", self.hands[0].value());
        bet
    }

    fn advance(&mut self) {
        loop {
            self.active += 1;
            if self.active >= self.hands.len() {
                if self.hands.iter().all(|h| h.is_bust()) {
                    self.all_bust = true;
                    self.message = "All bust!".to_string();
                    self.end_round();
                    return;
                }
                self.enter_dealer();
                return;
            }
            if !self.hands[self.active].done {
                self.state = PlayState::SplitTurn { hand_idx: self.active };
                self.message = format!("Hand {}: {}  [H] [S] [D]", self.active + 1, self.hands[self.active].value());
                return;
            }
        }
    }

    fn enter_dealer(&mut self) {
        self.state = PlayState::DealerTurn;
        self.reveal = true;
        self.tick = 0;
        let dv = hand_value(&self.dealer);
        if dv >= 17 && !(dv == 17 && is_soft(&self.dealer)) {
            self.finish_round();
            return;
        }
        self.message = "House draws...".to_string();
    }

    fn finish_round(&mut self) {
        self.state = PlayState::RoundOver;
        let dv = hand_value(&self.dealer);
        if dv > 21 {
            self.message = "House busts!".to_string();
        } else {
            self.message = format!("House: {}", dv);
        }
        self.evaluate();
    }

    fn evaluate(&mut self) {
        let dv = hand_value(&self.dealer);
        let dealer_bj = dv == 21 && self.dealer.len() == 2;
        for h in &mut self.hands {
            let bj = h.is_blackjack();
            h.outcome = Some(if bj && dealer_bj { Outcome::Push }
                else if bj { Outcome::Blackjack }
                else if h.is_bust() || dealer_bj { Outcome::Lose }
                else if dv > 21 || h.value() > dv { Outcome::Win }
                else if h.value() == dv { Outcome::Push }
                else { Outcome::Lose });
        }
    }

    pub fn settle(&mut self) -> Vec<(Outcome, i64)> {
        let mut out = Vec::new();
        for h in &self.hands {
            let delta = match h.outcome.unwrap_or(Outcome::Lose) {
                Outcome::Win => h.bet,
                Outcome::Lose => -h.bet,
                Outcome::Push => 0,
                Outcome::Blackjack => h.bet + h.bet / 2,
            };
            out.push((h.outcome.unwrap_or(Outcome::Lose), delta));
        }
        out
    }

    fn end_round(&mut self) {
        self.state = PlayState::RoundOver;
        self.reveal = true;
        self.evaluate();
    }

    pub fn tick(&mut self) {
        match self.state {
            PlayState::DealerTurn => {
                self.tick += 1;
                if self.tick >= crate::core::BJ_DEALER_TICK_DELAY {
                    let dv = hand_value(&self.dealer);
                    if dv > 21 { self.finish_round(); return; }
                    if dv >= 17 && !(dv == 17 && is_soft(&self.dealer)) {
                        self.finish_round();
                        return;
                    }
                    let c = self.draw();
                    self.dealer.push(c);
                    self.message = format!("House draws... {}", hand_value(&self.dealer));
                    self.tick = 0;
                }
            }
            PlayState::RoundOver => {
                if let Some(ref anim) = self.anim {
                    if anim.flash > 0 {
                        let a = self.anim.as_mut().unwrap();
                        a.flash -= 1;
                        if a.flash == 0 { self.anim = None; }
                    }
                }
            }
            _ => {}
        }
    }
}

fn is_soft(cards: &[Card]) -> bool {
    let mut aces = 0u8;
    let mut total = 0u8;
    for c in cards {
        match c.rank {
            1 => { aces += 1; total += 11; }
            11..=13 => total += 10,
            n => total += n,
        }
    }
    while total > 21 && aces > 0 { total -= 10; aces -= 1; }
    aces > 0 && total <= 21
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(rank: u8, suit: Suit) -> Card {
        Card { rank, suit }
    }

    #[test]
    fn initial_deal_gives_two_cards_to_player_and_dealer() {
        let mut game = BlackjackEngine::new();
        game.bet = 10_000;
        game.deck = vec![
            card(10, Suit::Spades),
            card(8, Suit::Clubs),
            card(7, Suit::Hearts),
            card(9, Suit::Diamonds),
        ];

        assert!(game.deal(100_000));
        assert_eq!(game.hands.len(), 1);
        assert_eq!(game.hands[0].cards.len(), 2);
        assert_eq!(game.dealer.len(), 2);
        assert_eq!(game.hands[0].cards[0].rank, 9);
        assert_eq!(game.hands[0].cards[1].rank, 8);
        assert_eq!(game.dealer[0].rank, 7);
        assert_eq!(game.dealer[1].rank, 10);
    }

    #[test]
    fn split_sets_split_turn_instead_of_returning_early() {
        let mut game = BlackjackEngine::new();
        game.bet = 10_000;
        game.state = PlayState::PlayerTurn;
        game.active = 0;
        game.hands = vec![Hand {
            cards: vec![card(8, Suit::Hearts), card(8, Suit::Spades)],
            bet: 10_000,
            done: false,
            doubled: false,
            outcome: None,
        }];
        game.deck = vec![
            card(6, Suit::Clubs),
            card(5, Suit::Diamonds),
        ];

        let extra = game.split(100_000);

        assert_eq!(extra, 10_000);
        assert_eq!(game.hands.len(), 2);
        assert_eq!(game.state, PlayState::SplitTurn { hand_idx: 0 });
        assert_eq!(game.hands[0].cards.len(), 2);
        assert_eq!(game.hands[1].cards.len(), 2);
    }

    #[test]
    fn double_down_adds_one_card_and_returns_extra_bet() {
        let mut game = BlackjackEngine::new();
        game.state = PlayState::PlayerTurn;
        game.active = 0;
        game.hands = vec![Hand {
            cards: vec![card(5, Suit::Hearts), card(6, Suit::Spades)],
            bet: 10_000,
            done: false,
            doubled: false,
            outcome: None,
        }];
        game.dealer = vec![card(10, Suit::Diamonds), card(9, Suit::Clubs)];
        game.deck = vec![card(10, Suit::Hearts)];

        let extra = game.double_down(100_000);

        assert_eq!(extra, 10_000);
        assert_eq!(game.hands[0].cards.len(), 3);
        assert!(game.hands[0].doubled);
    }
}
