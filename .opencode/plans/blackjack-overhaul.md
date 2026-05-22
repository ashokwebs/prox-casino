# Blackjack Overhaul & Dashboard Redesign Plan

## Goals
1. Fix split/double (chip check bug + 'd' key conflict)
2. Remove surrender
3. Add tick-based dealing/dealer animations
4. Hide dealer cards when player busts all hands
5. Big hand value display
6. Professional dashboard redesign

## File-by-File Changes

### A. src/games/blackjack/mod.rs

**Remove surrender:**
- Delete `RoundOutcome::Surrendered` enum variant
- Delete `can_surrender: bool`, `surrendered: bool` from `BlackjackState`
- Delete `surrender()` method
- Remove all `self.state.can_surrender = true/false` assignments
- Remove surrendered branch in `evaluate_results()`

**Fix split/double signatures:**
- `double_down(&mut self) -> bool` → `double_down(&mut self, chips: i64) -> bool`
- `split(&mut self) -> bool` → `split(&mut self, chips: i64) -> bool`
- Both pass `chips` to `can_double(chips)` / `can_split(chips)`

**Add `RoundState::Dealing` — deal animation:**
- `start_round`: draw 4 cards into `deal_queue: Vec<Card>`, set state to `Dealing`
- `tick()` handles `Dealing`: every 4 ticks, pop one card from `deal_queue`
- Deal order: player card 1 → dealer card 1 → player card 2 → dealer card 2
- After queue empty: check BJ → insurance → PlayerTurn

**Dealer play animation:**
- `enter_dealer_turn()`: pre-compute draws into `dealer_queue`, set `DealerTurn`
- `tick_dealer_turn()`: every 6 ticks, pop one card from `dealer_queue`
- When queue empty → `finish_dealer_turn()` → `RoundOver`

**All-bust: dealer cards stay hidden:**
- In `advance_after_hand()`: if all bust, set `all_bust=true`, go to `RoundOver` without `enter_dealer_turn()`

**New fields:**
- `BlackjackGame`: `deal_queue: Vec<Card>`, `dealer_queue: Vec<Card>`, `deal_frame: u8`
- `BlackjackState`: `dealer_bust: bool`, `all_bust: bool`

### B. src/app/mod.rs

**Fix 'd' key:**
```rust
KeyCode::Char('d') => {
    if self.view == View::Blackjack {
        self.handle_view_keys(code)?;
    } else if self.mode == Mode::Offline {
        // daily bonus
    }
}
```

**Pass chips to split/double, remove surrender.**

### C. src/ui/screens/blackjack.rs

**Big hand value display:** double-bordered box with large value text.
**Controls:** remove [Z], clean layout.
**Dealing animation:** show revealed cards incrementally.
**All-bust:** hide dealer hole card.

### D. src/ui/screens/dashboard.rs

**Cleaner ASCII logo** with box-drawn "PROX CASINO" in red.
**Balanced two-column layout**, professional spacing.
