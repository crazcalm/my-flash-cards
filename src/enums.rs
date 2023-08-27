use std::fmt::Display;

#[derive(Debug, PartialEq, PartialOrd, Default)]
pub enum FlashCardState {
    #[default]
    Front,
    Back,
    Hint,
}

impl Display for FlashCardState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlashCardState::Back => write!(f, "back"),
            FlashCardState::Hint => write!(f, "hint"),
            FlashCardState::Front => write!(f, "front"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_state() {
        assert_eq!("back".to_string(), FlashCardState::Back.to_string());
        assert_eq!("hint".to_string(), FlashCardState::Hint.to_string());
        assert_eq!("front".to_string(), FlashCardState::Front.to_string());
    }
}
