/// `TokenInfo`s provide information on what an API token allows applications to look at on behalf
/// of the user who created them.
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub id: String,
    pub name: String,
    pub permissions: Vec<String>,
}
