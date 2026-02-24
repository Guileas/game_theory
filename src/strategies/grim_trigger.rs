use crate::{Strategy, strategy::Action};

pub struct GrimTrigger;

impl GrimTrigger {
    pub fn new() -> Self {
        Self
    }
}

impl Strategy for GrimTrigger {
    fn name(&self) -> &str {
        std::any::type_name::<GrimTrigger>()
    }

    fn next_move(&mut self, opponent_history: &[Action]) -> Action {

        let mut is_desefect:bool = false;
        if opponent_history.contains(&Action::Defect) {
            is_desefect = true
        }

        match is_desefect {
            true => Action::Defect,
            false => Action::Cooperate
        }

    }
}