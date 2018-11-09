/// `Region`s represent the broad categorizations of regions within Guild Wars 2.
#[derive(Debug, Serialize, Deserialize)]
pub enum Region {
    /// The continent of Central Tyria.
    Tyria,
    /// Maguuma, and various Heart of Thorns maps.
    Maguuma,
    /// The Crystal Desert.
    CrystalDesert,
}
