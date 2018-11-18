extern crate gw2rs;
extern crate futures;
extern crate tokio_core;

use tokio_core::reactor::Core;
use futures::Future;
use gw2rs::prelude::*;

fn main() {
    let mut reactor = Core::new().unwrap();
    let gw2_client = GW2::new(String::from("FE630B14-6CBF-D845-9918-641093FF4361C0508B84-B0C6-42A5-A995-3A855F079680"), Locale::EN, reactor.handle());
    let token_info = gw2_client.token_info().map(|res| {
        println!("Your token is: {} - {}", &res.name, &res.id);
    });
    reactor.run(token_info).expect("Fatal error when running the API call future");
}