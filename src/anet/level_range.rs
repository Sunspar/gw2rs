/// `LevelRange`s represent a range of levels, inclusive, that an including type is valid for.
#[derive(Debug, Serialize, Deserialize)]
pub struct LevelRange {
    min: u64,
    max: u64,
}

impl LevelRange {
    /// Returns the lower bound of the LevelRange, i.e the maximum level supported by the
    /// associated item.
    pub fn min(&self) -> u64 {
        self.min
    }

    /// Returns the upper bound of the LevelRange, i.e the maximum level supported by the
    /// associated item.
    pub fn max(&self) -> u64 {
        self.max
    }
}
