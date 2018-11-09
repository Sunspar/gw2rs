use crate::prelude::*;

gw2rs_id_u64!(TitleId);

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Title {
    id: TitleId,
    name: String,
    achievements: Vec<AchievementId>,
}

impl Title {
    /// Returns the titles internal identifier.
    pub fn id(&self) -> TitleId {
        self.id
    }

    /// Returns the title's localized name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the slice of achievement identifiers required to unlock this title.
    pub fn achievements(&self) -> &[AchievementId] {
        self.achievements.as_slice()
    }
}
