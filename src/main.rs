mod routes;
mod db;

use actix_web::{App, HttpServer, web::ServiceConfig};
use crate::routes::{index, rss}; 

fn init(cfg: &mut ServiceConfig) {
    index::reg(cfg);
    rss::reg(cfg);
}

fn main() {
    match HttpServer::new( || App::new().configure(init) )
        .bind("0.0.0.0:1337")
        .expect("Cannot launch on 1337")
        .run() {
            Ok(_) => { println!("Leftshift is online!"); },
            Err(e) => { eprintln!("{}", e); }
        }
}
