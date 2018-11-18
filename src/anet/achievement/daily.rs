use ::prelude::*;

/// `DailyAchievements` are the response of the daily achievement endpoints and map to the various
/// categories of daily achievements found in the Game.
#[derive(Debug, Serialize, Deserialize)]
pub struct DailyAchievements {
    pub pve: Vec<DailyAchievement>,
    pub pvp: Vec<DailyAchievement>,
    pub wvw: Vec<DailyAchievement>,
    pub fractals: Vec<DailyAchievement>,
    pub special: Vec<DailyAchievement>,
}

gw2rs_id_u64!(DailyAchievementId);

/// `DailyAchievement`s represent single daily achievements within Guild Wars 2.
#[derive(Debug, Serialize, Deserialize)]
pub struct DailyAchievement {
    pub id: DailyAchievementId,
    pub level: LevelRange,
    pub required_access: Vec<RequiredAccess>,
}
