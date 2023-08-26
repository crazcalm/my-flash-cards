use std::io::Error;

use crate::{Cards, FlashCard, FlashCards, Loader};

pub struct Csv {}

impl<T> Loader<T> for Csv
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
    use crate::Card;

    #[test]
    fn test_csv_reader() {
        let data = "\
front,back,hint,
front_1,back_1,hint_1,
front_2,back_2,hint_2,
frint_3,back_3,,
";
        let mut result = Csv::load(data.as_bytes()).unwrap();

        let card_1: Card = result.draw().unwrap();
        let card_2: Card = result.draw().unwrap();
        let card_3: Card = result.draw().unwrap();

        assert_eq!(card_3.get_hint(), None);
        assert_eq!(card_2.get_front(), "front_2".to_string());
        assert_eq!(card_1.get_hint(), Some("hint_1".to_string()));
    }
}
