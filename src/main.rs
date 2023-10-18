use std::io::{self, Read, Write};
use std::str::FromStr;

use std::thread;

use crate::jsonrpc::Request;
use crate::utils::err_msg;
use crate::lsp::get_response;

pub mod jsonrpc;
pub mod utils;
pub mod lsp;

fn main() {
    let mut stdin = io::stdin();

    let mut read_buff = [0; 256];
    let mut no_bytes = true;
    let mut buffer = String::new();
    let mut request = Request::new();

    err_msg(String::from("Starting loop"));
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
                    err_msg(String::from("error: nothing to read"));
                    break;
                }
            }
            Err(error) => {
                err_msg(String::from("error: {error}"));
                break;
            }
        }
    }
}
