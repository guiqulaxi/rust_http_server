extern crate http_server;
use http_server::server::Server;


fn main() {

   
    let server = Server::new( "127.0.0.1:8080", ".");
    server.run();
}
