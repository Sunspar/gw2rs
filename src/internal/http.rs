use futures::{Future, Stream};
use hyper::{Method, Request, Uri};
use hyper::client::{Client, HttpConnector};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use ::std::fmt::{Display, Write};
use ::std::str::FromStr;
use url::Url;

use ::prelude::*;

#[cfg(feature = "header-locale")]
use hyper::header::{q, AcceptLanguage, Authorization, Bearer, LanguageTag, QualityItem};

#[cfg(feature = "header-locale")]
use ::std::collections::BTreeMap;

const API_ROOT: &str = "https://api.guildwars2.com/v2";

fn process_request_params_generic(
    request: &mut Request,
    url: &mut Url,
    quantity: Option<u64>,
    locale: Option<Locale>,
    token: Option<&str>
) {
    let mut query_pairs = url.query_pairs_mut();

    if let Some(qty) = quantity {
        query_pairs.append_pair("quantity", &qty.to_string());
    }

    if let Some(l) = locale {
        // Pull the LanguageTag in directly, so we can avoid having to depend on
        // language-tags just to use a single macro that might not even get
        // compiled into client code in the first place.
        #[cfg(feature = "header-locale")]
        request.headers_mut().set(AcceptLanguage(vec![
            QualityItem::new(
                LanguageTag {
                    language: Some(l.to_string()),
                    extlangs: Vec::new(),
                    script: None,
                    region: None,
                    variants: Vec::new(),
                    extensions: BTreeMap::new(),
                    privateuse: Vec::new(),
                },
                q(1000),
            ),
        ]));

        #[cfg(feature = "query-locale")]
        pairs.append_pair("lang", l.as_ref());
    }

    if let Some(t) = token {
        #[cfg(feature = "header-auth")]
        request.headers_mut().set(Authorization(Bearer {
            token: String::from(t),
        }));

        #[cfg(feature = "query-auth")]
        query_pairs.append_pair("access_token", t);
    }
}

fn process_request_params_strings<A, B>(
    request: &mut Request,
    url: &mut Url,
    identifiers: Option<A>,
    quantity: Option<u64>,
    locale: Option<Locale>,
    token: Option<&str>
)
where 
    A: AsRef<[B]>,
    B: AsRef<str> + Display
{
    process_request_params_generic(request, url, quantity, locale, token);
    let mut query_pairs = url.query_pairs_mut();

    if let Some(ids) = identifiers {
        let mut result = String::new();
        let mut iter = ids.as_ref().iter();
        if let Some(iter_item) = iter.next() {
            write!(&mut result, "{}", iter_item.as_ref()).expect("Failure building identifier string for API request");
            for elt in iter {
                write!(&mut result, ",{}", elt.as_ref()).expect("Failure building identifier string for API request");
            }
        }
        query_pairs.append_pair("ids", &result);
    }   
}

fn process_request_params_ints<A, B>(
    request: &mut Request,
    url: &mut Url,
    identifiers: Option<A>,
    quantity: Option<u64>,
    locale: Option<Locale>,
    token: Option<&str>
) where
    A: AsRef<[B]>,
    B: Into<u64> + Display
{
    process_request_params_generic(request, url, quantity, locale, token);
    let mut query_pairs = url.query_pairs_mut();

    if let Some(ids) = identifiers {
        let mut result = String::new();
        let mut iter = ids.as_ref().iter();
        if let Some(iter_item) = iter.next() {
            write!(&mut result, "{}", iter_item).expect("Failure building identifier string for API request");
            for other_item in iter {
                write!(&mut result, ",{}", other_item).expect("Failure building identifier string for API request");
            }
        }
        query_pairs.append_pair("ids", &result);
    }   
}

pub(crate) fn request_without_ids(
    client: &Client<HttpsConnector<HttpConnector>>,
    endpoint: Endpoint,
    quantity: Option<u64>,
    locale: Option<Locale>,
    token: Option<&str>,
) -> impl Future<Item = String, Error = APIError> {
    let mut url = Url::parse(&format!("{}{}", &API_ROOT, endpoint.as_str()))
        .expect("Fatal error parsing api URL");
    let mut request = Request::new(Method::Get, Uri::default());
    process_request_params_generic(&mut request, &mut url, quantity, locale, token);
    request.set_uri(Uri::from_str(&url.into_string()).expect("Fatal error converting constructed URL to Hyper's Uri type."));

    client
        .request(request)
        .and_then(|res|
            res.body().concat2().map(|chunk| String::from_utf8_lossy(&chunk).to_string())
        )
        .map_err(|_| APIError::InternalHTTP)
}

pub(crate) fn request_with_string_ids<A, B>(
    client: &Client<HttpsConnector<HttpConnector>>,
    endpoint: Endpoint,
    identifiers: Option<A>,
    quantity: Option<u64>,
    locale: Option<Locale>,
    token: Option<&str>
) -> impl Future<Item = String, Error = APIError>
where
    A: AsRef<[B]>,
    B: AsRef<str> + Display
{
    let mut url = Url::parse(&format!("{}{}", &API_ROOT, endpoint.as_str()))
        .expect("Fatal error parsing api URL");
    let mut request = Request::new(Method::Get, Uri::default());
    process_request_params_strings(&mut request, &mut url, identifiers, quantity, locale, token);
    request.set_uri(Uri::from_str(&url.into_string()).expect("Fatal error converting constructed URL to Hyper's Uri type."));

    client
        .request(request)
        .and_then(|res|
            res.body().concat2().map(|chunk| String::from_utf8_lossy(&chunk).to_string())
        )
        .map_err(|_| APIError::InternalHTTP)
}

pub(crate) fn request_with_numeric_ids<A, B>(
    client: &Client<HttpsConnector<HttpConnector>>,
    endpoint: Endpoint,
    identifiers: Option<A>,
    quantity: Option<u64>,
    locale: Option<Locale>,
    token: Option<&str>
) -> impl Future<Item = String, Error = APIError>
where
    A: AsRef<[B]>,
    B: Into<u64> + Display
{
    let mut url = Url::parse(&format!("{}{}", &API_ROOT, endpoint.as_str())).expect("Fatal error parsing api URL");
    let mut request = Request::new(Method::Get, Uri::default());
    process_request_params_ints(&mut request, &mut url, identifiers, quantity, locale, token);
    request.set_uri(Uri::from_str(&url.into_string()).expect("Fatal error converting constructed URL to Hyper's Uri type."));

    client
        .request(request)
        .and_then(|res|
            res.body().concat2().map(|chunk| String::from_utf8_lossy(&chunk).to_string())
        )
        .map_err(|_| APIError::InternalHTTP)
}

/// Converts a fetched string of (presumably) JSON data into the appropriate struct.
pub(crate) fn convert_to_struct<T>(data: &str) -> Result<T, APIError>
where
    T: DeserializeOwned,
{
    match serde_json::from_str::<T>(data) {
        Ok(v) => Ok(v),
        Err(v1) => match serde_json::from_str::<ErrorResponse>(data) {
            Ok(v2) => {
                warn!("API request resulted in a request error.");
                Err(APIError::BadRequest(v2))
            }
            Err(v2) => {
                warn!("Error converting API response into structs.");
                debug!("Initial Error was: {:?}", v1);
                debug!("Error during error checking was: {:?}", v2);
                Err(APIError::BadData)
            }
        },
    }
}
