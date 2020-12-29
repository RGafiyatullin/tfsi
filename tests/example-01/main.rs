#![recursion_limit = "16"]

#[macro_use]
extern crate tfsi;

use tfsi::prelude::*;

mod capabilities;
mod config;
mod di;

#[test]
fn t01_01() {
    let injector = Injector::create()
        // ConfigComponent:
        //  Requires:
        //  - `&AppConfig`
        //  Provides:
        //  - `DbConfig`
        //  - `HttpBindConfig`
        .with_component(di::ConfigComponent::default())
        // DbComponent
        //  Requires:
        //  - `DbConfig`
        //  Provides:
        //  - `Db`
        .with_component(di::DbComponent::default())
        // HttpComponent
        //  Requires:
        //  - `HttpBindConfig`
        //  Provides:
        //  - `HttpServer`
        .with_component(di::HttpComponent::default())
        // CapsComponent
        //  Requires:
        //  - `HttpServer`
        //  - `Db`
        //  Provides:
        //  - `(Db, HttpServer)`
        // .bootstrap_component::<di::CapsComponent, _, _>(providers::ProvideNil)
        // A value for `AppConfig` is provided
        .with_value(config());

    // The injector at compile time decides how to provide a `(Db, HttpServer)`:
    // - `(Db, HttpServer)` via CapsComponent
    //  - `Db` via DbComponent
    //   - `DbConfig` via `ConfigComponent`
    //    - `AppConfig` via ad-hoc provider
    //  - `HttpServer` via HttpComponent
    //   - `HttpBindConfig` via `ConfigComponent`
    //    - `AppConfig` via ad-hoc provider
    // let (db, http_server): (capabilities::Db, capabilities::HttpServer) = injector.provide();

    let db: capabilities::Db = injector.provide();
    let http_server: capabilities::HttpServer = injector.provide();
    println!("http_server: {:?}", http_server);
    println!("db: {:?}", db);
}

fn config() -> config::AppConfig {
    config::AppConfig {
        http_bind: Default::default(),
        db: Default::default(),
    }
}
