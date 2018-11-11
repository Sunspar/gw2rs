use futures::Future;

use ::prelude::*;

impl GW2 {
    /// Fetches the list of Specialization identifiers.
    #[must_use]
    pub fn specializations(&self) -> impl Future<Item = Vec<SpecializationId>, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::Specializations;
        ::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<SpecializationId>>(&res))
    }

    /// Fetches a list of class specializations.
    #[must_use]
    pub fn specializations_by_ids<'a>(&self, ids: &'a [u64]) -> impl Future<Item = Vec<Specialization>, Error = APIError> + 'a {
        let client = self.get_http_client();
        let locale = self.locale();
        let endpoint = Endpoint::Specializations;
        ::internal::http::request_with_numeric_ids( client, endpoint, Some(ids), None, Some(locale), None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<Specialization>>(&res))
    }
}