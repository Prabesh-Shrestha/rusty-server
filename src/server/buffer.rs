use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

#[warn(dead_code)]
#[derive(Clone)]
pub struct Buffer {
    pub req: String,
    pub info: HashMap<String, String>,
}
impl Display for Buffer {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "request: {} \n{:#?}", self.req, self.info)
    }
}

impl Buffer {
    pub fn new(req: String, info: HashMap<String, String>) -> Buffer {
        Buffer { req, info }
    }
    pub fn parse_stream(buffer: [u8; 1024]) -> Buffer {
        let buffer = String::from_utf8_lossy(&buffer[..]).replace("\"", "");
        let mut buffer: Vec<&str> = buffer.split("\r\n").collect();

        let req = buffer.remove(0).to_string();
        let mut info: HashMap<String, String> = HashMap::new();

        for i in buffer {
            match i.split_once(": ") {
                Some((x, y)) => {
                    info.insert(x.to_string(), y.to_string());
                }
                None => continue,
                //TODO: HANDLE THIS
            }
        }
        Buffer::new(req, info)
    }
}
