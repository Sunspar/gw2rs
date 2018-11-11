/// `ErrorResponse`s represent the JSON payloads from API calls that resulted in errors.
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// The error message returned by ArenaNet.
    pub text: String,
}