use std::fmt::Display;

use crate::error::LoxError;

#[derive(Debug, Clone)]
enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // End of file
    Eof,
}

#[derive(Debug, Clone)]
enum Literal {
    String(String),
    Number(f64),
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {} {:?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LoxError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::new(),
            literal: None,
            line: self.line,
        });

        Ok(self.tokens.clone())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(())
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => Ok(()),
            '\n' => {
                self.line += 1;
                Ok(())
            }
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => {
                while self.peek().is_alphanumeric() {
                    self.advance();
                }

                let text = &self.source[self.start..self.current];
                match text {
                    "and" => self.add_token(TokenType::And),
                    "class" => self.add_token(TokenType::Class),
                    "else" => self.add_token(TokenType::Else),
                    "false" => self.add_token(TokenType::False),
                    "for" => self.add_token(TokenType::For),
                    "fun" => self.add_token(TokenType::Fun),
                    "if" => self.add_token(TokenType::If),
                    "nil" => self.add_token(TokenType::Nil),
                    "or" => self.add_token(TokenType::Or),
                    "print" => self.add_token(TokenType::Print),
                    "return" => self.add_token(TokenType::Return),
                    "super" => self.add_token(TokenType::Super),
                    "this" => self.add_token(TokenType::This),
                    "true" => self.add_token(TokenType::True),
                    "var" => self.add_token(TokenType::Var),
                    "while" => self.add_token(TokenType::While),
                    _ => self.add_token(TokenType::Identifier),
                }
            }
            _ => Err(LoxError::new(self.line, "Unexpected character".to_string())),
        }
    }

    fn add_token(&mut self, token_type: TokenType) -> Result<(), LoxError> {
        Ok(self.tokens.push(Token {
            token_type: token_type,
            lexeme: self.source[self.start..self.current].to_string(),
            literal: None,
            line: self.line,
        }))
    }

    fn add_token_literal(
        &mut self,
        token_type: TokenType,
        literal: Literal,
    ) -> Result<(), LoxError> {
        Ok(self.tokens.push(Token {
            token_type: token_type,
            lexeme: self.source[self.start..self.current].to_string(),
            literal: Some(literal),
            line: self.line,
        }))
    }

    fn advance(&mut self) -> char {
        let char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        char
    }

    fn match_char(&mut self, arg: char) -> bool {
        if let Some(c) = self.source.chars().nth(self.current) {
            if c == arg {
                self.current += 1;
                return true;
            }
        }
        false
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn string(&mut self) -> Result<(), LoxError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(LoxError::new(self.line, "Unterminated string".to_string()));
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_literal(TokenType::String, Literal::String(value))
    }

    fn number(&mut self) -> Result<(), LoxError> {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        self.add_token_literal(
            TokenType::Number,
            Literal::Number(
                self.source[self.start..self.current]
                    .parse::<f64>()
                    .unwrap(),
            ),
        )
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }
}
