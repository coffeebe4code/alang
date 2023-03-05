use lexer::Lexeme;
use token::Token;

#[derive(Debug, PartialEq)]
pub enum Expr<'s> {
    TriOp(Box<Expr<'s>>, Box<Expr<'s>>, Box<Expr<'s>>, Token),
    BinOp(Box<Expr<'s>>, Box<Expr<'s>>, Token),
    AsOp(Box<Expr<'s>>, Box<Expr<'s>>, Token),
    UnOp(Box<Expr<'s>>, Token),
    Number(Lexeme<'s>),
    Ident(Lexeme<'s>),
    Chars(Lexeme<'s>),
}

#[macro_export]
macro_rules! expr {
    ($val:ident, $($inner:tt)*) => {
        Box::new(Expr::$val($($inner)*));
    };
}
