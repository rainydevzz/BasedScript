use std::{fs, env};

fn main() {
    let fp: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&fp[1]);

    let stripped = strip(contents.unwrap());

    let mut lexer = Lexer::new(stripped);

    let tokens = lexer.lex();
    let mut parser = Parser::new(tokens);
    parser.parse();
}

fn strip(contents: String) -> String {
    let mut cloned = contents.clone();
    cloned = cloned.replace('\n', " ");
    cloned = cloned.replace('\r', "");
    return cloned;
}

#[derive(Debug)]
#[derive(PartialEq)]
enum TokenKind {
    String,
    Equal,
    Identifier,
    Let,
    Print,
    Number
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

#[derive(Debug)]
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

            if self.counter == self.contents.len() - 1 || self.counter == self.contents.len() {
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
                    } else if buf == "print" {
                        if self.cur_char() == '(' {
                            buf = "".to_string();
                            self.adv();
                            while self.cur_char() != ')' {
                                buf.push(self.cur_char());
                                self.adv();
                            }
                            tokens.push(Token::new(TokenKind::Print, buf));
                            self.adv();
                        }
                    } else {
                        tokens.push(Token::new(TokenKind::Identifier, buf));
                        self.adv();
                    }
                }

                _ if self.cur_char().is_numeric() => {
                    let mut buf = String::new();
                    while self.cur_char().is_numeric() {
                        buf.push(self.cur_char());
                        self.adv();
                    }
                    tokens.push(Token::new(TokenKind::Number, buf));
                }

                '\'' => {
                    let mut buf = String::new();
                    self.adv();
                    while self.cur_char() != '\'' {
                        buf.push(self.cur_char());
                        self.adv();
                    }
                    tokens.push(Token::new(TokenKind::String, buf));
                    self.adv();
                }

                '"' => {
                    let mut buf = String::new();
                    self.adv();
                    while self.cur_char() != '"' {
                        buf.push(self.cur_char());
                        self.adv();
                    }
                    tokens.push(Token::new(TokenKind::String, buf));
                    self.adv();
                }

                '=' => {
                    tokens.push(Token::new(TokenKind::Equal, "=".to_owned()));
                    self.adv();
                }

                ' ' => {
                    self.adv();
                }

                '/' => {
                    self.adv();
                    while self.cur_char() != '/' {
                        self.adv();
                    }
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
            Some(_value) => {
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

#[derive(Debug)]
#[allow(dead_code)] // to be used later
struct Variable {
    name: String,
    value: String
}

impl Variable {
    pub fn new(name: String, value: String) -> Self {
        Self {name: name, value: value}
    }
}

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    counter: usize,
    stack: Vec<Variable>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens: tokens, counter: 0, stack: Vec::new() }
    }

    pub fn parse(&mut self) {
        while self.counter < self.tokens.len() {
            let cur_tok = &self.tokens[self.counter];

            match cur_tok.kind {
                TokenKind::Let => {
                    if matches!(self.tokens[self.counter + 1].kind, TokenKind::Identifier) {
                        self.adv();
                    } else {
                        panic!("expected identifier.");
                    }
                }

                TokenKind::Identifier => {
                    if matches!(self.tokens[self.counter - 1].kind, TokenKind::Let) && matches!(self.tokens[self.counter + 1].kind, TokenKind::Equal) {
                        self.adv();
                    } else {
                        panic!("no 'let' token found");
                    }
                }

                TokenKind::Equal => {
                    if matches!(self.tokens[self.counter - 1].kind, TokenKind::Identifier) && (matches!(self.tokens[self.counter + 1].kind, TokenKind::String) || matches!(self.tokens[self.counter + 1].kind, TokenKind::Number)) {
                        self.adv();
                    } else {
                        panic!("no identifier token found");
                    }
                }

                TokenKind::String => {
                    if matches!(self.tokens[self.counter - 1].kind, TokenKind::Equal) {
                        self.stack.push(Variable::new(self.tokens[self.counter - 2].literal.clone(), cur_tok.literal.clone()));
                        self.adv();
                    } else {
                        panic!("literal with no assignment");
                    }
                }

                TokenKind::Number => {
                    if matches!(self.tokens[self.counter - 1].kind, TokenKind::Equal) {
                        self.stack.push(Variable::new(self.tokens[self.counter - 2].literal.clone(), cur_tok.literal.clone()));
                        self.adv();
                    } else {
                        panic!("literal with no assignment");
                    }
                }

                TokenKind::Print => {
                    let tok_res = self.stack.iter().find(|v| v.name == cur_tok.literal);
                    if !tok_res.is_none() {
                        println!("{}", tok_res.unwrap().value);
                        self.adv();
                    } else if cur_tok.literal.starts_with("'") || cur_tok.literal.starts_with("\"") {
                        println!("{}", cur_tok.literal);
                        self.adv();
                    } else if cur_tok.literal.chars().collect::<Vec<char>>()[0].is_numeric() {
                        println!("{}", cur_tok.literal);
                        self.adv();
                    } else if tok_res.is_none() {
                        panic!("variable {} not found", cur_tok.literal);
                    }
                }
            }
        }
    }

    fn adv(&mut self) {
        self.counter += 1;
    }
}