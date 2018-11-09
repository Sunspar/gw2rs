gw2rs_id_u64!(WVWRankId);

/// Ranks one can unlock in World versus World.
#[derive(Debug, Deserialize)]
pub struct WVWRank {
    id: WVWRankId,
    title: String,
    min_rank: u64,
}

impl WVWRank {
    /// Returns the rank's identifier.
    pub fn id(&self) -> WVWRankId {
        self.id
    }

    /// Returns the ran's title, which is displayed to invaders.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the minimium "level" in WvWvW one must be to unlock this rank.
    pub fn min_rank(&self) -> u64 {
        self.min_rank
    }
}
