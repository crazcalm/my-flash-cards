use std::fmt::Display;
use std::io::Error;
use std::iter::FromIterator;

use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Deserialize;

#[derive(Debug, PartialEq, PartialOrd, Default)]
pub enum FlashCardState {
    #[default]
    Front,
    Back,
    Hint,
}

pub trait FlashCard<'de>: serde::Deserialize<'de> + Display {
    fn get_front(&self) -> &str;
    fn get_back(&self) -> &str;
    fn get_hint(&self) -> &str;
}

pub trait FlipFlashCard: for<'de> FlashCard<'de> {
    fn get_state(&self) -> &FlashCardState;
    fn set_state(&mut self, state: FlashCardState);
    fn flip(&mut self) -> &FlashCardState;
}

pub trait FlashCards<T>: Display
where
    T: for<'de> FlashCard<'de>,
{
    fn shuffle(&mut self);
    fn draw(&mut self) -> Option<T>;
    fn add_card(&mut self, new_card: T);
    fn deck_size(&self) -> usize;
}

#[derive(Deserialize)]
pub struct Card {
    #[serde(skip)]
    state: FlashCardState,
    front: String,
    back: String,
    hint: String,
}

impl Card {
    pub fn new(front: String, back: String, hint: String) -> Self {
        Card {
            front,
            back,
            hint,
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
        &self.hint
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

pub struct Cards<T>
where
    T: for<'de> FlashCard<'de>,
{
    data: Vec<T>,
}

impl<T> Cards<T>
where
    T: for<'de> FlashCard<'de>,
{
    fn new() -> Self {
        Cards { data: Vec::new() }
    }
}

impl<T> Display for Cards<T>
where
    T: for<'de> FlashCard<'de>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //TODO: Figure out how to use Vec.join (needs to be implemented)
        for card in &self.data {
            writeln!(f, "{},", card)?;
        }

        Ok(())
    }
}

impl<T> FlashCards<T> for Cards<T>
where
    T: for<'de> FlashCard<'de>,
{
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.data.shuffle(&mut rng);
    }

    fn deck_size(&self) -> usize {
        self.data.len()
    }
    fn draw(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn add_card(&mut self, new_card: T) {
        self.data.push(new_card);
    }
}

impl<U> FromIterator<U> for Cards<U>
where
    U: for<'de> FlashCard<'de>,
{
    fn from_iter<T: IntoIterator<Item = U>>(iter: T) -> Self {
        let mut collection = Self::new();
        for card in iter {
            collection.add_card(card);
        }
        collection
    }
}

pub trait Loader<T: for<'de> FlashCard<'de>> {
    fn load(reader: impl std::io::Read) -> Result<Box<dyn FlashCards<T>>, Error>;
}

struct CSV {}

impl<T> Loader<T> for CSV
where
    T: for<'de> FlashCard<'de> + 'static,
{
    fn load(reader: impl std::io::Read) -> Result<Box<dyn FlashCards<T>>, Error> {
        let mut rdr = csv::Reader::from_reader(reader);
        let mut cards: Cards<T> = Cards::new();

        for result in rdr.deserialize() {
            let record: T = result?;
            cards.add_card(record);
        }

        Ok(Box::new(cards))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_cards() -> Cards<Card> {
        (0..10)
            .map(|x| {
                Card::new(
                    format!("{} - front", x),
                    format!("{} - back", x),
                    format!("{} - hint", x),
                )
            })
            .collect()
    }

    #[test]
    fn test_csv_reader() {
        let data = "\
front,back,hint, 
front_1,back_1,hint_1,
front_2,back_2,hint_2,
";
        //let mut result = csv_reader(data.as_bytes()).unwrap();
        let mut result = CSV::load(data.as_bytes()).unwrap();

        let card_2: Card = result.draw().unwrap();
        let card_1: Card = result.draw().unwrap();

        assert_eq!(card_2.get_front(), "front_2");
        assert_eq!(card_1.get_hint(), "hint_1");
    }

    #[test]
    fn test_flashcards_draw() {
        let mut cards = create_test_cards();

        while cards.deck_size() > 0 {
            assert!(cards.draw().is_some());
        }

        assert!(cards.draw().is_none());
    }

    #[test]
    fn test_flashcards_deck_size() {
        let cards = create_test_cards();
        assert_eq!(10, cards.deck_size())
    }

    #[test]
    fn test_flashcards_shuffle() {
        let mut found_difference = false;

        // The outer for loop is just in case the shuffle order is the same as the orginal order.
        // In that case, try again.
        for _ in 0..10 {
            // Shuffle the first deck
            let mut cards_1 = create_test_cards();
            cards_1.shuffle();

            // Do not shuffle the second deck
            let mut cards_2 = create_test_cards();

            println!("cards_1: {}", cards_1);
            println!("cards_2: {}", cards_2);

            for _ in 0..10 {
                let card_from_deck_1 = cards_1.draw().unwrap();
                let card_from_deck_2 = cards_2.draw().unwrap();

                println!("cards_1 draw: {}", card_from_deck_1);
                println!("cards_2 draw: {}", card_from_deck_2);

                if !format!("{}", card_from_deck_1).eq(&format!("{}", card_from_deck_2)) {
                    found_difference = true;
                    break;
                }
            }

            if found_difference {
                break;
            }
        }

        assert!(found_difference)
    }

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
