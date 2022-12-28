use std::{fs, env};

fn main() {
    let fp: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&fp[1]);

    let mut lexer = Lexer::new(contents.unwrap());

    lexer.lex();
}

enum TokenKind {
    String,
    Equal,
    Identifier
}

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

    pub fn lex(&mut self) {
        let tokens: Vec<Token> = Vec::new();

        while self.counter < self.contents.len() {
            let c = self.cur_char(); 
            
            match c {
                _ if c.is_alphabetic() => {
                    
                },

                _ => {
                    
                }
            }
        }
    }

    fn cur_char(&self) -> char {
        *self.contents.get(self.counter).unwrap()
    }

    fn adv(&mut self) {
        self.counter += 1;
    }
}