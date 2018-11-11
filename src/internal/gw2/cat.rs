use futures::Future;

use crate::prelude::*;

impl GW2 {
    /// Retrieves cat identifiers.
    pub fn cats(&self) -> impl Future<Item = Vec<CatId>, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::Cats;
        crate::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| crate::internal::http::convert_to_struct::<Vec<CatId>>(&res))
    }

    /// Retrieves cats using a list of identifiers.
    pub fn cats_by_ids<'a>(&self, ids: &'a [u64]) -> impl Future<Item = Vec<Cat>, Error = APIError> + 'a {
        let client = self.get_http_client();
        let endpoint = Endpoint::Cats;
        crate::internal::http::request_with_numeric_ids(client, endpoint, Some(ids), None, None, None)
            .and_then(|res| crate::internal::http::convert_to_struct::<Vec<Cat>>(&res))
    }
}