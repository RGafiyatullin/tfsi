#![recursion_limit = "16"]

#[macro_use]
extern crate tfsi;
pub use ::tfsi::prelude as di_prelude;

mod app;
mod app_config;
mod capabilities;
mod di;

#[test]
fn main_run() {
    use crate::app::App;
    use crate::app::AppComponent;
    use crate::app_config::AppConfig;
    use crate::capabilities::real_capabilities::*;
    use crate::di::*;
    use crate::di_prelude::*;

    println!("MAIN-RUN");

    let app_config = AppConfig {
        db: RealDbConfig {},
    };

    let injector = Injector::create()
        .with_component(AppComponent::<RealCapabilities>::default())
        .with_component(RealDbComponent)
        .with_component(RealDbConfigComponent)
        .with_value(app_config);

    let app: App<_> = injector.provide();
    app.run();
}

#[test]
fn main_test() {
    use crate::app::App;
    use crate::app::AppComponent;
    use crate::capabilities::test_capabilities::*;
    use crate::di_prelude::*;

    println!("MAIN-TEST");

    let injector = Injector::create()
        .with_value(TestDb::default())
        .with_component(AppComponent::<TestCapabilities>::default());

    let app: App<_> = injector.provide();
    app.run();
}
