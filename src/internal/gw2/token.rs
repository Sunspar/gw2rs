use futures::Future;

use crate::prelude::*;

impl GW2 {
    /// Fetches information on the provided access token.
    pub fn token_info(&self) -> impl Future<Item = TokenInfo, Error = APIError> {
        let api_key = self.api_key();
        let locale = self.locale();
        let client = self.get_http_client();
        let endpoint = Endpoint::TokenInfo;
        crate::internal::http::request_without_ids( client, endpoint, None, Some(locale), Some(api_key))
            .and_then(|res| crate::internal::http::convert_to_struct::<TokenInfo>(&res))
    }
}