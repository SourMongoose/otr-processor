use openskill::rating::Rating;
use crate::model::structures::mode::Mode;

#[derive(Debug)]
pub struct PlayerRating {
    pub player_id: i32,
    pub mode: Mode,
    pub rating: Rating
}