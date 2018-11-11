use futures::Future;

use ::prelude::*;

impl GW2 {
    /// Retrieves a list of Glider identifiers.
    #[must_use]
    pub fn gliders(&self) -> impl Future<Item = Vec<GliderId>, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::Gliders;
        ::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<GliderId>>(&res))
    }

    /// Retrieves a list of GW2 Gliders.
    #[must_use]
    pub fn gliders_by_ids<'a>(&self, ids: &'a [u64]) -> impl Future<Item = Vec<Glider>, Error = APIError> + 'a {
        let client = self.get_http_client();
        let locale = self.locale();
        let endpoint = Endpoint::Gliders;
        ::internal::http::request_with_numeric_ids( client, endpoint, Some(ids), None, Some(locale), None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<Glider>>(&res))
    }
}