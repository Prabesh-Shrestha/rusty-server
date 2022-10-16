use rusty_server::*;
fn main() {
    let mut myserver = server::Server::new();
    myserver.bind("7878");

    myserver.get("/", "public/index.html");
    myserver.get("/sleep", "public/sleep.html");
    myserver.start();
}
