#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Build {
    id: u64,
}

impl Build {
    /// Returns the build ID number.
    pub fn id(&self) -> u64 {
        self.id
    }
}
