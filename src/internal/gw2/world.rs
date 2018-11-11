use futures::Future;

use ::prelude::*;

impl GW2 {
    /// Fetches a list of world (server) identifiers.
    #[must_use]
    pub fn worlds(&self) -> impl Future<Item = Vec<WorldId>, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::Worlds;
        ::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<WorldId>>(&res))
    }

    /// Fetches world data for the desired world identifiers.
    #[must_use]
    pub fn worlds_by_ids<'a>(&self, ids: &'a [u64]) -> impl Future<Item = Vec<World>, Error = APIError> + 'a {
        let client = self.get_http_client();
        let locale = self.locale();
        let endpoint = Endpoint::Worlds;
        ::internal::http::request_with_numeric_ids( client, endpoint, Some(ids), None, Some(locale), None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<World>>(&res))
    }
}