use crate::internal::error_response::ErrorResponse;

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
