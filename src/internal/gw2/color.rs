use futures::Future;

use ::prelude::*;

impl GW2 {
    /// Retrieves valid color identifiers.
    #[must_use]
    pub fn colors(&self) -> impl Future<Item = Vec<ColorId>, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::Colors;
        ::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<ColorId>>(&res))
    }
    
    /// Retrieves a list of Colors using the given list of identifiers.
    #[must_use]
    pub fn colors_by_ids<'a>(&self, ids: &'a [u64]) -> impl Future<Item = Vec<Color>, Error = APIError> + 'a {
        let client = self.get_http_client();
        let locale = self.locale();
        let endpoint = Endpoint::Colors;
        ::internal::http::request_with_numeric_ids( client, endpoint, Some(ids), None, Some(locale), None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<Color>>(&res))
    }
}