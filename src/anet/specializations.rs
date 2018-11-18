use ::prelude::*;

gw2rs_id_u64!(SpecializationId);

#[derive(Debug, Deserialize, Serialize)]
pub struct Specialization {
    pub id: SpecializationId,
    pub name: String,
    pub profession: String, // TODO: enum
    pub elite: bool,
    pub icon: String,
    pub background: String,
    pub minor_traits: Vec<TraitId>,
    pub major_traits: Vec<TraitId>,
}
