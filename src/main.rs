use std::{fmt, iter::Peekable, str::Chars};

fn main() {
    //let mut line = String::new();
    //std::io::stdin().read_line(&mut line).unwrap();
    //
    let code = r#"define incr X = X + 1
                  define decr X = X - 1
        "#;

    //println!("code: {}", code);
    let mut lexer = Lexer::new(code);

    //lexer.parse();

    //let first_token = lexer.next_token();
    //println!("Token: {}", first_token);

    //let second_token = lexer.next_token();
    //println!("Token: {}", second_token);

    //let third_token = lexer.next_token();
    //println!("Token: {}", third_token);

    //let fourth_token = lexer.next_token();
    //println!("Token: {}", fourth_token);

    //let fifth_token = lexer.next_token();
    //println!("Token: {}", fifth_token);

    //let sixth_token = lexer.next_token();
    //println!("Token: {}", sixth_token);

    //let seventh_token = lexer.next_token();
    //println!("Token: {}", seventh_token);

    //let code_chars = "".chars();
}

#[derive(Debug, PartialEq)]
enum TokenType {
    Assign,
    Define,
    EOF,
    Identifier(String),
    Integer(String),
    Minus,
    Plus,
    SemiColon,
}

impl core::fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            TokenType::Assign => write!(f, "assign"),
            TokenType::Define => write!(f, "define"),
            TokenType::EOF => write!(f, "end of file"),
            TokenType::Identifier(identifier) => write!(f, "identifier {}", identifier),
            TokenType::Integer(value) => write!(f, "integer {}", value),
            TokenType::Minus => write!(f, "minus"),
            TokenType::Plus => write!(f, "plus"),
            TokenType::SemiColon => write!(f, "semicolon"),
        }
    }
}

struct Lexer<'a> {
    code: Peekable<Chars<'a>>,
    column: usize,
    row: usize,
    ch: Option<char>,
}

impl<'a> Lexer<'a> {
    fn new(code: &'a str) -> Self {
        let mut lexer = Lexer {
            code: code.chars().peekable(),
            column: 0,
            row: 1,
            ch: None,
        };

        lexer.read_char();
        return lexer;
    }

    //fn parse(&mut self) {
    //    while self.ch != None {
    //        println!("token={}", self.next_token());
    //    }
    //}

    fn read_char(&mut self) {
        self.ch = self.code.next();

        match self.ch {
            Some(ch) => {
                if ch == '\n' {
                    self.row += 1;
                    self.column = 0;
                } else {
                    self.column += 1;
                }
            }
            _ => {}
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.code.peek()
    }

    fn read_word(&mut self) -> String {
        let mut word = String::new();

        while self.ch.is_some() && self.ch.unwrap() != ' ' {
            word.push(self.ch.unwrap());

            self.read_char();
        }

        return word;
    }

    fn read_number(&mut self) -> String {
        let mut number = String::new();

        while self.ch.is_some() && self.ch.unwrap().is_digit(10) {
            number.push(self.ch.unwrap());
            self.read_char();
        }

        return number;
    }
    fn next_token(&mut self) -> TokenType {
        while self.ch.is_some() && (self.ch.unwrap() == ' ' || self.ch.unwrap() == '\n') {
            self.read_char();
        }

        let token = match self.ch {
            Some('+') => TokenType::Plus,
            Some('-') => TokenType::Minus,
            Some('=') => TokenType::Assign,
            Some(';') => TokenType::SemiColon,
            Some('0'..='9') => {
                //TODO: PQ PRECISA RETORNAR EXPLICITAMENTE
                return TokenType::Integer(self.read_number());
            }
            Some(_) => {
                let word = self.read_word();
                match word.as_str() {
                    "define" => TokenType::Define,
                    _ => TokenType::Identifier(word),
                }
            }
            None => TokenType::EOF,
        };

        self.read_char();

        return token;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn define_a_function() {
        let code = "define increase X = X + 1;";

        let mut lexer = Lexer::new(code);

        let expected_tokens = vec![
            TokenType::Define,
            TokenType::Identifier("increase".to_string()),
            TokenType::Identifier("X".to_string()),
            TokenType::Assign,
            TokenType::Identifier("X".to_string()),
            TokenType::Plus,
            TokenType::Integer(1.to_string()),
            TokenType::SemiColon,
            TokenType::EOF,
        ];

        for ele in expected_tokens {
            assert_eq!(ele, lexer.next_token());
        }
    }

    #[test]
    fn test() {
        let code = r#"define increase X = X + 1;
                      define decrease X = X - 1;"#;

        let mut lexer = Lexer::new(code);

        let expected_tokens = vec![
            TokenType::Define,
            TokenType::Identifier("increase".to_string()),
            TokenType::Identifier("X".to_string()),
            TokenType::Assign,
            TokenType::Identifier("X".to_string()),
            TokenType::Plus,
            TokenType::Integer(1.to_string()),
            TokenType::SemiColon,
            TokenType::Define,
            TokenType::Identifier("decrease".to_string()),
            TokenType::Identifier("X".to_string()),
            TokenType::Assign,
            TokenType::Identifier("X".to_string()),
            TokenType::Minus,
            TokenType::Integer(1.to_string()),
            TokenType::SemiColon,
            TokenType::EOF,
        ];

        for ele in expected_tokens {
            assert_eq!(ele, lexer.next_token());
        }
    }
}
