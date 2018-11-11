use ::prelude::*;

gw2rs_id_string!(LegendId);

#[derive(Debug, Serialize, Deserialize)]
pub struct Legend {
    id: LegendId,
    swap: SkillId,
    heal: SkillId,
    elite: SkillId,
    utilities: Vec<SkillId>,
}

impl Legend {
    /// Returns the legend identifier for this Revenant legend.
    pub fn id(&self) -> &LegendId {
        &self.id
    }

    /// Returns the skill of the Revenant legend swap skill.
    pub fn swap(&self) -> SkillId {
        self.swap
    }

    /// Returns the skill identifier of the legend's related heal skills.
    pub fn heal(&self) -> SkillId {
        self.heal
    }

    /// Returns the skill identifier of the legend's related elite skills.
    pub fn elite(&self) -> SkillId {
        self.elite
    }

    /// Returns a slice of the skill identifiers of the legend's related utility skills.
    pub fn utilities(&self) -> &[SkillId] {
        self.utilities.as_slice()
    }
}
