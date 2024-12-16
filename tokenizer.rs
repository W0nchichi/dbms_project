use super::token;
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

    pub fn tokenize_next_token(&self){
        
    }
}