# GW2 RS
[![Build Status](https://travis-ci.org/Sunspar/gw2rs.svg?branch=master)](https://travis-ci.org/Sunspar/gw2rs)[![Build status](https://ci.appveyor.com/api/projects/status/hou7p7nlae6a7rmk/branch/master?svg=true)](https://ci.appveyor.com/project/Sunspar/gw2rs/branch/master)

GW2RS is an api wrapper around the ArenaNet Guild Wars 2 API, for Rust.

It makes heavy use of futures, in part due to the hyper backend. wrapped api calls return the `GW2Result<T>` type, which is just an alias for boxed futures with the crate's custom `APIError` type.

In general, you can use the library by first spinning up a tokio core, then passing a handle into the GW2 struct. This is necessary due to the HTTP calls being offloaded to hyper, and part of the instantiation of that code needs a handle to the event loop that everything will eventually run.

# Examples / Demo
Feel free to check the `tests/sanity-check-*.rs` files for short, basic examples on using endpoints. If you're familiar with the current workflow for Futures, then it's pretty in line with other libraries (e.g you call a function that gives you a future, which you then run on an executor.)

```rust
extern crate gw2rs;
extern crate futures;
extern crate tokio_core;

use tokio_core::reactor::Core;
use futures::Future;
use gw2rs::prelude::*;

fn main() {
    let mut reactor = Core::new().unwrap();
    let gw2_client = GW2::new(String::from("B5A035FF-EBE0-7A46-B615-F910431D9D6F190E6C87-2FCF-4EA3-81BD-79A73A7C9704"), Locale::EN, reactor.handle());
    
    let token_info = gw2_client.token_info().map(|res| {
        println!("Your token is: {} - {}", res.name(), res.id());
    });
    
    reactor.run(token_info).unwrap();
}
```

# Crate Features
## HTTP Requests
By default, GW2RS uses request headers for both the Auth token and locale. You can override this functionality and store them in the URL as query params by using either the `query-auth` or `query-locale` features for individual items, or perhaps more commonly the `query-all` feature. You'll need to disable the default features or itll end up in both places (which shouldnt harm anything, but its extra work for nothing).

# Tests
Running tests is pretty straightforward. There are currently a few sanity check integration tests that should run a simple request/assert chain for each of the endpoints as well as a few basic checks for GW2 struct field interaction. You can run everything using `cargo test`.

The only caveat is that because some API calls require various permissions on an API key, you'll need to provide that. The tests expect the `GW2RS_TEST_API_KEY` to be set and will panic if missing.


# Implementing Endpoints
Implementing new endpoints is a fairly simple process:
- Update the Endpoint enum with the variant and path for the endpoint
- Create any relevant structs under the anet module
- Update the prelude module so that glob imports correctly pull in the new type
- Add exposed functions in the gw2 struct so users can deal with the endpoint
- Add sanity checks to the `test/sanity-check-apis.rs` integration test.

A fair amount of copy/paste is needed to implement the GW2 struct functions, but I cant find a simple way to macro that which doesnt invovle you typing out basically the same data in the macro, so its copy + paste + adjustments-as-needed for now.