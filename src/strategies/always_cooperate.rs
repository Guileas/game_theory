use crate::{strategy::Action, Strategy};

pub struct AlwaysCooperate;

impl AlwaysCooperate {
    pub fn new() -> Self {
        Self
    }
}

impl Strategy for AlwaysCooperate {
    fn name(&self) -> &str {
        "always_cooperate"
    }

    fn next_move(&mut self, _opponent_history: &[Action]) -> Action {
        Action::Cooperate
    }
}
