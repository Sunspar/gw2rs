extern crate futures;
extern crate gw2rs;
extern crate tokio_core;
use gw2rs::prelude::*;
use tokio_core::reactor::Core;

#[test]
fn new() {
    let reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    assert_eq!(gw2_client.locale(), Locale::EN);
}

#[test]
fn locale() {
    let reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let l = gw2_client.locale();
    assert_eq!("en", l.to_str());
}

#[test]
fn set_locale() {
    let reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let mut gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    gw2_client.set_locale(Locale::DE);
    assert_eq!("de", gw2_client.locale().to_str());
}

#[test]
fn api_key() {
    let reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token.clone(), Locale::EN, reactor.handle());
    let api_key = gw2_client.api_key();
    assert_eq!(token, api_key);
}

#[test]
fn set_api_key() {
    let reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let mut gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let new_token =
        String::from("D2BCE439-B74C-4847-9A16-3A6BD6F1DB7EEDCE05C4-18BA-478F-8759-18C988FB460B");
    gw2_client.set_api_key(new_token.clone());
    assert_eq!(&new_token, gw2_client.api_key());
}
