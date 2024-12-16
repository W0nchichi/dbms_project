pub enum Token {
    //Keywords (i.e. Select)
    Select,
    Insert,
    Delete,
    Where,
    From,
    Not,
    As,
    And,
    Or,
    GroupBy,
    // Identifiers (i.e. age → Identifier (Identifier("age")
    Identifier(String),
    //Operators (<, !=)
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    //Punctuation (i.e. ',', ';')
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    //End (I guess?)
    EndOfFunction
}