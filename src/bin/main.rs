use rusty_server::{*, server::Content};
fn main() {
    let mut myserver = server::Server::new();
    myserver.bind("7878");

    myserver.get("/", Content::Addr("public/index.html".to_string()));
    myserver.get("/sleep", Content::Addr("public/sleep.html".to_string()));

    myserver.get("/maris", Content::Fn(|thing| {
        String::from("chaina")
        }));


    myserver.start();
}
