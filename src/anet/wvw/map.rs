/// The `WVWMap` enum provides variants for all World versus World maps.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum WVWMap {
    /// Represents the red server's home boarderlands.
    RedHome,

    /// Represents the green server's home borderlands.
    GreenHome,

    /// Represents the blue server's home borderlands.
    BlueHome,

    /// Represents the Eternal Battlegrounds map.
    Center,
}
