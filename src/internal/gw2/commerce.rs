use futures::Future;

use ::prelude::*;

impl GW2 {
    /// Fetches the current going rate converting the given number of gems into gold.
    #[must_use]
    pub fn gems_to_gold(&self, qty: u64) -> impl Future<Item = CurrencyConversion, Error = APIError> {
        let client = self.get_http_client();
        let endpoint = Endpoint::CommerceExchangeGems;
        ::internal::http::request_without_ids(client, endpoint, Some(qty), None, None)
            .and_then(|res| ::internal::http::convert_to_struct::<CurrencyConversion>(&res))
    }

    /// Fetches the current conversion rate converting gold into gems.
    #[must_use]
    pub fn gold_to_gems(&self, qty: u64) -> impl Future<Item = CurrencyConversion, Error = APIError> {
        let client = self.get_http_client();
        let endpoint = Endpoint::CommerceExchangeCoins;
        ::internal::http::request_without_ids(client, endpoint, Some(qty), None, None)
            .and_then(|res| ::internal::http::convert_to_struct::<CurrencyConversion>(&res))
    }

    /// Fetches trading post sales data for the given item identifiers.
    #[must_use]
    pub fn trading_post_listings<'a>(&self, ids: &'a [u64]) -> impl Future<Item = Vec<Listing>, Error = APIError> + 'a {
        let client = self.get_http_client();
        let endpoint = Endpoint::CommerceListings;
        ::internal::http::request_with_numeric_ids(client, endpoint, Some(ids), None, None, None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<Listing>>(&res))
    }

    /// Fetches trading post data data on the prices of the given items.
    #[must_use]
    pub fn trading_post_prices<'a>(&self, ids: &'a [u64]) -> impl Future<Item = Vec<Prices>, Error = APIError> + 'a {
        let client = self.get_http_client();
        let endpoint = Endpoint::CommercePrices;
        ::internal::http::request_with_numeric_ids(client, endpoint, Some(ids), None, None, None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<Prices>>(&res))
    }

    /// Fetches the trading post deliveries for an account.
    #[must_use]
    pub fn trading_post_deliveries(&self) -> impl Future<Item = TradingPostDeliveries, Error = APIError> {
        let api_key = self.api_key();
        let client = self.get_http_client();
        let endpoint = Endpoint::CommerceDelivery;
        ::internal::http::request_without_ids(client, endpoint, None, None, Some(api_key))
            .and_then(|res| ::internal::http::convert_to_struct::<TradingPostDeliveries>(&res))
    }

    /// Fetches the current buy orders for an account.
    #[must_use]
    pub fn trading_post_current_buys(&self) -> impl Future<Item = Vec<TradingPostTransaction>, Error = APIError> {
        let api_key = self.api_key();
        let client = self.get_http_client();
        let endpoint = Endpoint::CommerceTransactionsCurrentBuys;
        ::internal::http::request_without_ids(client, endpoint, None, None, Some(api_key))
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<TradingPostTransaction>>(&res))
    }

    /// Fetches the current sell orders for an account.
    #[must_use]
    pub fn trading_post_current_sells(&self) -> impl Future<Item = Vec<TradingPostTransaction>, Error = APIError> {
        let api_key = self.api_key();
        let client = self.get_http_client();
        let endpoint = Endpoint::CommerceTransactionsCurrentSells;
        ::internal::http::request_without_ids(client, endpoint, None, None, Some(api_key))
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<TradingPostTransaction>>(&res))
    }

    /// Fetches historical buy orders for an account.
    #[must_use]
    pub fn trading_post_past_buys(&self) -> impl Future<Item = Vec<TradingPostTransaction>, Error = APIError> {
        let api_key = self.api_key();
        let client = self.get_http_client();
        let endpoint = Endpoint::CommerceTransactionsHistoryBuys;
        ::internal::http::request_without_ids(client, endpoint, None, None, Some(api_key))
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<TradingPostTransaction>>(&res))
    }

    /// Fetches historical sell orders for an account.
    #[must_use]
    pub fn trading_post_past_sells(&self) -> impl Future<Item = Vec<TradingPostTransaction>, Error = APIError> {
        let api_key = self.api_key();
        let client = self.get_http_client();
        let endpoint = Endpoint::CommerceTransactionsHistorySells;
        ::internal::http::request_without_ids(client, endpoint, None, None, Some(api_key))
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<TradingPostTransaction>>(&res))
    }
}