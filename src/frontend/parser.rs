use crate::frontend::token::{Token, TokenType};
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug)]
pub struct Parser {
    tok_stream: Peekable<IntoIter<Token>>,
}
impl Parser {
    pub fn new(tok_stream: IntoIter<Token>) -> Self {
        Self {
            tok_stream: tok_stream.peekable(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Tree {
    pub l_child: Option<Node>,
    pub r_child: Option<Node>,
}

impl Tree {
    pub fn default() -> Self {
        Self {
            l_child: None,
            r_child: None,
        }
    }

    pub fn new(l_child: Option<Node>, r_child: Option<Node>) -> Self {
        Self { l_child, r_child }
    }
}
#[derive(Debug, PartialEq)]
pub struct Node {
    pub value: Token,
    pub branch: Box<Tree>,
}

impl Node {
    pub fn new(tok_val: Token, branch: Box<Tree>) -> Self {
        Self {
            value: tok_val,
            branch,
        }
    }
}

impl Parser {
    pub fn parse_expression(&mut self) -> Option<Node> {
        if let Some(peeked_token) = self.tok_stream.peek() {
            println!("parse_expression() called! {:?}", peeked_token);
            match peeked_token {
                Token {
                    tok_type: TokenType::OPERATION,
                    ..
                } => self.parse_operation(),
                Token {
                    tok_type: TokenType::REGISTER,
                    ..
                } => {
                    println!("unexpected placement of register!");
                    None
                }
                Token {
                    tok_type: TokenType::EOF,
                    ..
                } => None,
                _ => {
                    self.tok_stream.next();
                    self.parse_expression()
                }
            }
        } else {
            None
        }
    }
    pub fn parse_operation(&mut self) -> Option<Node> {
        let mut l_child: Node;
        if let Some(peeked_token) = self.tok_stream.peek() {
            l_child = Node::new(peeked_token.clone(), Box::new(Tree::default()));
        } else {
            return None;
        }
        self.tok_stream.next();
        if let Some(peeked_token) = self.tok_stream.peek() {
            match peeked_token {
                Token {
                    tok_type: TokenType::REGISTER,
                    ..
                } => {
                    l_child.branch.l_child = self.parse_operand();
                    l_child.branch.r_child = self.parse_operand();
                    Some(l_child)
                }
                _ => Some(l_child),
            }
        } else {
            Some(l_child)
        }
    }
    pub fn parse_operand(&mut self) -> Option<Node> {
        if let Some(peeked_token) = self.tok_stream.peek() {
            match peeked_token {
                Token {
                    tok_type: TokenType::REGISTER,
                    ..
                } | Token {
                    tok_type: TokenType::IMM_VALUE,
                    ..
                } => {
                    let token_buffer = peeked_token.clone();
                    self.tok_stream.next();
                    Some(Node::new(token_buffer, Box::new(Tree::default())))
                }
                Token {
                    tok_type: TokenType::COMMA_DELIM,
                    ..
                } => {
                    self.tok_stream.next();
                    self.parse_operand()
                }
                _ => None,
            }
        } else {
            None
        }
    }
}
