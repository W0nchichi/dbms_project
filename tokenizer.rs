use crate::dbms_rust_project::token::Token;
/*
Basically, the tokenizers job is to categorize the keywords in the Query, let's define a few goals
1. check if all words are valid in the language
2. Categorize each Word into it's own type (i.e. Keyword, Literals, Operators)
3. Parse "SELECT name FROM users;" into [Select, Identifier("name"), From, Identifier("users"), Semicolon]
4. Break down a query into relevant statements (i.e. see below)
    SELECT name, age FROM users WHERE age >= 18;
    [Select, Identifier(Name), Comma, Identifier(Age), From, Identifier(Users), WHERE, Identifier(age), greaterEqauls, NumericLiteral(18.0), semicolon]

 */
pub struct Tokenizer<'a>{
    input: &'a str,
    position: usize,
    current_char: Option<char>,
}

impl<'a> Tokenizer<'a>{
    pub fn new(input: &'a str) -> Self{
        //needs to be mutable so that as we go through the string can update position and current_chat
        let mut tokenizer = Tokenizer {
            input,
            position: 0,
            current_char: None,
        };
        //since the Tokenizer is already instantiated with a reference to a string, we can begin tokenizing that string
        //<Insert a function to begin>
        tokenizer
    }

    // |‾|_|‾| |‾‾‾| |‾|    |‾‾‾| |‾‾‾| |‾‾‾|
    // |     | |===  | |__  |  _| |===  |  <
    // |_|‾|_| |___| |____| |_|   |___| |_|\_\
    pub fn next_token(&mut self){
        if self.position < self.input.len() {
            //update position by 1, change char to the next char in a slice of the input
            //idk if you can properly use .next, but probably not right
            self.position += 1;
            self.current_char = Some(self.input.chars().nth(self.position).unwrap());
        } else {
            self.current_char = None;
        }
    }

    //getting rid of those pesky whitespaces
    pub fn skip_whitespace(&mut self){
        while let Some(char) = self.current_char {
            if char.is_whitespace(){
                self.next_token();
            } else {
                break;
            }
        }
    }
    
    //from a starting character, identify the correct token necessary
    pub fn identify_token(&mut self) -> Token{
        let start = self.position - 1;

        //make a while loop over all of the characters of the token until a space
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                self.next_token();
            } else {
                break;
            }
        }
        //match the word with that position
        let end = self.position - 1;
        let ident = &self.input[start..end];
        match ident.to_uppercase().as_str() {
            "SELECT" => {self.next_token(); Token::Select}
            "FROM" => {self.next_token(); Token::From}
            /*
            NEED TO FINISH THIS TOKENIZATION!!!!!
             */
            _ => {
                self.next_token();
                Token::EndOfFunction
            }
        }
        
    }


    // |‾‾‾‾| |‾‾‾‾| |‾|/‾| |‾‾‾| |‾\ ||
    //  ‾||‾  | |‾|| |   /  |===  |  \||
    //   ||   |__‾_| |_|\_\ |___| |_|\_|


    /* To reference the Token.rs Enum, use this v
    match self.current_char {
        Some('*') => {
            Next_Word_Function()
            Token::Asterisk
        }
    }
    */

    pub fn tokenize_next_token(&mut self) -> Token{
        self.skip_whitespace();

        match self.current_char {
            Some('*') => {
                self.next_token();
                Token::Asterisk
            }
            Some(';') => {
                self.next_token();
                Token::Semicolon
            }
            Some(',') => {
                self.next_token();
                Token::Comma
            }   
            //don't need a parameter since already know current token
            Some(char) => self.identify_token(),
            None => Token::EndOfFunction,
            _ => {
                self.next_token();
                Token::EndOfFunction
            }
        }
        
        
    }
}