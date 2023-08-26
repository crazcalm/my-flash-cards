use std::collections::VecDeque;
use std::fmt::Display;
use std::iter::FromIterator;

use rand::{thread_rng, Rng};

use crate::{FlashCard, FlashCards};

#[derive(Debug)]
pub struct Cards<T>
where
    T: for<'de> FlashCard<'de>,
{
    data: VecDeque<T>,
}

impl<T> Cards<T>
where
    T: for<'de> FlashCard<'de>,
{
    pub fn new() -> Self {
        Cards {
            data: VecDeque::new(),
        }
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
        let total_num_of_cards = self.data.len();

        for _ in 0..total_num_of_cards {
            let rand_num_1 = rng.gen_range(0..total_num_of_cards);
            let rand_num_2 = rng.gen_range(0..total_num_of_cards);

            self.data.swap(rand_num_1, rand_num_2);
        }
    }

    fn deck_size(&self) -> usize {
        self.data.len()
    }
    fn draw(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    fn add_card(&mut self, new_card: T) {
        self.data.push_back(new_card);
    }
    fn add_card_to_top(&mut self, new_card: T) {
        self.data.push_front(new_card);
    }
    fn add_deck(&mut self, mut deck: Box<dyn FlashCards<T>>) {
        while deck.deck_size() > 0 {
            self.add_card(deck.draw().unwrap());
        }
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
    fn test_flashcards_add_deck() {
        let mut cards_1 = create_test_cards();
        let cards_2 = create_test_cards();

        cards_1.add_deck(Box::new(cards_2));

        assert_eq!(cards_1.deck_size(), 20);
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
    fn test_flashcards_add_card_to_top() {
        let mut cards = create_test_cards();

        let card = cards.draw().unwrap();

        let card1_text = card.get_front();

        cards.add_card_to_top(card);

        let card_2 = cards.draw().unwrap();

        assert_eq!(card1_text, card_2.get_front());
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
