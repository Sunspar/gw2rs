use ::prelude::*;

gw2rs_id_u64!(StoryId);

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Story {
    pub id: StoryId,
    pub season: StorySeasonId,
    pub name: String,
    pub description: String,
    pub timeline: String,
    pub level: u64,
    pub order: u64,
    pub chapters: Vec<StoryChapter>,
    pub races: Option<Vec<String>>, // TODO: Enum this
    pub flags: Option<Vec<String>>, // TODO: Enum this
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StoryChapter {
    pub name: String,
}
