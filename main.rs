use std::{fs, env};

fn main() {
    let fp: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&fp[1]);

    let mut lexer = Lexer::new(contents.unwrap());

    let tokens = lexer.lex();
    println!("{:?}", tokens);
}

#[derive(Debug)]
enum TokenKind {
    String,
    Equal,
    Identifier,
    Let
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    literal: String
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Self {
        Self {
            kind: kind,
            literal: literal
        }
    }
}

struct Lexer {
    contents: Vec<char>,
    counter: usize
}

impl Lexer {
    pub fn new(content: String) -> Self {
        Self {
            contents: content.chars().collect(),
            counter: 0
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while self.contents.len() > self.counter {

            if self.counter == self.contents.len() - 1 {
                break;
            }
            
            match self.cur_char() {
                _ if self.cur_char().is_alphabetic() => {
                    let mut buf = String::new();
                    while self.cur_char().is_alphabetic() {
                        buf.push(self.cur_char());
                        self.adv();
                    }
                    if buf == "let" {
                        tokens.push(Token::new(TokenKind::Let, buf));
                    } else {
                        tokens.push(Token::new(TokenKind::Identifier, buf));
                    }
                }

                _ if self.cur_char() == '\'' => {
                    self.adv();
                    let mut buf = String::new();
                    while self.cur_char() != '\'' {
                        buf.push(self.cur_char());
                        self.adv();
                    }
                    tokens.push(Token::new(TokenKind::String, buf));
                }

                '=' => {
                    tokens.push(Token::new(TokenKind::Equal, "=".to_owned()));
                    self.adv();
                }

                ' ' => {
                    self.adv();
                }

                _ => {
                    self.adv();
                }
            }
        }
        return tokens;
    }

    fn cur_char(&self) -> char {
        let c = self.contents.get(self.counter);
        
        match c {
            Some(value) => {
                *c.unwrap()
            }
            None => {
                '!'
            }
        }
    }

    fn adv(&mut self) {
        self.counter += 1;
    }
}