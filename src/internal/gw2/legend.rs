use futures::Future;

use ::prelude::*;

impl GW2 { 
    /// Fetches a list of GW2 Revenant Legend identifiers.
    #[must_use]
    pub fn legends(&self) -> impl Future<Item = Vec<LegendId>, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::Legends;
        ::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<LegendId>>(&res))
    }

    /// Fetches a list of GW2 Revenant Legends. 
    #[must_use]
    pub fn legends_by_ids<'a>(&self, ids: &'a [&str]) -> impl Future<Item = Vec<Legend>, Error = APIError> + 'a {
        let client = self.get_http_client();   
        let locale = self.locale(); 
        let endpoint = Endpoint::Legends;
        ::internal::http::request_with_string_ids(client, endpoint, Some(ids), None, Some(locale), None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<Legend>>(&res))
    }
}