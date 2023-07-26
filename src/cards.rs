use std::fmt::Display;
use std::iter::FromIterator;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::{FlashCard, FlashCards};

#[derive(Debug)]
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
    pub fn new() -> Self {
        Cards { data: Vec::new() }
    }
}

impl<T> Display for Cards<T>
where
    T: for<'de> FlashCard<'de>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Early exit
        if self.data.is_empty() {
            return Ok(());
        }

        let length = &self.data.len();
        for index in 0..length - 1 {
            write!(f, "{}, ", &self.data[index])?;
        }
        write!(f, "{}", &self.data[length - 1])?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Card;

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
    fn test_flashcards_empty_print() {
        let cards: Cards<Card> = Cards::new();
        assert_eq!("", format!("{}", cards))
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
}
