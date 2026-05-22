#[allow(dead_code)]
pub trait GameEngine: Send {
    fn name(&self) -> &'static str;
    fn tick(&mut self);
}
