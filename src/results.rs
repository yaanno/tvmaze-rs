use crate::episode::Episode;

// result item (episode) depends on the api call type

pub type WebResults = Vec<Episode>;
pub type TVResults = Vec<Episode>;
pub type AllResults = Vec<Episode>;
