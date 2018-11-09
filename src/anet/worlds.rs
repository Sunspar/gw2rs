gw2rs_id_u64!(WorldId);

/// The `World` struct provides datapoints related to a world (server). It includes things like the
/// localized name and general server population.
#[derive(Debug, Deserialize)]
pub struct World {
    id: WorldId,
    name: String,
    population: WorldPopulation,
}

impl World {
    /// Returns the world identifier for the given world (server).
    pub fn id(&self) -> WorldId {
        self.id
    }

    /// Returns the world name, as localized by the GW2 settings at request time.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the overall world population for the given world (server).
    pub fn population(&self) -> WorldPopulation {
        self.population
    }
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
