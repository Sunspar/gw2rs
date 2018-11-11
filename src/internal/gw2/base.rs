use hyper::client::{Client, HttpConnector};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Handle;

use ::prelude::*;

/// The Guild Wars 2 API Wrapper.
///
/// Structs of this type provide rust functions mapping to ArenaNet GuildWars 2 API (Version 2).
/// The struct retains the API key to use for authenticated requests, along with the desired Locale.
///
/// Almost every method used to interact with the API through this crate is a self method on GW2.
/// Mutator functions have been provided for scenarios where changing the API key or locale is
/// desired, but since the wrapper keeps very little state, it is usually easier to just spawn new
/// instances for one-off calls.
///
/// All exposed API functionality is, therefore, built on the requirement that only an immutable
/// reference is necessary once initialized.
#[derive(Debug)]
pub struct GW2 {
    /// The Guild Wars 2 API key to use for authenticated requests.
    api_key: String,
    /// The locale to use for localized requests.
    locale: Locale,
    /// The Tokio Core Handle for the API.
    tokio_handle: Handle,
    /// The internal HTTP client for API requests
    http_client: Client<HttpsConnector<HttpConnector>>,
}

impl GW2 {
    /// Initializes the GW2 API interface with a given API token.
    pub fn new<I>(token: I, locale: Locale, handle: Handle) -> GW2
    where
        I: Into<String>,
    {
        Self::new_with_threads(token, locale, handle, 8)
    }

    pub fn new_with_threads<I>(token: I, locale: Locale, handle: Handle, threads: usize)
    -> GW2
    where
        I: Into<String>,
    {
        trace!("Initializing a new GW2 struct.");

        let http_client = Client::configure()
            .connector(
                HttpsConnector::new(threads, &handle).expect("Failed to initialize SSL connector."),
            )
            .build(&handle);

        GW2 {
            api_key: token.into(),
            locale: locale,
            tokio_handle: handle,
            http_client: http_client,
        }
    }

    /// Returns a reference to the current Locale.
    pub fn locale(&self) -> Locale {
        self.locale
    }

    /// Updates the current locale.
    pub fn set_locale(&mut self, l: Locale) {
        self.locale = l;
    }

    /// Returns a reference to the current API key.
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Updates the current API Key.
    ///
    /// # Note
    /// This method is provided for convenience. In usual workflows however it will probably make
    /// more sense to just instantiate a new GW2 struct, as there isnt really any state to
    /// lose. However, this is handy for situations where you're keeping a global struct, or
    /// need to be conservative regarding allocations being made.
    pub fn set_api_key(&mut self, key: String) {
        self.api_key = key;
    }

    /// Grants the caller an immutable reference to the internal HTTP Client.
    /// Useful for queueing up http requests as work to the internal Tokio Core.
    pub fn get_http_client(&self) -> &Client<HttpsConnector<HttpConnector>> {
        &self.http_client
    }

    /// Grants the caller an immutable reference to the internal Tokio handle.
    pub fn get_tokio_handle(&self) -> &Handle {
        &self.tokio_handle
    }
}