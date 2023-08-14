use crate::tokens::Token;
use std::error::Error;
use std::fmt;

pub trait Lexer {
    fn state_transfer(state: u8, input: char) -> Result<u8, StateError>;
    fn lex_analysis(input: &String) -> Result<Vec<Token>, LexerError>;
}

#[derive(Debug)]
pub enum LexerError {
    WrongLetterError(char),
    StateError,
    WrongIdentifierError,
}
impl Error for LexerError {}
impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        match *self {
            LexerError::WrongLetterError(ref s) => {
                write!(f, "Letter Invalid: {}", s)
            },
            LexerError::StateError => {
                write!(f, "State Error")
            },
            LexerError::WrongIdentifierError => {
                write!(f, "ID starts with digits")
            }
        }
    }
}

#[derive(Debug)]
pub enum StateError {
    NoNextStateAtFinalError,
    NoNextStateAtMidError,
    InvalidStateError
}
impl Error for StateError {}
impl fmt::Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::NoNextStateAtFinalError => {
                write!(f, "No next state at final states.")
            }
            Self::NoNextStateAtMidError => {
                write!(f, "No next state at middle states.")
            }
            Self::InvalidStateError => {
                write!(f, "No such state.")
            }
        }
    }
}