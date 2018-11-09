use crate::prelude::*;

gw2rs_id_u64!(StoryId);

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Story {
    id: StoryId,
    season: StorySeasonId,
    name: String,
    description: String,
    timeline: String,
    level: u64,
    order: u64,
    chapters: Vec<StoryChapter>,
    races: Option<Vec<String>>, // TODO: Enum this
    flags: Option<Vec<String>>, // TODO: Enum this
}

impl Story {
    /// Returns the story's internal identifier
    pub fn id(&self) -> StoryId {
        self.id
    }

    /// Returns the story's associated season.
    pub fn season(&self) -> &StorySeasonId {
        &self.season
    }

    /// Returns the story's localized in-game name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the story's localized in-game description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the timeline of this story -- the years in which this story is presented
    /// in-game.
    pub fn timeline(&self) -> &str {
        &self.timeline
    }

    /// Returns the minimum level at which a character can progress in this story.
    pub fn level(&self) -> u64 {
        self.level
    }

    /// Returns the ordering of this story in the in-game listing.
    pub fn order(&self) -> u64 {
        self.order
    }

    /// Returns the chapters this story has.
    pub fn chapters(&self) -> &[StoryChapter] {
        self.chapters.as_slice()
    }

    /// Returns the races this story is restricted to, if restricted at all.
    pub fn races(&self) -> Option<&[String]> {
        match self.races {
            Some(ref v) => Some(v.as_slice()),
            None => None,
        }
    }

    /// Returns the flags associated with this story.
    pub fn flags(&self) -> Option<&[String]> {
        match self.flags {
            Some(ref v) => Some(v.as_slice()),
            None => None,
        }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StoryChapter {
    name: String,
}

impl StoryChapter {
    /// Returns the chapter's localized name.
    pub fn name(&self) -> &str {
        &self.name
    }
}
