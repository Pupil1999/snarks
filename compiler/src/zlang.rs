use crate::{
    lexer::Lexer,
    lexer::{LexerError, StateError},
    tokens::Token
};

pub struct ZlangCompiler;
impl Lexer for ZlangCompiler {
    fn lex_analysis(input: &String) -> Result<Vec<Token>, LexerError>{
        let mut token_list = Vec::<Token>::new();
        let mut state: u8 = 1;
        let mut buffer = String::new();
        let mut citer = input.clone();
        citer.push(' ');
        for c in citer.chars() {
            // print!("{}--{}->", state, c);
            // Update the FSM state according to the input.
            match Self::state_transfer(state, c) {
                // Update the state and store the input char
                Ok(new_state) => {
                    state = new_state;
                    buffer.push(c)
                }
                Err(StateError::NoNextStateAtFinalError) => {
                    if !Token::is_split_token(c) {
                        return Err(LexerError::WrongLetterError(c));
                    }
                    match state {
                        2 => {
                            if Token::is_keyword(&buffer) {
                                token_list.push(Token::KEYWORDS(buffer.clone()))
                            } else {
                                token_list.push(Token::IDENTIFIER(buffer.clone()))
                            }
                        }
                        3|5 => {
                            token_list.push(Token::NUMBER(i128::from_str_radix(&buffer, 10).unwrap()));
                        }
                        4|6|7|8|9|10 => token_list.push(Token::OPERATOR(buffer.clone())),
                        _ => {}
                    }

                    // The state is reset, but still need to be proceded.
                    // That's because we haven't deal with the new letter.
                    buffer.clear();
                    state = 1;
                    match Self::state_transfer(state, c) {
                        Ok(new_state) => {
                            state = new_state;
                            buffer.push(c);
                        }
                        Err(StateError::NoNextStateAtFinalError) => {
                            if !c.is_whitespace() && c != ' ' {
                                token_list.push(Token::SYMBOL(c));
                            }
                        }
                        _ => {}
                    }
                },
                Err(StateError::NoNextStateAtMidError) => {
                    return Err(LexerError::StateError)
                },
                Err(StateError::InvalidStateError) => {
                    return Err(LexerError::StateError)
                }
            }
        }

        Ok(token_list)
    }

    /**
     * @param state: current state of FSM
     * @param input: current input
     * @return: new state of FSM, or error
     */
    fn state_transfer(state: u8, input: char) -> Result<u8, StateError> {
        match state {
            1 => {
                if input.is_alphabetic() || input == '_' {
                    return Ok(2)
                } else if Token::is_none_zero_digit(input) {
                    return Ok(5)
                } else if input == '-' {
                    return Ok(4)
                } else if input == '0' {
                    return Ok(3)
                } else if input == '*' {
                    return Ok(7)
                } else if Token::is_operator(input) && input != '-' && input != '*'{
                    return Ok(6)
                } else {
                    return Err(StateError::NoNextStateAtFinalError)
                }
            }
            2 => {
                if input.is_alphanumeric() || input == '_' {
                    return Ok(2)
                } else {
                    return Err(StateError::NoNextStateAtFinalError)
                }
            }
            3 => {
                return Err(StateError::NoNextStateAtFinalError)
            }
            4 => {
                if Token::is_none_zero_digit(input) {
                    return Ok(5)
                } else if input == '=' {
                    return Ok(9)
                } else {
                    return Err(StateError::NoNextStateAtFinalError)
                }
            }
            5 => {
                if input.is_numeric() {
                    return Ok(5)
                } else {
                    return Err(StateError::NoNextStateAtFinalError)
                }
            }
            6 => {
                if input == '=' {
                    return Ok(10)
                } else {
                    return Err(StateError::NoNextStateAtFinalError)
                }
            }
            7 => {
                if input == '*' || input == '=' {
                    return Ok(8)
                } else {
                    return Err(StateError::NoNextStateAtFinalError)
                }
            }
            8 => Err(StateError::NoNextStateAtFinalError),
            9 => Err(StateError::NoNextStateAtFinalError),
            10 => Err(StateError::NoNextStateAtFinalError),
            _ => Err(StateError::InvalidStateError)
        }
    }
}


#[cfg(test)]
mod tests {
    use std::fs;
    use crate::lexer::Lexer;

    use super::ZlangCompiler;

    #[test]
    fn test_clex() {
        let path = String::from("src/code.cvm");
        let contents = fs::read_to_string(path).unwrap();
        let tokens = ZlangCompiler::lex_analysis(&contents).unwrap();
        println!("{:#?}", tokens);
    }
}