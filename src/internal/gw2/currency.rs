use futures::Future;

use ::prelude::*;

impl GW2 {
    /// Retrieves a list of Currency identifiers.
    #[must_use]
    pub fn currencies(&self) -> impl Future<Item = Vec<CurrencyId>, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::Currencies;
        ::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<CurrencyId>>(&res))
    }

    /// Retrieves a list of Currency objects using the given list of identifiers.
    #[must_use]
    pub fn currencies_by_ids<'a>(&self, ids: &'a [u64]) -> impl Future<Item = Vec<Currency>, Error = APIError> + 'a {
        let client = self.get_http_client();
        let locale = self.locale();
        let endpoint = Endpoint::Currencies;
        ::internal::http::request_with_numeric_ids( client, endpoint, Some(ids), None, Some(locale), None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<Currency>>(&res))
    }
}