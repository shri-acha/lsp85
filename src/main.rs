mod frontend;
mod server;

// use frontend::lexer::Lexer;
// use frontend::parser::{Parser,Node};
// use frontend::token::{Token, TokenType,Location};
// use frontend::utils::files::get_source_buffer;

use lsp_server::{ExtractError, Message, Notification, Response};
use lsp_types::{
    request::{Completion, HoverRequest},
    SignatureHelpParams,
};
use server::{handlers, lsp85};
use std::error::Error;

use crate::server::handlers::{diagnostic_handler, signature_help_handler};

pub fn main() -> Result<(), Box<dyn Error>> {
    let lsp = lsp85::build()
        .stdio()
        .enable_hover()
        .enable_completion()
        .enable_diagnostics()
        .enable_signature_help()
        .initialize();

    let lsp = match lsp {
        Ok(lsp) => lsp,
        Err(e) => {
            eprintln!("init failed: {:?}", e);
            return Err(e);
        }
    };

    let conn = match lsp.conn.as_ref() {
        Some(conn) => conn,
        None => {
            eprintln!("no conn");
            return Err("no conn".into());
        }
    };

    for msg in &conn.receiver {
        eprintln!("Message incoming: {:?}", msg);
        match msg {
            Message::Request(req) => {
                let down = match conn.handle_shutdown(&req) {
                    Ok(true) => true,
                    Ok(false) => false,
                    Err(e) => {
                        eprintln!("error: {:?}", e);
                        false
                    }
                };
                if down {
                    eprintln!("shutting down!");
                    return Ok(());
                }
                eprintln!("got request: {:?}", req);

                let _ = lsp_router!(req,lsp,{
                    Completion=>handlers::completion_handler,
                    HoverRequest=>handlers::hover_handler,
                });
            }
            Message::Response(rs) => {
                eprintln!("response: {:?}", rs);
            }
            Message::Notification(n) => {
                match &n {
                    Notification { method, params } => {
                        if *method == String::from("textDocument/didSave") {
                            eprintln!("Document saved!, running diagnostics!");
                            let result = diagnostic_handler(params)?;
                            conn.sender
                                .send(Message::Notification(lsp_server::Notification {
                                    method: "textDocument/publishDiagnostics".to_string(),
                                    params: result,
                                }))?;
                        } else if *method == String::from("textDocument/signatureHelp") {
                            let params: SignatureHelpParams =
                                serde_json::from_value(params.clone())?;
                            let result = signature_help_handler(params)?;
                            conn.sender
                                .send(Message::Notification(lsp_server::Notification {
                                    method: "textDocument/publishSignatureHelp".to_string(),
                                    params: result,
                                }))?;
                        } else {
                            eprintln!("unimplemented");
                        }
                    }
                }
                eprintln!("notification: {:?}", n);
            }
        }
    }

    if let Some(io_threads) = lsp.io_threads {
        if let Err(e) = io_threads.join() {
            eprintln!("Error joining IO threads: {:?}", e);
        }
    }

    Ok(())
}
// if let Some(source) = get_source_buffer("test_value.asm") {
//     // buffered reading
//     let mut ast_list: Vec<Option<Node>> = vec![];
//     for (line_no, read_buf) in source {
//         if let Ok(read_buf) = read_buf {
//             let mut l = Lexer::new(read_buf, line_no);
//             let mut tokns_buf: Vec<Token> = vec![];
//             tokns_buf.push(Token::new(0,TokenType::BOL,Location::new(0,0),String::from("BOL")));

//             for tok in l {
//                 tokns_buf.push(tok);
//             }
//             // println!("{:?}", tokns_buf);

//             let mut p = Parser::new(tokns_buf.into_iter());
//             ast_list.push(p.parse_expression());
//         } else {
//             println!("Error reading!");
//         }
//         println!("{:?}",ast_list);
//     }
// }
