use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq)]
#[repr(usize)]
pub enum TokenType {
    // Single-character tokens
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

    // One or two character tokens
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
    For,
    Fun,
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

    // Special tokens
    Error,
    Eof,
}

pub struct Token {
    token_type: TokenType,
    start: usize,
    length: usize,
    line: usize,
    pub error: Option<String>,
}

impl Token {
    pub fn get_token_type(&self) -> TokenType {
        self.token_type.clone()
    }

    pub fn get_start(&self) -> usize {
        self.start
    }

    pub fn get_length(&self) -> usize {
        self.length
    }

    pub fn get_line(&self) -> usize {
        self.line
    }
}

pub struct Scanner<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Scanner {
            source: source,
            chars: source.chars().peekable(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;
        
        if self.is_at_end() {
            return self.make_token(TokenType::Eof)
        }

        match self.advance() {
            Some('(') => return self.make_token(TokenType::LeftParen),
            Some(')') => return self.make_token(TokenType::RightParen),
            Some('{') => return self.make_token(TokenType::LeftBrace),
            Some('}') => return self.make_token(TokenType::RightBrace),
            Some(';') => return self.make_token(TokenType::Semicolon),
            Some(',') => return self.make_token(TokenType::Comma),
            Some('.') => return self.make_token(TokenType::Dot),
            Some('-') => return self.make_token(TokenType::Minus),
            Some('+') => return self.make_token(TokenType::Plus),
            Some('/') => return self.make_token(TokenType::Slash),
            Some('*') => return self.make_token(TokenType::Star),
            Some('!') => {
                if self.match_char('=') {
                    return self.make_token(TokenType::BangEqual);
                } else {
                    return self.make_token(TokenType::Bang);
                }
            }
            Some('=') => {
                if self.match_char('=') {
                    return self.make_token(TokenType::EqualEqual);
                } else {
                    return self.make_token(TokenType::Equal);
                }
            }
            Some('<') => {
                if self.match_char('=') {
                    return self.make_token(TokenType::LessEqual);
                } else {
                    return self.make_token(TokenType::Less);
                }
            }
            Some('"') => {
                return self.string();
            }
            Some('>') => {
                if self.match_char('=') {
                    return self.make_token(TokenType::GreaterEqual);
                } else {
                    return self.make_token(TokenType::Greater);
                }
            }
            Some(value) => {
                if value.is_digit(10) {
                    return self.number();
                }

                if value.is_alphabetic() {
                    return self.identifier();
                }

                return self.error_token("Unexpected character.");
            }
            None => {
                return self.make_token(TokenType::Eof);
            }
        }
    }

    pub fn get_token(&mut self, index: usize, len: usize) -> &str{
        let start = self.source.char_indices().nth(index).map(|(i, _)| i).unwrap_or_else(|| usize::MAX);
        if start == usize::MAX {
            return "";
        }
        let end = self.source.char_indices().nth(index + len).map(|(i, _)| i).unwrap_or_else(|| self.source.len());
        let slice = &self.source[start..end];
        slice
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.chars.next()
    }

    fn is_at_end(&mut self) -> bool {
        self.chars.peek() == Some(&'\0') || self.chars.peek() == None
    }
    
    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type: token_type,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
            error: None,
        }
    }

    fn error_token(&self, message: &str) -> Token {
        Token {
            token_type: TokenType::Error,
            start: self.start,
            length: message.len(),
            line: self.line,
            error: Some(String::from(message)),
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if *self.chars.peek().unwrap() != expected {
            return false;
        }
        
        true
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.chars.peek() {
                Some(' ') | Some('\r') | Some('\t') => {
                    self.advance();
                }
                Some('\n') => {
                    self.line += 1;
                    self.advance();
                }
                Some(_) => break,
                None => break
            }
        }
    }

    fn string(&mut self) -> Token {
        while self.chars.peek() != Some(&'"') && !self.is_at_end() {
            if self.chars.peek() == Some(&'\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string");
        }

        self.advance();
        self.make_token(TokenType::String)
    }

    fn peek_next(&self) -> Option<char> {
        let mut clone = self.chars.clone();
        clone.next();
        clone.peek().copied()
    }

    fn number(&mut self) -> Token {
        while matches!(self.chars.peek(), Some(c) if c.is_ascii_digit()) {
            self.advance();
        }

        if self.chars.peek() == Some(&'.') {
            if let Some(next) = self.peek_next() {
                if next.is_ascii_digit() {
                    self.advance();

                    while matches!(self.chars.peek(), Some(c) if c.is_ascii_digit()) {
                        self.advance();
                    }
                }
            }
        }

        self.make_token(TokenType::Number)
    }

    fn identifier_type(lexeme: &str) -> TokenType {
        match lexeme {
            "and"    => TokenType::And,
            "class"  => TokenType::Class,
            "else"   => TokenType::Else,
            "if"     => TokenType::If,
            "nil"    => TokenType::Nil,
            "or"     => TokenType::Or,
            "print"  => TokenType::Print,
            "return" => TokenType::Return,
            "super"  => TokenType::Super,
            "var"    => TokenType::Var,
            "while"  => TokenType::While,
            "false"  => TokenType::False,
            "for"    => TokenType::For,
            "fun"    => TokenType::Fun,
            "this"   => TokenType::This,
            "true"   => TokenType::True,
            _        => TokenType::Identifier,
        }
    }

    fn identifier(&mut self) -> Token {
        while matches!(self.chars.peek(), Some(c) if c.is_alphanumeric()) {
            self.advance();
        }
        let lexeme = &self.source[self.start..self.current];
        self.make_token(Self::identifier_type(lexeme))
    }
}