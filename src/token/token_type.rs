#[derive(Debug)]
pub enum TokenType {
    ILLEGAL, // illegal
    EOF,     // eof

    // identifier + literals
    IDENT, // add，foobar, x, y, z,...
    INT,   // 12345

    // 运算符
    ASSIGN,   // =
    PLUS,     // +
    MINUS,    // -
    BANG,     // !
    ASTERISK, // *
    SLASH,    // /
    LT,       // <
    GT,       // >
    EQ,       // ==
    NOTEQ,    // !=

    // 分隔符
    COMMA,     // ,
    SEMICOLON, // ;
    LPAREN,    // (
    RPAREN,    // )
    LBRACE,    // {
    RBRACE,    // }

    // 关键字
    FUNCTION, // fn
    LET,      // let
    TRUE,     // true
    FALSE,    // false
    IF,       // if
    ELSE,     // else
    RETURN,   // return
}

impl TokenType {
    /// LookupIdent 通过检查关键字表来判断给定的标识符是否是关键字。如果是，则
    /// 返回关键字的 TokenType 常量。如果不是，则返回 token.IDENT，这个 TokenType 表
    /// 示当前是用户定义的标识符。
    pub fn lookup_ident(ident: &str) -> TokenType {
        match ident {
            "fn" => TokenType::FUNCTION,
            "let" => TokenType::LET,
            "true" => TokenType::TRUE,
            "false" => TokenType::FALSE,
            "if" => TokenType::IF,
            "else" => TokenType::ELSE,
            "return" => TokenType::RETURN,
            _ => TokenType::IDENT,
        }
    }
}
