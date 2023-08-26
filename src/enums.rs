#[derive(Debug, PartialEq, PartialOrd, Default)]
pub enum FlashCardState {
    #[default]
    Front,
    Back,
    Hint,
}
