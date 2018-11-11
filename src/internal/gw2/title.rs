use futures::Future;

use ::prelude::*;

impl GW2 {
    /// Fetches character title identifiers.
    #[must_use]
    pub fn titles(&self) -> impl Future<Item = Vec<TitleId>, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::Titles;
        ::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<TitleId>>(&res))
    }

    /// Fetches specificied character titles.
    #[must_use]
    pub fn titles_by_ids<'a>(&self, ids: &'a [u64]) -> impl Future<Item = Vec<Title>, Error = APIError> + 'a {
        let client = self.get_http_client();
        let locale = self.locale();
        let endpoint = Endpoint::Titles;
        ::internal::http::request_with_numeric_ids( client, endpoint, Some(ids), None, Some(locale), None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<Title>>(&res))
    }
}