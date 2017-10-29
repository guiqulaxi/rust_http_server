extern crate http_server;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    use std::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("new client!");
            }
            Err(e) => { /* connection failed */ }
        }
    }
      use http_server::server::Server;
      let server=Server::new(8080,"127.0.0.1",".");
      server.run();
    }
}
