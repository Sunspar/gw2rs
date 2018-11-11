use futures::Future;

use crate::prelude::*;

impl GW2 {
    /// Fetches the list of Story identifiers.
    pub fn stories(&self) -> impl Future<Item = Vec<StoryId>, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::Stories;
        crate::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| crate::internal::http::convert_to_struct::<Vec<StoryId>>(&res))
    }

    /// Fetches a list of stories with identifiers matching the input set.
    pub fn stories_by_ids<'a>(&self, ids: &'a [u64]) -> impl Future<Item = Vec<Story>, Error = APIError> + 'a {
        let client = self.get_http_client();
        let locale = self.locale();
        let endpoint = Endpoint::Stories;
        crate::internal::http::request_with_numeric_ids( client, endpoint, Some(ids), None, Some(locale), None)
            .and_then(|res| crate::internal::http::convert_to_struct::<Vec<Story>>(&res))
    }

    /// Fetches the list of story season identifiers.
    pub fn story_seasons(&self) -> impl Future<Item = Vec<StorySeasonId>, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::StorySeasons;
        crate::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| crate::internal::http::convert_to_struct::<Vec<StorySeasonId>>(&res))
    }

    /// Fetches a list of StorySeason's with identifiers matching the input set.
    pub fn story_seasons_by_ids<'a>(&self, ids: &'a [&str]) -> impl Future<Item = Vec<StorySeason>, Error = APIError> + 'a {
        let client = self.get_http_client();   
        let locale = self.locale(); 
        let endpoint = Endpoint::StorySeasons;
        crate::internal::http::request_with_string_ids(client, endpoint, Some(ids), None, Some(locale), None)
            .and_then(|res| crate::internal::http::convert_to_struct::<Vec<StorySeason>>(&res))
    }
}