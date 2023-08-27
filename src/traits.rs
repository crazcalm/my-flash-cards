use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::io::Error;
use std::rc::Weak;

use crate::FlashCardState;

pub trait FlashCard<'de>: serde::Deserialize<'de> + Display + Debug {
    fn get_front(&self) -> String;
    fn get_back(&self) -> String;
    fn get_hint(&self) -> Option<String>;
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
    fn add_card_to_top(&mut self, new_card: T);
    fn deck_size(&self) -> usize;
    fn add_deck(&mut self, deck: Box<dyn FlashCards<T>>);
}

pub trait Loader<T: for<'de> FlashCard<'de>> {
    fn load(reader: impl std::io::Read) -> Result<Box<dyn FlashCards<T>>, Error>;
}

pub trait FlashCardsManager<T: FlipFlashCard> {
    fn next_card(&mut self) -> Option<Weak<RefCell<T>>>;
    fn current_card(&mut self) -> Option<Weak<RefCell<T>>>;
    fn flip_current_card(&mut self);
    fn try_to_flip_current_card_to_hint(&mut self);
    fn reset_current_card_state(&mut self);
    fn previous_card(&mut self) -> Option<Weak<RefCell<T>>>;
    fn shuffle(&mut self);
    fn add_previous_cards_to_deck(&mut self);
    fn num_of_cards_in_deck(&self) -> usize;
    fn num_of_cards_seen(&self) -> usize;
}
