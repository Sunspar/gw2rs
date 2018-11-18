gw2rs_id_u64!(WVWRankId);

/// Ranks one can unlock in World versus World.
#[derive(Debug, Deserialize)]
pub struct WVWRank {
    pub id: WVWRankId,
    pub title: String,
    pub min_rank: u64,
}
