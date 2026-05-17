/// HUMAN: Gates round card dealing behind the current Round Start sequence.
/// AI: Round one waits for the location intro; later rounds can add round-start steps here.
#[derive(bevy::prelude::Resource, Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PendingRoundDealResource {
    pub is_pending: bool,
    pub is_round_deal_complete: bool,
    pub waits_for_location_intro: bool,
    pub location_intro_completed_event_count: usize,
    pub last_location_intro_completed_elapsed_ms: u64,
    pub near_deal_completed_event_count: usize,
    pub last_near_deal_completed_card_count: usize,
}
