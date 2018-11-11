use futures::Future;

use crate::prelude::*;

impl GW2 {
    /// Fetches character title identifiers.
    pub fn titles(&self) -> impl Future<Item = Vec<TitleId>, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::Titles;
        crate::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| crate::internal::http::convert_to_struct::<Vec<TitleId>>(&res))
    }

    /// Fetches specificied character titles.
    pub fn titles_by_ids<'a>(&self, ids: &'a [u64]) -> impl Future<Item = Vec<Title>, Error = APIError> + 'a {
        let client = self.get_http_client();
        let locale = self.locale();
        let endpoint = Endpoint::Titles;
        crate::internal::http::request_with_numeric_ids( client, endpoint, Some(ids), None, Some(locale), None)
            .and_then(|res| crate::internal::http::convert_to_struct::<Vec<Title>>(&res))
    }
}