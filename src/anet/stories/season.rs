use ::prelude::*;

gw2rs_id_string!(StorySeasonId);

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StorySeason {
    id: StorySeasonId,
    name: String,
    order: u64,
    stories: Vec<StoryId>,
}

impl StorySeason {
    /// Returns the season's internal identifier.
    pub fn id(&self) -> &StorySeasonId {
        &self.id
    }

    /// Returns the season's localized name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the season's natural ordering in the list of living story seasons.
    pub fn order(&self) -> u64 {
        self.order
    }

    /// Returns the season's list of related stories.
    pub fn stories(&self) -> &[StoryId] {
        self.stories.as_slice()
    }
}
