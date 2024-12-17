use crate::dbms_rust_project::token::Token;
use std::str::Chars;
use std::iter::Peekable;

/*
Basically, the tokenizers job is to categorize the keywords in the Query, let's define a few goals
1. check if all words are valid in the language
2. Categorize each Word into it's own type (i.e. Keyword, Literals, Operators)
3. Parse "SELECT name FROM users;" into [Select, Identifier("name"), From, Identifier("users"), Semicolon]
4. Break down a query into relevant statements (i.e. see below)
    SELECT name, age FROM users WHERE age >= 18;
    [Select, Identifier(Name), Comma, Identifier(Age), From, Identifier(Users), WHERE, Identifier(age), greaterEqauls, NumericLiteral(18.0), semicolon]
 */

#[derive(Debug, PartialEq)]
pub enum LexerError {
    UnclosedStringLiteral,
    InvalidNumberFormat(String),
    UnknownEscapeSequence(char),
    // Add more error types as needed
}

pub struct Tokenizer<'a> {
    input: &'a str,
    chars: Peekable<Chars<'a>>,
    current_char: Option<char>,
    position: usize, // Position in the input string
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars_iter = input.chars().peekable();
        let first_char = chars_iter.next();
        Tokenizer {
            input,
            chars: chars_iter,
            current_char: first_char,
            position: 0,
        }
    }
    
    // |‾|_|‾| |‾‾‾| |‾|    |‾‾‾| |‾‾‾| |‾‾‾|
    // |     | |===  | |__  |  _| |===  |  <
    // |_|‾|_| |___| |____| |_|   |___| |_|\_\
    // Advance to the next character
    fn next_char(&mut self) {
        self.current_char = self.chars.next();
        self.position += 1;
    }

    // Peek at the next character without consuming it
    fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    // Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }

    // Identify keywords or identifiers
    fn identify_token(&mut self) -> Token {
        let mut ident = String::new();

        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.next_char();
            } else {
                break;
            }
        }

        match ident.to_uppercase().as_str() {
            "SELECT" => Token::Select,
            "FROM" => Token::From,
            "WHERE" => Token::Where,
            "INSERT" => Token::Insert,
            "DELETE" => Token::Delete,
            "NOT" => Token::Not,
            "AS" => Token::As,
            "AND" => Token::And,
            "OR" => Token::Or,
            "GROUPBY" => Token::GroupBy,
            // Add more keywords as needed
            _ => Token::Identifier(ident),
        }
    }

    // |===============|
    // |number literals|
    // |===============|
    // Handle numeric literals
    fn number(&mut self) -> Result<Token, LexerError> {
        let mut number_str = String::new();
        let mut has_decimal = false;

        while let Some(c) = self.current_char {
            if c.is_ascii_digit() {
                number_str.push(c);
                self.next_char();
            } else if c == '.' {
                if has_decimal {
                    // Second decimal point encountered; invalid number format
                    return Err(LexerError::InvalidNumberFormat(number_str));
                }
                has_decimal = true;
                number_str.push(c);
                self.next_char();
            } else {
                break;
            }
        }

        // Validate the number format
        if number_str.ends_with('.') {
            return Err(LexerError::InvalidNumberFormat(number_str));
        }

        // Parse the string to a floating-point number
        match number_str.parse::<f64>() {
            Ok(num) => Ok(Token::NumericLiteral(num)),
            Err(_) => Err(LexerError::InvalidNumberFormat(number_str)),
        }
    }

    // |===============|
    // |String literals|
    // |===============|
    // Handle string literals
    fn string_literal(&mut self) -> Result<Token, LexerError> {
        let quote_char = self.current_char.unwrap(); // Assume it's either ' or "
        self.next_char(); // Skip the opening quote
        let mut string = String::new();

        while let Some(c) = self.current_char {
            if c == quote_char {
                self.next_char(); // Skip the closing quote
                return Ok(Token::StringLiteral(string));
            } else if c == '\\' {
                self.next_char(); // Skip the backslash
                if let Some(escaped_char) = self.current_char {
                    match escaped_char {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        '\\' => string.push('\\'),
                        '"' => string.push('"'),
                        '\'' => string.push('\''),
                        other => {
                            // Handle unknown escape sequences
                            return Err(LexerError::UnknownEscapeSequence(other));
                        }
                    }
                    self.next_char();
                } else {
                    // Handle unclosed escape sequence
                    return Err(LexerError::UnclosedStringLiteral);
                }
            } else {
                string.push(c);
                self.next_char();
            }
        }

        // If we reach here, the string was not closed
        Err(LexerError::UnclosedStringLiteral)
    }

    // |‾‾‾‾| |‾‾‾‾| |‾|/‾| |‾‾‾| |‾\ ||
    //  ‾||‾  | |‾|| |   /  |===  |  \||
    //   ||   |__‾_| |_|\_\ |___| |_|\_|
    // Main method to tokenize the next token
    pub fn tokenize_next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();

        match self.current_char {
            Some('*') => {
                self.next_char();
                Ok(Token::Asterisk)
            }
            Some(';') => {
                self.next_char();
                Ok(Token::Semicolon)
            }
            Some(',') => {
                self.next_char();
                Ok(Token::Comma)
            }
            Some('=') => {
                self.next_char();
                // Check for '==' operator if needed
                Ok(Token::Equals)
            }
            Some('(') => {
                self.next_char();
                Ok(Token::LeftParen)
            }
            Some(')') => {
                self.next_char();
                Ok(Token::RightParen)
            }
            Some('"') | Some('\'') => {
                // Handle string literals
                self.string_literal()
            }
            Some('<') => {
                self.next_char();
                Ok(Token::LessThan)
            }
            Some('>') => {
                self.next_char();
                Ok(Token::GreaterThan)
            }
            Some(c) if c.is_ascii_digit() => {
                // Handle numeric literals
                self.number()
            }
            Some(c) if c.is_alphabetic() || c == '_' => {
                // Handle identifiers and keywords
                Ok(self.identify_token())
            }
            None => Ok(Token::EOF),
            Some(c) => {
                self.next_char();
                Ok(Token::Illegal(c))
            }
        }
    }

    // Tokenize the entire input and collect all tokens into a vector
    pub fn tokenize_all(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        loop {
            let token = self.tokenize_next_token()?;
            tokens.push(token.clone());
            if token == Token::EOF {
                break;
            }
        }
        Ok(tokens)
    }
}
