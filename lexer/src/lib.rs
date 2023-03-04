use logos::{Lexer, Logos};
use token::Token;

#[derive(Debug, PartialEq)]
pub struct Lexeme<'s> {
    pub slice: &'s str,
    pub token: Token,
}

pub struct ALexer<'s> {
    current: Option<Token>,
    lexer: Lexer<'s, Token>,
}

impl<'s> ALexer<'s> {
    pub fn new(buffer: &'s str) -> Self {
        return ALexer {
            current: None,
            lexer: Token::lexer(buffer),
        };
    }
    pub fn check_if(&mut self, token: Token) -> bool {
        if self.peek().unwrap().is_kind(token) {
            return true;
        }
        return false;
    }
    pub fn check_of_if(&mut self, token: &[Token]) -> bool {
        if self.peek().unwrap().is_of_kind(token) {
            return true;
        }
        return false;
    }
    pub fn get_lexeme(&mut self) -> Lexeme<'s> {
        Lexeme {
            slice: self.lexer.slice(),
            token: self.collect_token(),
        }
    }

    pub fn collect_if(&mut self, token: Token) -> Option<Lexeme<'s>> {
        if self.peek()?.is_kind(token) {
            return Some(self.get_lexeme());
        }
        return None;
    }
    pub fn collect_of_if(&mut self, token: &[Token]) -> Option<Lexeme<'s>> {
        if self.peek()?.is_of_kind(token) {
            return Some(self.get_lexeme());
        }
        return None;
    }
    pub fn peek(&mut self) -> Option<Token> {
        if self.current.is_none() {
            self.current = self.lexer.next();
        }
        self.current.clone()
    }
    pub fn collect_token(&mut self) -> Token {
        let temp = self.current.clone().unwrap();
        self.current = None;
        return temp;
    }
    pub fn collect_lexeme(&mut self) -> Lexeme<'s> {
        self.get_lexeme()
    }
    pub fn is_num(&mut self) -> bool {
        self.check_if(Token::Num)
    }
    pub fn is_tri_op(&mut self) -> bool {
        self.check_of_if(&[Token::Rot, Token::Hack])
    }
    pub fn is_bin_op(&mut self) -> bool {
        self.check_of_if(&[
            Token::Plus,
            Token::Sub,
            Token::Div,
            Token::Mod,
            Token::Mul,
            Token::Xor,
            Token::RShift,
            Token::LShift,
            Token::Or,
            Token::And,
        ])
    }
    pub fn is_un_op(&mut self) -> bool {
        self.check_of_if(&[Token::Not, Token::Neg, Token::Dup, Token::Pop])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_collect() {
        let mut lexer = ALexer::new("--x dup");
        assert_eq!(lexer.peek().unwrap(), Token::Sub);
        assert_eq!(lexer.collect_token(), Token::Sub);
        assert_eq!(lexer.collect_if(Token::Sub).unwrap().token, Token::Sub);
        assert_eq!(
            lexer
                .collect_of_if(&[Token::Plus, Token::Symbol])
                .unwrap()
                .token,
            Token::Symbol
        );
    }
}
