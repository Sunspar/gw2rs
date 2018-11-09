gw2rs_id_u64!(UpgradeId);

/// Ranks one can unlock in World versus World.
#[derive(Debug, Deserialize)]
pub struct Upgrade {
    id: UpgradeId,
    tiers: Vec<UpgradeTier>,
}

impl Upgrade {
    /// Returns the internal identifier used for querying the API.
    pub fn id(&self) -> UpgradeId {
        self.id
    }

    /// Returns the set of upgrade tiers associated with this upgrade.
    pub fn tiers(&self) -> &[UpgradeTier] {
        self.tiers.as_slice()
    }
}

/// Represents a tier of upgraded status on structures within World versus World maps.
#[derive(Debug, Deserialize)]
pub struct UpgradeTier {
    name: String,
    yaks_required: u64,
    upgrades: Vec<UpgradeTierUpgrade>,
}

impl UpgradeTier {
    /// Returns the tier's name as presented in the game.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the number of yaks required to unlock the next upgrade tier.
    pub fn yaks_required(&self) -> u64 {
        self.yaks_required
    }

    /// Returns the set of individual upgrades associated with this tier.
    pub fn upgrades(&self) -> &[UpgradeTierUpgrade] {
        self.upgrades.as_slice()
    }
}

/// Represents an individual structure upgrade within World versus World.
#[derive(Debug, Deserialize)]
pub struct UpgradeTierUpgrade {
    name: String,
    description: String,
    icon: String,
}

impl UpgradeTierUpgrade {
    /// Returns the upgrade's name as found in the game.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the description of the upgrade as found in the game.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the URL of the upgrade icon.
    pub fn icon(&self) -> &str {
        &self.icon
    }
}
