use ::prelude::*;

gw2rs_id_string!(StorySeasonId);

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StorySeason {
    pub id: StorySeasonId,
    pub name: String,
    pub order: u64,
    pub stories: Vec<StoryId>,
}
