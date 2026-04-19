use crate::frontend::token::{Location, Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    chars: Vec<char>,         // pre-computed characters for O(1) access
    pub ch: char,             // current literal
    pub curr_position: usize, // current position
    pub read_position: usize, // next position
    pub location: Location,   // current location
}

impl Lexer {
    pub fn new(source: String, line_no: usize) -> Self {
        let chars: Vec<char> = source.chars().collect();
        let ch = chars.first().copied().expect("source of size <1?");
        Self {
            chars,
            ch,
            curr_position: 0,
            read_position: 1,
            location: Location {
                row: line_no,
                col: 0,
            },
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        match self.ch {
            c if self.ch.is_alphabetic() => {
                // identifier
                return Some(self.read_identifier());
            }
            c if self.ch.is_numeric() => {
                return Some(self.read_immediate());
            }
            ',' => {
                self.consume();
                return Some(Token::new(
                    1,
                    TokenType::COMMA_DELIM,
                    self.location,
                    String::from(','),
                ));
            }
            ' ' => {
                self.consume();
                return self.next();
            }
            '\n' => {
                self.consume();
                let buf_token = Some(Token::new(
                    1,
                    TokenType::EOL,
                    self.location,
                    String::from('\n'),
                ));
                self.location.col = 0;
                self.location.row += 1;

                return buf_token;
            }
            '\0' => {
                return None;
            }
            _ => {
                self.consume();
                return Some(Token::new(
                    1,
                    TokenType::ILLEGAL,
                    self.location,
                    String::from('\0'),
                ));
            }
        }
    }
}
impl Lexer {
    pub fn consume(&mut self) {
        if self.read_position >= self.chars.len() {
            self.ch = '\0';
        } else {
            self.ch = self.chars[self.read_position];
        }
        self.curr_position = self.read_position;
        self.read_position = self.curr_position + 1;
        self.location.col += 1;
    }
    pub fn read_identifier(&mut self) -> Token {
        let mut identifier_buf = String::new();
        while self.ch.is_alphabetic() {
            identifier_buf.push(self.ch);
            self.consume();
        }
        Token::new(
            identifier_buf.len(),
            get_identifier_token(&identifier_buf),
            self.location,
            identifier_buf,
        )
    }
    pub fn read_immediate(&mut self) -> Token {
        let mut immediate_buf = String::new();

        //Support for hex digits
        while self.ch.is_ascii_hexdigit() {
            immediate_buf.push(self.ch);
            self.consume();
        }

        //H suffix handling Eg: 123AH
        if self.ch == 'H' {
            immediate_buf.push(self.ch);
            self.consume();
        }
        Token::new(
            immediate_buf.len(),
            TokenType::IMM_VALUE,
            self.location,
            immediate_buf,
        )
    }
}
fn get_identifier_token(identifier_lit: &str) -> TokenType {
    match identifier_lit {
        "ADD" | "SUB" | "MOV" | "MVI" | "LXI" | "PUSH" | "POP" | "INR" | "DCR" | "DAD" | "LDAX"
        | "STAX" => TokenType::OPERATION,
        "A" | "B" | "C" | "D" | "E" | "PSW" | "H" | "L" | "SP" => TokenType::REGISTER,
        _ => TokenType::ILLEGAL,
    }
}

#[cfg(test)]
mod tests {

    use super::Lexer;
    use crate::frontend::token::{Location, Token, TokenType};
    #[test]
    fn imm_test() {
        let source = String::from("MVI A,05H\n");
        let mut l = Lexer::new(source, 0);
        let mut tokens: Vec<Token> = vec![];
        for token in l {
            tokens.push(token);
        }

        assert_eq!(
            vec![
                Token::new(
                    3,
                    TokenType::OPERATION,
                    Location::new(0, 3),
                    "MVI".to_string()
                ),
                Token::new(1, TokenType::REGISTER, Location::new(0, 5), "A".to_string()),
                Token::new(
                    1,
                    TokenType::COMMA_DELIM,
                    Location::new(0, 6),
                    ",".to_string()
                ),
                Token::new(
                    3,
                    TokenType::IMM_VALUE,
                    Location::new(0, 9),
                    "05H".to_string()
                ),
                Token::new(1, TokenType::EOL, Location::new(0, 10), "\n".to_string())
            ],
            tokens
        );
    }

    #[test]
    fn reg_pair() {
        let source = String::from("MVI A,SP\n");
        let mut l = Lexer::new(source, 0);
        let mut tokens: Vec<Token> = vec![];
        for token in l {
            tokens.push(token);
        }

        assert_eq!(
            vec![
                Token::new(
                    3,
                    TokenType::OPERATION,
                    Location::new(0, 3),
                    "MVI".to_string()
                ),
                Token::new(1, TokenType::REGISTER, Location::new(0, 5), "A".to_string()),
                Token::new(
                    1,
                    TokenType::COMMA_DELIM,
                    Location::new(0, 6),
                    ",".to_string()
                ),
                Token::new(
                    2,
                    TokenType::REGISTER,
                    Location::new(0, 8),
                    "SP".to_string()
                ),
                Token::new(1, TokenType::EOL, Location::new(0, 9), "\n".to_string())
            ],
            tokens
        );
    }
}
