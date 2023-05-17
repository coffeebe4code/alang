use ast::*;
use lexer::ALexer;

pub struct Parser<'s> {
    lexer: ALexer<'s>,
}

impl<'s> Parser<'s> {
    pub fn new(lexer: ALexer<'s>) -> Self {
        Parser { lexer }
    }
    pub fn program(&mut self) -> Vec<Box<Expr<'s>>> {
        let mut program: Vec<Box<Expr>> = vec![];
        let mut stack: Vec<Box<Expr>> = vec![];
        loop {
            match self.lexer.peek() {
                Some(_) => {
                    if self.lexer.is_num() {
                        stack.push(expr!(Number, self.lexer.collect_lexeme()));
                    } else if self.lexer.is_chars() {
                        program.push(expr!(Chars, self.lexer.collect_lexeme()));
                    } else if self.lexer.is_un_op() {
                        let right = stack.pop();
                        if right.is_none() {
                            panic!("stack underflow");
                        }
                        program.push(expr!(UnOp, right.unwrap(), self.lexer.collect_token(),));
                    } else if self.lexer.is_bin_op() {
                        // 4 2 /
                        let right = stack.pop();
                        let left = stack.pop();
                        if right.is_none() || left.is_none() {
                            panic!("stack underflow");
                        }
                        program.push(expr!(
                            BinOp,
                            left.unwrap(),
                            right.unwrap(),
                            self.lexer.collect_token(),
                        ));
                    } else if self.lexer.is_tri_op() {
                        let right = stack.pop();
                        let middle = stack.pop();
                        let left = stack.pop();
                        if right.is_none() || middle.is_none() || left.is_none() {
                            panic!("stack underflow");
                        }
                        program.push(expr!(
                            TriOp,
                            left.unwrap(),
                            middle.unwrap(),
                            right.unwrap(),
                            self.lexer.collect_token(),
                        ));
                    } else if self.lexer.is_as_op() {
                        let var = stack.pop();
                        let value = stack.pop();
                        if var.is_none() || value.is_none() {
                            panic!("stack underflow");
                        }
                        program.push(expr!(
                            AsOp,
                            value.unwrap(),
                            var.unwrap(),
                            self.lexer.collect_token(),
                        ));
                    } else {
                        panic!("not implemented");
                    }
                }
                _ => {
                    break;
                }
            }
        }
        return program;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::Lexeme;
    use token::Token;
    #[test]
    fn it_should_parse_bin() {
        let mut parser = Parser::new(ALexer::new("55 55 +"));
        assert_eq!(
            parser.program().pop().unwrap(),
            expr!(
                BinOp,
                expr!(
                    Number,
                    Lexeme {
                        slice: "55",
                        span: 0..2,
                        token: Token::Num
                    }
                ),
                expr!(
                    Number,
                    Lexeme {
                        slice: "55",
                        span: 3..5,
                        token: Token::Num
                    }
                ),
                Token::Plus
            )
        );
    }
    #[test]
    fn it_should_parse_ident() {}
}
