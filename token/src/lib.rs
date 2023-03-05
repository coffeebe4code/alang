use logos::Logos;

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum Token {
    // special operator symbols
    #[token("dup")]
    Dup,
    #[token("hack")]
    Hack,
    #[token("swap")]
    Swap,
    #[token("rot")]
    Rot,
    #[token("pop")]
    Pop,
    #[token("nip")]
    Nip,
    #[token("neg")]
    Neg,
    // end special operator symbols
    #[token("let")]
    Let,
    #[token("const")]
    Const,
    #[token("pub")]
    Pub,
    #[token("once")]
    Once,
    #[token("this")]
    This,
    #[token("trait")]
    Trait,
    #[token("queue")]
    Queue,
    #[token("thread")]
    Thread,
    #[token("proc")]
    Proc,
    #[token("undef")]
    Undef,
    #[token("for")]
    For,
    #[token("break")]
    Break,
    #[token("loop")]
    Loop,
    #[token("in")]
    In,
    #[token("of")]
    Of,
    #[token("func")]
    Func,
    #[token("typeof")]
    TypeOf,
    #[token("obj")]
    Object,
    #[token("enum")]
    Enum,
    #[token("list")]
    List,
    #[token("true")]
    True,
    #[token("false")]
    False,

    #[token("(")]
    OParen,
    #[token(")")]
    CParen,
    #[token("{")]
    OBrace,
    #[token("}")]
    CBrace,
    #[token("[")]
    OArray,
    #[token("]")]
    CArray,

    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token("$")]
    Dollar,
    #[token("?")]
    Question,
    #[token("#")]
    Pound,
    #[token(":")]
    Colon,
    #[token(";")]
    SColon,
    #[token("`")]
    Backtick,
    #[token("@")]
    At,
    #[token("<")]
    Lt,
    #[token("<=")]
    LtEq,
    #[token(">")]
    Gt,
    #[token(">=")]
    GtEq,
    #[token("/")]
    Div,
    #[token("\\")]
    BSlash,
    #[token("+")]
    Plus,
    #[token("_")]
    Rest,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("|")]
    Or,
    #[token("&")]
    And,
    #[token("^")]
    Xor,
    #[token("<<")]
    LShift,
    #[token(">>")]
    RShift,
    #[token("~")]
    Not,
    #[token("=")]
    As,
    #[token("&&")]
    AndLog,
    #[token("||")]
    OrLog,
    #[token("!=")]
    NotEquality,
    #[token("==")]
    Equality,
    #[token("!")]
    NotLog,
    #[token("%")]
    Mod,
    #[regex(r#"'([^'\\]|\\t|\\u|\\n|\\')*'"#)]
    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#)]
    Chars,
    #[regex("[a-zA-Z]+")]
    Symbol,
    #[regex("[1-9][0-9]*\\.[0-9]+|0\\.[0-9]+|0|[1-9][0-9]*")]
    Num,

    #[regex(r"//.*", logos::skip)]
    #[regex(r"[ \t\r\f\n]+", logos::skip)]
    #[error]
    Error,
}

impl Token {
    pub fn is_kind(self, tok: Token) -> bool {
        return tok == self;
    }
    pub fn is_of_kind(self, tokens: &[Token]) -> bool {
        return tokens.iter().any(|t| *t == self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_tokenizes() {
        let mut lexer = Token::lexer("5 x let");
        assert_eq!(lexer.next(), Some(Token::Num));
        assert_eq!(lexer.next(), Some(Token::Symbol));
        assert_eq!(lexer.next(), Some(Token::Let));
    }

    #[test]
    fn it_tokenizes_chars() {
        let mut lexer = Token::lexer("\"5 x let\"");
        let mut lexer2 = Token::lexer("'5 x let'");
        let mut lexer3 = Token::lexer("\"5 \\\"x let\"");
        assert_eq!(lexer.next(), Some(Token::Chars));
        assert_eq!(lexer.slice(), "\"5 x let\"");
        assert_eq!(lexer2.next(), Some(Token::Chars));
        assert_eq!(lexer2.slice(), "'5 x let'");
        assert_eq!(lexer3.next(), Some(Token::Chars));
        assert_eq!(lexer3.slice(), "\"5 \\\"x let\"");
    }

    #[test]
    fn it_tokenizes_nums() {
        let mut lexer1 = Token::lexer("5");
        let mut lexer2 = Token::lexer("50");
        let mut lexer3 = Token::lexer("0");
        let mut lexer4 = Token::lexer("55");
        assert_eq!(lexer1.next(), Some(Token::Num));
        assert_eq!(lexer2.next(), Some(Token::Num));
        assert_eq!(lexer3.next(), Some(Token::Num));
        assert_eq!(lexer4.next(), Some(Token::Num));
    }
    #[test]
    fn it_skips_comments() {
        let mut lexer1 = Token::lexer("5 //comment");
        let mut lexer2 = Token::lexer(" //comment\n 2");
        let mut lexer3 = Token::lexer("7 //comment\n");
        let mut lexer4 = Token::lexer("7 / //comment\n 5");
        assert_eq!(lexer1.next(), Some(Token::Num));
        assert_eq!(lexer1.next(), None);
        assert_eq!(lexer2.next(), Some(Token::Num));
        assert_eq!(lexer3.next(), Some(Token::Num));
        assert_eq!(lexer3.next(), None);
        assert_eq!(lexer4.next(), Some(Token::Num));
        assert_eq!(lexer4.next(), Some(Token::Div));
        assert_eq!(lexer4.next(), Some(Token::Num));
    }
    #[test]
    fn it_tokenizes_decimals() {
        let mut lexer1 = Token::lexer("5.0");
        let mut lexer2 = Token::lexer("50.0");
        let mut lexer3 = Token::lexer("0.0");
        let mut lexer4 = Token::lexer("0.1");
        let mut lexer5 = Token::lexer(".1");
        let mut lexer6 = Token::lexer("1.");
        let mut lexer7 = Token::lexer("01.2");
        let mut lexer8 = Token::lexer("1.00");
        assert_eq!(lexer1.next(), Some(Token::Num));
        assert_eq!(lexer2.next(), Some(Token::Num));
        assert_eq!(lexer3.next(), Some(Token::Num));
        assert_eq!(lexer4.next(), Some(Token::Num));
        assert_eq!(lexer5.next(), Some(Token::Dot));
        assert_eq!(lexer5.next(), Some(Token::Num));
        assert_eq!(lexer6.next(), Some(Token::Num));
        assert_eq!(lexer6.next(), Some(Token::Dot));
        assert_eq!(lexer7.next(), Some(Token::Num));
        assert_eq!(lexer7.next(), Some(Token::Num));
        assert_eq!(lexer8.next(), Some(Token::Num));
    }
}
