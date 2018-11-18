gw2rs_id_u64!(WorldId);

/// The `World` struct provides datapoints related to a world (server). It includes things like the
/// localized name and general server population.
#[derive(Debug, Deserialize)]
pub struct World {
    pub id: WorldId,
    pub name: String,
    pub population: WorldPopulation,
}

/// The `WorldPopulation` enum provides compile-time checks around the population restrictions
/// for worlds (servers).
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum WorldPopulation {
    /// The world population is "Low".
    Low,
    /// The world population is "Medium".
    Medium,
    /// The world population is "High".
    High,
    /// The world population is "VeryHigh".
    VeryHigh,
    /// The world population is "Full". Transfers and new account's cannot create characters
    /// here.
    Full,
}
