use crate::frontend::token::TokenType;
use crate::{frontend::lexer::Lexer, server::completion_items::get_completion_items};
use crate::server::utils::get_documentation;
use lsp_server::{RequestId};
use lsp_types::{
    CompletionParams, CompletionResponse, HoverParams,
};

use crate::frontend::utils::files::get_source_line;

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
            let lexer = Lexer::new(line,position.line as usize);
            lexer
                .filter(|tok| {matches!(tok.tok_type, TokenType::OPERATION | TokenType::REGISTER | TokenType::IMM_VALUE)})
                .find(|tok| {
                        let tok_start = tok.location.col - tok.offset;
                        let tok_end = tok.location.col;
                        (col >= tok_start) && (col < tok_end)
                    })
        })
        .map(|token| token);
    
    if let Some(ref word) = hovered_word {
        if word.tok_type != TokenType::IMM_VALUE {
            let info = hovered_word.and_then(|word| {
                get_documentation()
                    .into_iter()
                    .find(|i| i.label == word.tok_literal)
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
        }else{
            let hover_result = lsp_types::Hover{
                contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent { 
                    kind: lsp_types::MarkupKind::Markdown,
                    value: format!("**Immediate value**\n\n{}",word.tok_literal) }),
                range: None,
            };
            serde_json::to_value(&hover_result)
        }
    }else{
            let hover_result = lsp_types::Hover{
                contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent { 
                    kind: lsp_types::MarkupKind::Markdown,
                    value: format!("No information available!"),
                }),
                range: None,
            };
            serde_json::to_value(&hover_result)
    }

}
