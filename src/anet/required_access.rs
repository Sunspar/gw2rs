/// `RequiredAccess`es represent whether access to a type is restricted based on a particular
/// expansion of Guild Wars 2.
#[derive(Debug, Serialize, Deserialize)]
pub enum RequiredAccess {
    /// The associated item requires the base game.
    GuildWars2,
    /// The associated item requires Guild Wars 2: Heart of Thorns.
    HeartOfThorns,
    /// The associated item requires Guild Wars 2: Path of Fire.
    PathOfFire,
}
