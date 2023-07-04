use crate::episode::Episode;

// result item (episode) depends on the api call type
// esp the type & show field is prolematic

// web/stream results
//

pub type WebResults = Vec<Episode>;
pub type TVResults = Vec<Episode>;
pub type AllResults = Vec<Episode>;
