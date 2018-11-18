use ::prelude::*;

gw2rs_id_string!(LegendId);

#[derive(Debug, Serialize, Deserialize)]
pub struct Legend {
    pub id: LegendId,
    pub swap: SkillId,
    pub heal: SkillId,
    pub elite: SkillId,
    pub utilities: Vec<SkillId>,
}
