use std::net::Ipv4Addr;
use std::env;
use warp::Filter;

mod content;
mod filters;
mod handlers;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "routes=info");
    }
    pretty_env_logger::init();

    let api = filters::routes();

    // View access logs by setting `RUST_LOG=todos`.
    let routes = api.with(warp::log("routes"));
    // Start up the server...
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(routes).run((Ipv4Addr::UNSPECIFIED, port)).await
}
