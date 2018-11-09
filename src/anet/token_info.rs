/// `TokenInfo`s provide information on what an API token allows applications to look at on behalf
/// of the user who created them.
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    id: String,
    name: String,
    permissions: Vec<String>,
}

impl TokenInfo {
    /// Returns the identifier of the token.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the associated name given to the token by it's creator.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the associated permissions this token allows applications to access.
    pub fn permissions(&self) -> &[String] {
        &self.permissions
    }
}
