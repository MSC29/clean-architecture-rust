use actix_web::dev::Server;
use std::net::TcpListener;

pub mod adapters;
pub mod application;
pub mod domain;
pub mod infrastructure;

extern crate dotenv;
extern crate log;

#[macro_use]
extern crate diesel;
extern crate r2d2;

pub fn run(listener: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    infrastructure::server(listener, db_name)
}
