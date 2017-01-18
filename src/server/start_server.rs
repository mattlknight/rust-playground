use nickel::{Nickel, Router};
use hyper::net::{Openssl};
use std::net::{SocketAddrV4, Ipv4Addr};

use ::req_logger;
use super::routes::add_route;


lazy_static! {
    static ref IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    static ref PORT: u16 = 8080;
    static ref ADDRESS: SocketAddrV4 = SocketAddrV4::new(*IP, *PORT);
}

/// Start an HTTPS nickel.rs server
pub fn start_server() {
    let mut server = Nickel::new();
    let mut router: Router = Nickel::router();

    server.utilize(req_logger::logger_fn);

    add_route(&mut router);

    server.utilize(router);

    let ssl = match Openssl::with_cert_and_key("keys/server.crt", "keys/server.key") {
        Ok(ssl) => ssl,
        Err(err) => panic!("Failed to open SSL keys from target/*/keys/; {:?}", err),
    };
    let listening = match server.listen_https(*ADDRESS, ssl) {
        Ok(srv) => srv,
        Err(err) => panic!("Failed to start Nickel Server; {:?}", err),
    };

    println!("Listening on: {:?}", listening.socket());
}
