// import the necessay library required
// use rusty_server::{*, server::Content};
use rusty_server::server::server::*;
use rusty_server::server::*;

fn main() {
    // creating the server instance
    let mut myserver = server::Server::new();
    // binding the server to a port
    // in this case localhost:7878
    myserver.bind("7878");

    myserver.get("/", Content::Addr("public/index.html".to_string()));
    myserver.get("/sleep", Content::Addr("public/sleep.html".to_string()));

    myserver.get("/maris", Content::Fn(|header| header.to_string()));

    myserver.start();
}
