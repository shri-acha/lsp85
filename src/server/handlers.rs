use core::fmt;

use crate::server::completion_items::get_completion_items;
use crate::server::utils::get_documentation;
use lsp_server::{RequestId};
use lsp_types::{
    CompletionParams, CompletionResponse, HoverParams,
};

use crate::frontend::lexer::Lexer;
use crate::frontend::parser::{Parser,Node};
use crate::frontend::token::{Token, TokenType,Location};
use crate::frontend::utils::files::{get_source_buffer,get_source_line};

pub fn completion_handler(
    id: &RequestId,
    params: CompletionParams,
) -> Result<serde_json::Value, serde_json::Error> {
    eprintln!("got completion request #{}: {:?}", id, params);    
    let result = CompletionResponse::Array(get_completion_items());
    serde_json::to_value(&result)
}


pub fn hover_handler(
    _id: &RequestId,
    params: HoverParams,
) -> Result<serde_json::Value, serde_json::Error> {

    let file_name = params.text_document_position_params.text_document.uri.path().as_str();
    let position = params.text_document_position_params.position;

    let hovered_word = get_source_line(file_name, position.line)
        .and_then(|source| source.ok())
        .and_then(|line| {
            let col = position.character as usize;
            let start = line[..col].rfind(|c: char| !c.is_alphanumeric()).map(|i| i + 1).unwrap_or(0);
            let end = line[col..].find(|c: char| !c.is_alphanumeric()).map(|i| i + col).unwrap_or(line.len());
            Some(line[start..end].trim().to_uppercase())
        });

    let info = hovered_word.and_then(|word| {
        get_documentation()
            .into_iter()
            .find(|i| i.label == word)
    });

    match info {
        Some(info) => {
            let hover_result = lsp_types::Hover {
                contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                    kind: lsp_types::MarkupKind::Markdown,
                    value: format!("**{}**\n\n{}", info.detail, info.documentation),
                }),
                range: None,
            };
            serde_json::to_value(&hover_result)
        }
        None => serde_json::to_value(Option::<lsp_types::Hover>::None),
    }
}
