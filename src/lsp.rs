use serde_json::json;
use crate::jsonrpc::Request;

pub fn get_response(req: Request) -> String {
    let body = json!({
        "id": 1,
        "result": {
            "all": "good"
        }
    }).to_string();
    let len = body.len();

    return format!("Content-Length: {len}\r\n\r\n{body}")
}
