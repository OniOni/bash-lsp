use serde_json::{Value};
use serde::Deserialize;
use lsp_types::{lsp_request, InitializeResult, ServerInfo, ServerCapabilities};
use lsp_types::request::Request as LSPRequest;

use crate::jsonrpc::Request;
use crate::utils::err_msg;

pub fn get_response(jsonrpc_req: Request) -> String {
    let json_req: Value = serde_json::from_str(&jsonrpc_req.body).unwrap();
    let method = json_req["method"].as_str().unwrap();

    let b = match method {
        <lsp_request!("initialize")>::METHOD => {
            let params =
                match <lsp_request!("initialize") as LSPRequest>::Params::deserialize(
                    &json_req["params"]
                 ) {
                    Ok(initParams) => initParams,
                    Err(err) => {
                        err_msg(format!("{err}"));
                        panic!();
                    },
                };

            serde_json::json!({
                "id": 1,
                "result": InitializeResult {
                    capabilities: ServerCapabilities::default(),
                    server_info: Some(ServerInfo {
                        name: String::from("bash-lsp"),
                        version: Some(String::from("0.0.1")),
                    }),
                }
            })
        },
        _ => serde_json::json!({
            "id": 1,
            "result": {
                "all": method,
            }
        }),
    };

    let body = b.to_string();
    let len = body.len();

    return format!("Content-Length: {len}\r\n\r\n{body}")
}
