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