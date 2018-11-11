use futures::Future;

use crate::prelude::*;

impl GW2 { 
    /// Fetches a list of GW2 Revenant Legend identifiers.
    pub fn legends(&self) -> impl Future<Item = Vec<LegendId>, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::Legends;
        crate::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| crate::internal::http::convert_to_struct::<Vec<LegendId>>(&res))
    }

    /// Fetches a list of GW2 Revenant Legends. 
    pub fn legends_by_ids<'a>(&self, ids: &'a [&str]) -> impl Future<Item = Vec<Legend>, Error = APIError> + 'a {
        let client = self.get_http_client();   
        let locale = self.locale(); 
        let endpoint = Endpoint::Legends;
        crate::internal::http::request_with_string_ids(client, endpoint, Some(ids), None, Some(locale), None)
            .and_then(|res| crate::internal::http::convert_to_struct::<Vec<Legend>>(&res))
    }
}