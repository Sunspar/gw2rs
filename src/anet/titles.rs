use ::prelude::*;

gw2rs_id_u64!(TitleId);

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Title {
    pub id: TitleId,
    pub name: String,
    pub achievements: Vec<AchievementId>,
}
