use crate::scanner::{
    Scanner, TokenType,
};

pub fn compile(source: String) {
    let mut scanner = Scanner::new(&source);
    let mut line = 0usize;
    loop {
        let token = scanner.scan_token();
        if token.get_token_type() == TokenType::Eof || token.get_token_type() == TokenType::Error {
            break;
        } else if token.get_token_type() == TokenType::Error {
            println!("ERR");
            break;
        }
        if token.get_line() != line {
            print!("{}", format!("{:04} ", token.get_line()));
            line = token.get_line();
        } else {
            print!("   | ");
        }
        if let Some(ref message) = token.error {
            println!("{}", message)
        }
        println!("{:>2} {} '{}'", token.get_token_type() as usize, token.get_start(), scanner.get_token(token.get_start(), token.get_length()));
    }
}