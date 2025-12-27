use crate::server::completion_items::get_completion_items;
use lsp_server::{ExtractError, Message, Notification, RequestId, Response};
use lsp_types::request::{Completion, HoverRequest};
use lsp_types::{
    CompletionParams, CompletionResponse, HoverParams,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn wasm_request_handler(req: JsValue) -> Result<JsValue, JsValue> {
    let msg = serde_wasm_bindgen::from_value(req)
        .map_err(|e| JsValue::from_str(&format!("Invalid request: {:?}", e)))?;
    match msg {
        Message::Request(req) => {
            // eprintln!("got request: {:?}", req);
            wasm_lsp_router!(req,{
                Completion=>wasm_completion_handler,
                HoverRequest=>wasm_hover_handler,
            });
            return Ok(format!("Request handled and routed!").into());
        }
        Message::Response(rs) => {
            // eprintln!("response: {:?}", rs);
            return Ok(format!("Response: {:?}", rs).into());
        }
        Message::Notification(n) => {
            match &n {
                Notification { method, .. } if *method == String::from("textDocument/didSave") => {
                    return Ok(format!("File saved!").into());
                }
                e => {
                    return Err(format!("Error in saving file!").into());
                }
            }
            return Ok(format!("notification: {:?}", n).into());
        }
    }
}

pub fn wasm_completion_handler(id: &RequestId, params: &CompletionParams) -> Result<JsValue, JsValue> {
    // eprintln!("got completion request #{}: {:?}", id, params);
    
    let result = CompletionResponse::Array(get_completion_items());
    let result = match serde_json::to_string(&result) {
        Ok(result) => result,
        Err(_) => "[ERROR] failed to convert JSON-2-String".to_string(),
    };
    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

pub fn wasm_hover_handler(id: &RequestId, params: &HoverParams) -> Result<JsValue, JsValue> {
    // eprintln!("hovr request {}: {:?}", id, params);

    let hover_result = lsp_types::Hover {
        contents: lsp_types::HoverContents::Scalar(lsp_types::MarkedString::String(
            "dummy hover info".to_string(),
        )),
        range: None,
    };

    let result = match serde_json::to_string(&hover_result) {
        Ok(result) => result,
        Err(_) => return Err("[ERROR] failed to convert JSON-2-String".to_string().into()),
    };
    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}
