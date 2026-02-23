use crate::{Strategy, strategy::Action};

pub struct TitForTat;
// tit for tat : coopère au premier tour pour copier le dernier coup de l'adversaire
impl TitForTat {
    pub fn new() -> Self {
        Self
    }
}

impl Strategy for TitForTat {
    fn name(&self) -> &str {
        "Tit For Tat"
    }

    fn next_move(&mut self, opponent_history: &[Action]) -> Action {
        match opponent_history.last() {
            None => Action::Cooperate,
            Some(Action::Cooperate) => Action::Cooperate,
            Some(Action::Defect) => Action::Defect,
        }
    }
}
