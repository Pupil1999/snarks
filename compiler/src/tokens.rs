// This mod defines all the tokens that the language will have
#[derive(Debug)]
pub enum Token {
    IDENTIFIER(String),
    NUMBER(i128),

    // ';', '{', '}', ':', ')', ']'
    SYMBOL(char),

    // '-', '+', '/', '*', '**', '>', '<', '||', '!', '&', '='
    OPERATOR(String),

    // 'break', 'continue', 'for', 'while', 'if', 'else'
    KEYWORDS(String)
}
impl Token {
    pub fn is_type(s: &String) -> bool {
        s == "int" ||
        s == "uint"
    }

    pub fn is_operator(c: char) -> bool {
        c == '-'  ||
        c == '+'  ||
        c == '*'  ||
        c == '/'  ||
        c == '='
    }

    pub fn is_keyword(s: &String) -> bool {
        s == "for"    ||
        s == "break"  ||
        s == "if"     ||
        s == "return" ||
        s == "true"   ||
        s == "false"  ||
        s == "func"   ||
        s == "public"
    }

    // If the FSM gets an invalid input char at some final states,
    // the char must be a split token
    pub fn is_split_token(c: char) -> bool {
        c == ' ' || 
        c == '(' ||
        c == ')' ||
        c == ';' ||
        c == ':' ||
        c == '{' ||
        c == '}' ||
        c == '[' ||
        c == ']' ||
        c == ',' ||
        c.is_whitespace() ||
        Self::is_operator(c)
    }
    
    pub fn is_none_zero_digit(c: char) -> bool {
        c.is_numeric() && c != '0'
    }
}