gw2rs_id_u64!(UpgradeId);

/// Ranks one can unlock in World versus World.
#[derive(Debug, Deserialize)]
pub struct Upgrade {
    pub id: UpgradeId,
    pub tiers: Vec<UpgradeTier>,
}

/// Represents a tier of upgraded status on structures within World versus World maps.
#[derive(Debug, Deserialize)]
pub struct UpgradeTier {
    pub name: String,
    pub yaks_required: u64,
    pub upgrades: Vec<UpgradeTierUpgrade>,
}

/// Represents an individual structure upgrade within World versus World.
#[derive(Debug, Deserialize)]
pub struct UpgradeTierUpgrade {
    pub name: String,
    pub description: String,
    pub icon: String,
}
