pub mod adapters;
pub mod application;
pub mod domain;
pub mod infrastructure;

extern crate dotenv;
extern crate log;

#[macro_use]
extern crate diesel;
extern crate r2d2;

use std::net::TcpListener;

use actix_web::dev::Server;

pub fn run(listener: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    infrastructure::server(listener, db_name)
}
