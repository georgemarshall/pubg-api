extern crate dotenv;
extern crate env_logger;
extern crate failure;
extern crate pubg_api;
extern crate tokio_core;

use failure::Error;
use tokio_core::reactor;

use pubg_api::Pubg;
use pubg_api::status::Status;

fn query_status() -> Result<Status, Error> {
    let mut core = reactor::Core::new()?;
    let pubg_api = Pubg::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        None,
        &core.handle(),
    );

    let work = pubg_api.status().get();

    core.run(work)
}

fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    env_logger::init();

    match query_status() {
        Ok(status) => println!("{:#?}", status),
        Err(error) => println!("Error {:#?}", error),
    }
}
