use ::prelude::*;

/// `DailyAchievements` are the response of the daily achievement endpoints and map to the various
/// categories of daily achievements found in the Game.
#[derive(Debug, Serialize, Deserialize)]
pub struct DailyAchievements {
    pve: Vec<DailyAchievement>,
    pvp: Vec<DailyAchievement>,
    wvw: Vec<DailyAchievement>,
    fractals: Vec<DailyAchievement>,
    special: Vec<DailyAchievement>,
}

impl DailyAchievements {
    /// Returns a slice of `DailyAchievement`s that represent the daily PvE achievements.
    pub fn pve(&self) -> &[DailyAchievement] {
        &self.pve
    }

    /// Returns a slice of `DailyAchievement`s that represent the daily Structured PvP
    /// achievements
    pub fn pvp(&self) -> &[DailyAchievement] {
        &self.pvp
    }

    /// Returns a slice of `DailyAchievement`s that represent the daily World Versus World
    /// achievements.
    pub fn wvw(&self) -> &[DailyAchievement] {
        &self.wvw
    }

    /// Returns a slice of `DailyAchievement`s that represent the daily Fractal of the Mists
    /// achievements.
    pub fn fractals(&self) -> &[DailyAchievement] {
        &self.fractals
    }

    /// Returns a slice of `DailyAchievement`s that dont fit into other categories of daily
    /// achievements.
    pub fn special(&self) -> &[DailyAchievement] {
        &self.special
    }
}

gw2rs_id_u64!(DailyAchievementId);
/// `DailyAchievement`s represent single daily achievements within Guild Wars 2.
#[derive(Debug, Serialize, Deserialize)]
pub struct DailyAchievement {
    id: DailyAchievementId,
    level: LevelRange,
    required_access: Vec<RequiredAccess>,
}

impl DailyAchievement {
    /// Returns the daily achievement's identifier.
    pub fn id(&self) -> DailyAchievementId {
        self.id
    }

    /// Returns the daily achievements level range.
    pub fn level(&self) -> &LevelRange {
        &self.level
    }

    /// Returns the daily achievements required access (expansion).
    pub fn required_access(&self) -> &[RequiredAccess] {
        &self.required_access
    }
}
