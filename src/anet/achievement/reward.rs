use crate::anet::achievement::region::Region;

/// `AchievementReward`s represent the various types of rewards you can receive for completing
/// achievements within the game.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AchievementReward {
    /// The associated achievement rewards the completer with the given number of copper coins.
    Coins {
        /// The amount of coins, in copper.
        count: u64,
    },

    /// The associated achievement rewards the completer with a multiple of the given item.
    Item {
        /// The item identifier.
        id: u64, // TODO: Newtype this once we have the Item endpoints in.
        /// The amount of the item.
        count: u64,
    },

    /// The associated achievement rewards the completer with the given mastery point.
    Mastery {
        /// The Mastery identifier.
        id: u64, // TODO: Newtype this once we have Mastery endpoints in.
        /// The Mastery's region (Tyria, Maguuma, etc).
        region: Region,
    },

    /// The associated achievement rewards the completer with the given title.
    Title {
        /// The Title's identifier.
        id: u64, // TODO: Newtype this once we have Title endpoints in.
    },
}
