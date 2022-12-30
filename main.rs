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
    let mut cloned = contents.to_string();
    cloned = cloned.replace('\n', " ");
    cloned = cloned.replace('\r', "");
    return cloned;
}

#[derive(Debug)]
enum TokenKind {
    String,
    Equal,
    Identifier,
    Let,
    Print,
    Number,
    Free,
    Plus,
    Dot,
    Semi
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
                        } else {
                            panic!("Print token found, but no (");
                        }
                    } else if buf == "free" {
                        if self.cur_char() == '(' {
                            buf = "".to_string();
                            self.adv();
                            while self.cur_char() != ')' {
                                buf.push(self.cur_char());
                                self.adv();
                            }
                            tokens.push(Token::new(TokenKind::Free, buf));
                            self.adv();
                        } else {
                            panic!("Free token found, but no (");
                        }
                    } else {
                        tokens.push(Token::new(TokenKind::Identifier, buf));
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

                '+' => {
                    tokens.push(Token::new(TokenKind::Plus, "+".to_owned()));
                    self.adv();
                }

                ' ' => {
                    self.adv();
                }

                '#' => {
                    self.adv();
                    while self.cur_char() != '#' {
                        self.adv();
                    }
                    self.adv();
                }

                '.' => {
                    tokens.push(Token::new(TokenKind::Dot, ".".to_owned()));
                    self.adv();
                }

                ';' => {
                    tokens.push(Token::new(TokenKind::Semi, ";".to_owned()));
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
                    } else if matches!(self.tokens[self.counter + 1].kind, TokenKind::Equal) {
                        let tok_res = self.stack.iter().position(|t| t.name == cur_tok.literal);
                        if !tok_res.is_none() {
                            self.stack[tok_res.unwrap()] = Variable::new(cur_tok.literal.to_string(), self.tokens[self.counter + 2].literal.to_string());
                            self.adv();
                        } else if tok_res.is_none() {
                            panic!("variable {} not found, must declare variable with 'let' before reassigning", cur_tok.literal);
                        }
                    } else if matches!(self.tokens[self.counter + 1].kind, TokenKind::Semi) && matches!(self.tokens[self.counter - 1].kind, TokenKind::Let) {
                        self.stack.push(Variable::new(cur_tok.literal.to_string(), "undefined".to_string()));
                        self.adv();
                    } else {
                        panic!("variable {} not found, must declare variable with 'let' before reassigning", cur_tok.literal);
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
                        self.stack.push(Variable::new(self.tokens[self.counter - 2].literal.to_string(), cur_tok.literal.to_string()));
                        self.adv();
                    } else {
                        panic!("literal with no assignment");
                    }
                }

                TokenKind::Number => {
                    if matches!(self.tokens[self.counter - 1].kind, TokenKind::Equal) && !matches!(self.tokens[self.counter + 1].kind, TokenKind::Plus) {
                        self.stack.push(Variable::new(self.tokens[self.counter - 2].literal.to_string(), cur_tok.literal.to_string()));
                        self.adv();
                    } else if matches!(self.tokens[self.counter + 1].kind, TokenKind::Plus) && matches!(self.tokens[self.counter + 2].kind, TokenKind::Number) {
                        if matches!(self.tokens[self.counter - 1].kind, TokenKind::Equal) {
                            let eval = cur_tok.literal.parse::<i32>().unwrap() + self.tokens[self.counter + 2].literal.parse::<i32>().unwrap();
                            self.stack.push(Variable::new(self.tokens[self.counter - 2].literal.to_string(), eval.to_string()));
                            self.adv();
                        } else {
                            self.adv();
                        }
                    } else if matches!(self.tokens[self.counter - 1].kind, TokenKind::Plus) && matches!(self.tokens[self.counter - 2].kind, TokenKind::Number) {
                        self.adv();
                    } else {
                        panic!("arithmetic error");
                    }
                }

                TokenKind::Print => {
                    let tok_res = self.stack.iter().find(|v| v.name == cur_tok.literal);
                    if !tok_res.is_none() {
                        if tok_res.unwrap().value != "" {
                            println!("{}", tok_res.unwrap().value);
                            self.adv();
                        } else {
                            panic!("cannot print an empty value");
                        }
                    } else if cur_tok.literal.starts_with("'") || cur_tok.literal.starts_with("\"") {
                        println!("{}", cur_tok.literal);
                        self.adv();
                    } else if cur_tok.literal.chars().collect::<Vec<char>>()[0].is_numeric() {
                        if cur_tok.literal.contains("+") {
                            let rep_str = cur_tok.literal.replace(" ", "");
                            let int_vec = rep_str.split("+").collect::<Vec<&str>>();
                            let mut sum = 0;
                            for i in int_vec.iter() {
                                let num = i.parse::<i32>().unwrap();
                                sum += num;
                            }
                            println!("{}", sum);
                            self.adv();
                        } else {
                            println!("{}", cur_tok.literal);
                            self.adv();
                        }
                    } else if tok_res.is_none() {
                        panic!("variable {} not found", cur_tok.literal);
                    }
                }

                TokenKind::Free => {
                    let tok_res = self.stack.iter().position(|v| v.name == cur_tok.literal);
                    if !tok_res.is_none() {
                        self.stack.remove(tok_res.unwrap());
                        self.adv();
                    } else {
                        panic!("variable {} not found", cur_tok.literal);
                    }
                }

                TokenKind::Plus => {
                    if !matches!(self.tokens[self.counter - 1].kind, TokenKind::Number) || !matches!(self.tokens[self.counter + 1].kind, TokenKind::Number) {
                        panic!("expected Number");
                    } else {
                        self.adv();
                    }
                }

                TokenKind::Dot => {
                    self.adv();
                }

                TokenKind::Semi => {
                    self.adv();
                }
            }
        }
    }

    fn adv(&mut self) {
        self.counter += 1;
    }
}