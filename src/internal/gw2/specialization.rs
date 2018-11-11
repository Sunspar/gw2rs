use futures::Future;

use crate::prelude::*;

impl GW2 {
    /// Fetches the list of Specialization identifiers.
    pub fn specializations(&self) -> impl Future<Item = Vec<SpecializationId>, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::Specializations;
        crate::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| crate::internal::http::convert_to_struct::<Vec<SpecializationId>>(&res))
    }

    /// Fetches a list of class specializations.
    pub fn specializations_by_ids<'a>(&self, ids: &'a [u64]) -> impl Future<Item = Vec<Specialization>, Error = APIError> + 'a {
        let client = self.get_http_client();
        let locale = self.locale();
        let endpoint = Endpoint::Specializations;
        crate::internal::http::request_with_numeric_ids( client, endpoint, Some(ids), None, Some(locale), None)
            .and_then(|res| crate::internal::http::convert_to_struct::<Vec<Specialization>>(&res))
    }
}