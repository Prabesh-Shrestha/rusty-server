
#[warn(dead_code)]
pub struct Buffer {
    req: String,
    host: String,
    connection: String,
    accept_encoding: Vec<String>,
}
impl Buffer {
    pub fn new(
        req: String,
        host: String,
        connection: String,
        accept_encoding: Vec<String>,
    ) -> Buffer {
        Buffer {
            req,
            host,
            connection,
            accept_encoding,
        }
    }
    pub fn parse_stream(buffer: [u8; 1024]) {}
}


