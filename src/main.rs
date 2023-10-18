use std::collections::HashMap;
use std::io::{self, Read, Write};
use serde_json::json;
use std::str::FromStr;

use std::thread;

fn write(stderr: &io::Stderr, mut message: String) -> io::Result<()> {
    let mut handle = stderr.lock();
    message.push_str("\n");
    handle.write_all(message.as_ref());
    handle.flush();

    Ok(())
}

struct Response {
    headers: HashMap<String, String>,
    body: String,
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Request {
    headers: HashMap<String, String>,
    body: String,
}

impl Request {

    fn new() -> Self {
        Self {
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    fn process_headers(&mut self, buffer: &String) -> String {
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
                    let stderr = io::stderr();
                    write(&stderr, format!("Empty."));
                }
            };
        }
        return ret;
    }
}

fn get_response(req: Request) -> String {
    let body = json!({
        "id": 1,
        "result": {
            "all": "good"
        }
    }).to_string();
    let len = body.len();

    return format!("Content-Length: {len}\r\n\r\n{body}")
}

fn main() {
    let mut stdin = io::stdin();
    let stderr = io::stderr();

    let mut read_buff = [0; 256];
    let mut no_bytes = true;
    let mut buffer = String::new();
    let mut request = Request::new();

    write(&stderr, String::from("Starting loop"));
    loop {
        match stdin.read(&mut read_buff) {
            Ok(n) => {
                if n > 0 || no_bytes {
                    let pkg = String::from_utf8(read_buff.to_vec()).unwrap();
                    buffer += &pkg;

                    if buffer.contains("\r\n\r\n") {
                        let rest = request.process_headers(&buffer);

                        if rest.len() > 0 {
                            buffer = rest;
                        } else {
                            buffer.clear();
                        }
                    }

                    let len = buffer.len();
                    let expected_len = usize::from_str(&request.headers["Content-Length"]).unwrap();
                    if len >= expected_len {
                        request.body = String::from(buffer.clone().get(..expected_len).unwrap());
                        buffer.clear();
                    }

                    if request.body.len() > 0 {

                        thread::spawn(|| {
                            let stdout = io::stdout();
                            let mut handle = stdout.lock();
                            let resp = get_response(request);

                            handle.write_all(resp.as_ref());
                            handle.flush();
                        });
                        request = Request::new();
                    }

                    if no_bytes {
                        no_bytes = false;
                    }

                    read_buff = [0; 256];
                } else {
                    write(&stderr, String::from("error: nothing to read"));
                    break;
                }
            }
            Err(error) => {
                write(&stderr, String::from("error: {error}"));
                break;
            }
        }
    }
}
