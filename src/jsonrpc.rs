use std::io;
use std::collections::HashMap;

use crate::utils::err_msg;

pub struct Response {
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Request {
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {

    pub fn new() -> Self {
        Self {
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    pub fn process_headers(&mut self, buffer: &String) -> String {
        let mut ret = String::new();
        for line in buffer.split("\r\n") {
            let v = line.split(": ").collect::<Vec<_>>();
            match v[..] {
                [key, value] => {
                    self.headers.insert(key.to_string(), value.to_string());
                },
                [value] => {
                    ret += value;
                },
                [..] => {
                    ret += line;
                },
                [] => {
                    err_msg(format!("Empty."));
                }
            };
        }
        return ret;
    }
}
