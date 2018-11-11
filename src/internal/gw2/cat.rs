use futures::Future;

use ::prelude::*;

impl GW2 {
    /// Retrieves cat identifiers.
    #[must_use]
    pub fn cats(&self) -> impl Future<Item = Vec<CatId>, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::Cats;
        ::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<CatId>>(&res))
    }

    /// Retrieves cats using a list of identifiers.
    #[must_use]
    pub fn cats_by_ids<'a>(&self, ids: &'a [u64]) -> impl Future<Item = Vec<Cat>, Error = APIError> + 'a {
        let client = self.get_http_client();
        let endpoint = Endpoint::Cats;
        ::internal::http::request_with_numeric_ids(client, endpoint, Some(ids), None, None, None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<Cat>>(&res))
    }
}