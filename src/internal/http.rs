use futures::{Future, Stream};
use hyper::{Method, Request, Uri};
use hyper::client::{Client, HttpConnector};
use hyper_tls::HttpsConnector;
use crate::prelude::*;
use serde::de::DeserializeOwned;
use std::str::FromStr;
use url::Url;

#[cfg(feature = "header-locale")]
use hyper::header::{q, AcceptLanguage, Authorization, Bearer, LanguageTag, QualityItem};
#[cfg(feature = "header-locale")]
use std::collections::BTreeMap;

use std::fmt::Write;

const API_ROOT: &str = "https://api.guildwars2.com/v2";

#[cfg(feature = "futures-boxed")]
type HttpReturn<A, B> = Box<Future<Item = A, Error = B>>;

fn preprocess<'a>(url: &mut Url, req: &mut Request, params: Vec<Param<'a>>) {
    let mut pairs = url.query_pairs_mut();

    for param in params {
        match param {
            Param::Locale(ref item) => {
                // Pull the LanguageTag in directly, so we can avoid having to depend on
                // language-tags just to use a single macro that might not even get
                // compiled into client code in the first place.
                #[cfg(feature = "header-locale")]
                req.headers_mut().set(AcceptLanguage(vec![
                    QualityItem::new(
                        LanguageTag {
                            language: Some(item.to_string()),
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
                pairs.append_pair("lang", item.as_ref());
            }

            Param::AuthToken(item) => {
                #[cfg(feature = "header-auth")]
                req.headers_mut().set(Authorization(Bearer {
                    token: String::from(item),
                }));

                #[cfg(feature = "query-auth")]
                pairs.append_pair("access_token", item);
            }

            Param::Quantity(item) => {
                pairs.append_pair("quantity", &item.to_string());
            }

            Param::StrIds(item) => {
                let mut result = String::new();
                let mut iter = item.into_iter();
                if let Some(iter_item) = iter.next() {
                    write!(&mut result, "{}", iter_item).expect("Failure building identifier string for API request");
                    for elt in iter {
                        write!(&mut result, ",{}", elt).expect("Failure building identifier string for API request");
                    }
                }
                pairs.append_pair("ids", &result);
            }

            Param::IntIds(item) => {
                let mut result = String::new();
                let mut iter = item.into_iter();
                if let Some(iter_item) = iter.next() {
                    write!(&mut result, "{}", iter_item).expect("Failure building identifier string for API request");
                    for other_item in iter {
                        write!(&mut result, ",{}", other_item).expect("Failure building identifier string for API request");
                    }
                }
                pairs.append_pair("ids", &result);
            }
        }
    }
}

pub(crate) fn http_request<'a>(
    client: &Client<HttpsConnector<HttpConnector>>,
    endpoint: Endpoint,
    params: Vec<Param<'a>>
) -> HttpReturn<String, APIError> {
    let mut url = Url::parse(&format!("{}{}", &API_ROOT, endpoint.as_str()))
        .expect("Fatal error parsing api URL");
    let mut request = Request::new(Method::Get, Uri::default());

    preprocess(&mut url, &mut request, params);

    request.set_uri(Uri::from_str(&url.into_string()).expect("Fatal error converting constructed URL to Hyper's Uri type."));

    let work = client
        .request(request)
        .and_then(|res|
            res.body().concat2().map(|chunk| String::from_utf8_lossy(&chunk).to_string())
        )
        .map_err(|_| APIError::InternalHTTP);

    #[cfg(feature = "futures-boxed")]
    Box::new(work)
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
