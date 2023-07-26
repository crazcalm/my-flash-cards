use std::fmt::Display;

use serde::Deserialize;

use crate::{FlashCard, FlipFlashCard, FlashCardState};

#[derive(Deserialize, Debug)]
pub struct Card {
    #[serde(skip)]
    state: FlashCardState,
    front: String,
    back: String,
    hint: Option<String>,
}

impl Card {
    pub fn new(front: String, back: String, hint: String) -> Self {
        Card {
            front,
            back,
            hint: Some(hint),
            state: FlashCardState::Front,
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.state {
            FlashCardState::Front => write!(f, "{}", self.get_front()),
            FlashCardState::Back => write!(f, "{}", self.get_back()),
            FlashCardState::Hint => write!(f, "{}", self.get_hint()),
        }
    }
}

impl FlashCard<'_> for Card {
    fn get_front(&self) -> &str {
        &self.front
    }
    fn get_back(&self) -> &str {
        &self.back
    }
    fn get_hint(&self) -> &str {
        match &self.hint {
            Some(hint) => &hint,
            None => "No Hint Found",
        }
    }
}

impl FlipFlashCard for Card {
    fn flip(&mut self) -> &FlashCardState {
        let result = match self.state {
            FlashCardState::Front => FlashCardState::Back,
            FlashCardState::Back => FlashCardState::Front,
            FlashCardState::Hint => FlashCardState::Front,
        };

        self.state = result;
        &self.state
    }
    fn get_state(&self) -> &FlashCardState {
        &self.state
    }

    fn set_state(&mut self, state: FlashCardState) {
        self.state = state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

   #[test]
    fn test_new_card() {
        let front = "front".to_string();
        let back = "back".to_string();
        let hint = "hint".to_string();

        let card = Card::new(front.clone(), back.clone(), hint.clone());

        assert_eq!(card.get_front(), &front);
        assert_eq!(card.get_back(), &back);
        assert_eq!(card.get_hint(), &hint);
        assert_eq!(card.get_state(), &FlashCardState::Front);
    }

    #[test]
    fn test_flip() {
        let front = "front".to_string();
        let back = "back".to_string();
        let hint = "hint".to_string();

        let mut card = Card::new(front.clone(), back.clone(), hint.clone());

        // Test Back
        card.flip();
        assert_eq!(card.get_state(), &FlashCardState::Back);

        // Test Front
        card.flip();
        assert_eq!(card.get_state(), &FlashCardState::Front);

        // Test Hint
        card.set_state(FlashCardState::Hint);
        card.flip();
        assert_eq!(card.get_state(), &FlashCardState::Front);
    }

    #[test]
    fn test_display_and_state_change() {
        let front = "front".to_string();
        let back = "back".to_string();
        let hint = "hint".to_string();

        let mut card = Card::new(front.clone(), back.clone(), hint.clone());

        // Test front
        assert_eq!(format!("{}", card), front);

        // Test back
        card.set_state(FlashCardState::Back);
        assert_eq!(format!("{}", card), back);

        // Test hint
        card.set_state(FlashCardState::Hint);
        assert_eq!(format!("{}", card), hint);
    }
}
