use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};

use rand::{thread_rng, Rng};

use crate::enums::FlashCardState;
use crate::traits::{FlashCard, FlashCards, FlashCardsManager, FlipFlashCard};

pub struct CardsManager<T>
where
    T: for<'de> FlashCard<'de>,
{
    unseen_cards: VecDeque<Rc<RefCell<T>>>,
    seen_cards: VecDeque<Rc<RefCell<T>>>,
}

impl<T> CardsManager<T>
where
    T: for<'de> FlashCard<'de> + FlipFlashCard,
{
    pub fn create_from_deck(mut deck: impl FlashCards<T>) -> Self {
        let mut unseen_cards = VecDeque::new();
        let mut card = deck.draw();
        while card.is_some() {
            unseen_cards.push_back(Rc::new(RefCell::new(card.unwrap())));
            card = deck.draw();
        }
        Self {
            seen_cards: VecDeque::new(),
            unseen_cards,
        }
    }
}

impl<T> FlashCardsManager<T> for CardsManager<T>
where
    T: for<'de> FlashCard<'de> + FlipFlashCard,
{
    fn next_card(&mut self) -> Option<Weak<RefCell<T>>> {
        match self.unseen_cards.pop_front() {
            Some(card) => {
                let card_weak_ref = Rc::downgrade(&card);
                self.seen_cards.push_front(card);

                Some(card_weak_ref)
            }
            None => None,
        }
    }

    fn previous_card(&mut self) -> Option<Weak<RefCell<T>>> {
        match self.seen_cards.pop_front() {
            Some(card) => {
                let card_weak_ref = Rc::downgrade(&card);
                self.unseen_cards.push_front(card);

                Some(card_weak_ref)
            }
            None => None,
        }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        let total_num_of_cards = self.unseen_cards.len();

        for _ in 0..total_num_of_cards {
            let rand_num_1 = rng.gen_range(0..total_num_of_cards);
            let rand_num_2 = rng.gen_range(0..total_num_of_cards);

            self.unseen_cards.swap(rand_num_1, rand_num_2);
        }
    }

    fn add_previous_cards_to_deck(&mut self) {
        for _ in 0..self.num_of_cards_seen() {
            self.unseen_cards
                .push_front(self.seen_cards.pop_front().unwrap());
        }
    }

    fn num_of_cards_seen(&self) -> usize {
        self.seen_cards.len()
    }

    fn num_of_cards_in_deck(&self) -> usize {
        self.unseen_cards.len()
    }

    fn current_card(&mut self) -> Option<Weak<RefCell<T>>> {
        match self.seen_cards.pop_front() {
            None => None,
            Some(card) => {
                let card_weak_ref = Rc::downgrade(&card);
                self.seen_cards.push_front(card);

                Some(card_weak_ref)
            }
        }
    }

    fn flip_current_card(&mut self) {
        match self.seen_cards.pop_front() {
            None => {}
            Some(card) => {
                let mut card_instance = card.borrow_mut();
                card_instance.flip();
                drop(card_instance);

                self.seen_cards.push_front(card);
            }
        }
    }

    fn try_to_flip_current_card_to_hint(&mut self) {
        match self.seen_cards.pop_front() {
            None => {}
            Some(card) => {
                let mut card_instance = card.borrow_mut();
                if card_instance.get_hint().is_some() {
                    card_instance.set_state(FlashCardState::Hint)
                }
                drop(card_instance);

                self.seen_cards.push_front(card);
            }
        }
    }

    fn reset_current_card_state(&mut self) {
        match self.seen_cards.pop_front() {
            None => {}
            Some(card) => {
                let mut card_instance = card.borrow_mut();
                card_instance.set_state(FlashCardState::Front);
                drop(card_instance);

                self.seen_cards.push_front(card);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Card, Cards};

    fn create_test_manager() -> CardsManager<Card> {
        let cards: Cards<Card> = (0..10)
            .map(|x| {
                Card::new(
                    format!("{} - front", x),
                    format!("{} - back", x),
                    format!("{} - hint", x),
                )
            })
            .collect();

        CardsManager::create_from_deck(cards)
    }

    #[test]
    fn test_reset_card_to_front() {
        let mut card_manager = create_test_manager();

        let _ = card_manager.next_card().unwrap();
        card_manager.flip_current_card();
        let card_ref = card_manager.current_card().unwrap();

        {
            let binding = card_ref.upgrade().unwrap();
            let card = binding.borrow();
            assert_eq!(card.get_state(), &FlashCardState::Back);
        }

        let _ = card_manager.reset_current_card_state();
        let binding = card_ref.upgrade().unwrap();
        let card = binding.borrow();
        assert_eq!(card.get_state(), &FlashCardState::Front);
    }

    #[test]
    fn test_get_current_card() {
        let mut card_manager = create_test_manager();

        let next_card = card_manager.next_card().unwrap();
        let current_card = card_manager.current_card().unwrap();

        assert!(next_card.ptr_eq(&current_card));
    }

    #[test]
    fn test_try_to_flip_current_card_to_hint() {
        let mut card_manager = create_test_manager();

        let _ = card_manager.next_card();
        card_manager.try_to_flip_current_card_to_hint();
        let binding = card_manager.current_card().unwrap().upgrade().unwrap();
        let card = binding.borrow();
        assert_eq!(card.get_state(), &FlashCardState::Hint);
    }
    #[test]
    fn test_flip_current_card() {
        let mut card_manager = create_test_manager();

        let _ = card_manager.next_card();
        card_manager.flip_current_card();
        let binding = card_manager.current_card().unwrap().upgrade().unwrap();
        let card = binding.borrow();
        assert_eq!(card.get_state(), &FlashCardState::Back);
    }

    #[test]
    fn test_previous_card() {
        let mut card_manager = create_test_manager();

        let previous_card = card_manager.previous_card();
        assert!(previous_card.is_none());

        let next_card = card_manager.next_card().unwrap();
        let previous_card = card_manager.previous_card().unwrap();

        assert!(previous_card.ptr_eq(&next_card));
        assert_eq!(
            previous_card.upgrade().unwrap().borrow().get_front(),
            next_card.upgrade().unwrap().borrow().get_front()
        );
    }

    #[test]
    fn test_next_card() {
        let mut card_manager = create_test_manager();

        let next_card = card_manager.next_card();
        assert!(next_card.is_some());
        assert_eq!(1, card_manager.num_of_cards_seen());
    }

    #[test]
    fn test_add_previous_card_to_deck() {
        let mut card_manager = create_test_manager();
        let mut seen_cards = VecDeque::new();

        let total_num_of_cards_in_manager = card_manager.num_of_cards_in_deck();

        for _ in 0..5 {
            seen_cards.push_back(card_manager.next_card().unwrap());
        }

        assert_eq!(5, card_manager.num_of_cards_seen());

        card_manager.add_previous_cards_to_deck();

        for _ in 0..5 {
            let expect_card = seen_cards.pop_front().unwrap();
            let card = card_manager.next_card().unwrap();

            assert!(expect_card.ptr_eq(&card));
        }

        card_manager.add_previous_cards_to_deck();
        assert_eq!(
            total_num_of_cards_in_manager,
            card_manager.num_of_cards_in_deck()
        );
    }

    #[test]
    fn test_shuffle() {
        let mut card_manager = create_test_manager();
        let mut seen_cards = VecDeque::new();

        for _ in 0..10 {
            seen_cards.push_back(card_manager.next_card().unwrap());
        }

        card_manager.add_previous_cards_to_deck();
        card_manager.shuffle();

        let mut is_shuffled = false;
        for _ in 0..10 {
            let expect_card = seen_cards.pop_front().unwrap();
            let card = card_manager.next_card().unwrap();

            if !expect_card.ptr_eq(&card) {
                is_shuffled = true;
                break;
            }
        }

        assert!(is_shuffled);
    }
}
