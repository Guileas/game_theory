use crate::{Strategy, strategy::Action};

pub struct AlwaysDefect;

impl AlwaysDefect {
    pub fn new() -> Self {
        Self
    }
}

impl Strategy for AlwaysDefect {
    fn name(&self) -> &str {
        "Always Defect"
    }
    fn next_move(&mut self, _opponent_history: &[Action]) -> Action {
        Action::Defect
    }
}
