use futures::Future;

/// The `GW2Result` type is the libraries native return type. Everything leaving the library should
/// return this type.
#[cfg(feature = "futures-boxed")]
pub type GW2Result<T> = Box<Future<Item = T, Error = APIError>>;

/// Enumerates error conditions expected throughout the library.
#[derive(Debug, Serialize)]
pub enum APIError {
    /// The response from the server was accepted, but the data was not convertible into the
    /// proper type.
    BadData,
    /// The request didnt make sense to the server. The `text` field stores the API response's
    /// error message.
    BadRequest(ErrorResponse),
    /// An internal error occured when generating the HTTP request. The error occured BEFORE an
    /// outgoing HTTP request was made.
    InternalHTTP,
    /// There was an error, but we aren't giving it a specific name or meaning yet.
    Generic,
}

/// `ErrorResponse`s represent the JSON payloads from API calls that resulted in errors.
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// The error message returned by ArenaNet.
    pub text: String,
}
