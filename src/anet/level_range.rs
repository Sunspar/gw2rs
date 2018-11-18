/// `LevelRange`s represent a range of levels, inclusive, that an including type is valid for.
#[derive(Debug, Serialize, Deserialize)]
pub struct LevelRange {
    pub min: u64,
    pub max: u64,
}
