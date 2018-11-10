use crate::prelude::*;

/// `UrlParam`s represent the various options passable to the Guild Wars 2 API.
/// Used internally to provide a consistent interface for passing options during an API call,
/// based on need.
#[derive(Debug)]
pub(crate) enum Param<'a> {
    Locale(&'a Locale),
    StrIds(&'a [&'a str]),
    IntIds(&'a [u64]),
    Quantity(u64),
    AuthToken(&'a str),
}