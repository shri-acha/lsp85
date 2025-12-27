use crate::server::completion_items::get_completion_items;
use lsp_server::{RequestId};
use lsp_types::{
    CompletionParams, CompletionResponse, HoverParams,
};

pub fn completion_handler(
    id: &RequestId,
    params: CompletionParams,
) -> Result<serde_json::Value, serde_json::Error> {
    eprintln!("got completion request #{}: {:?}", id, params);
    
    let result = CompletionResponse::Array(get_completion_items());
    serde_json::to_value(&result)
}

pub fn hover_handler(
    id: &RequestId,
    params: HoverParams,
) -> Result<serde_json::Value, serde_json::Error> {
    eprintln!("hovr request {}: {:?}", id, params);

    let hover_result = lsp_types::Hover {
        contents: lsp_types::HoverContents::Scalar(lsp_types::MarkedString::String(
            "dummy hover info".to_string(),
        )),
        range: None,
    };

    serde_json::to_value(&hover_result)
}
