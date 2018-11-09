use crate::prelude::*;

gw2rs_id_u64!(SpecializationId);

#[derive(Debug, Deserialize, Serialize)]
pub struct Specialization {
    id: SpecializationId,
    name: String,
    profession: String, // TODO: enum
    elite: bool,
    icon: String,
    background: String,
    minor_traits: Vec<TraitId>,
    major_traits: Vec<TraitId>,
}

impl Specialization {
    /// Returns the Specialization's internal identifer.
    pub fn id(&self) -> SpecializationId {
        self.id
    }

    /// Returns the Specialization's localized name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the Specialization's associated Profession (class).
    pub fn profession(&self) -> &str {
        &self.profession
    }

    /// Returns whether or not the Specialization is an elite specialization.
    pub fn elite(&self) -> bool {
        self.elite
    }

    /// Returns the Specialization's icon URL.
    pub fn icon(&self) -> &str {
        &self.icon
    }

    /// Returns the Specialization's background image URL.
    pub fn background(&self) -> &str {
        &self.background
    }

    /// Returns the Specialization's list of minor traits.
    pub fn minor_traits(&self) -> &[TraitId] {
        self.minor_traits.as_slice()
    }

    /// Returns the Specialization's list of major traits.
    pub fn major_traits(&self) -> &[TraitId] {
        self.major_traits.as_slice()
    }
}
