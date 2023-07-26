use std::io::Error;
use std::fmt::Display;

use crate::FlashCardState;


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

pub trait Loader<T: for<'de> FlashCard<'de>> {
    fn load(reader: impl std::io::Read) -> Result<Box<dyn FlashCards<T>>, Error>;
}


