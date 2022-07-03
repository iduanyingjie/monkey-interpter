use crate::token::{Token, TokenType};

use super::Lexer;

#[test]
fn test() {
    let input = r#"
let five = 5;
let ten = 10;
let add = fn(x, y) {
    x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
"#;
    let result = vec![
        Token::new(TokenType::LET, "let".into()),
        Token::new(TokenType::IDENT, "five".into()),
        Token::new(TokenType::ASSIGN, "=".into()),
        Token::new(TokenType::INT, "5".into()),
        Token::new(TokenType::SEMICOLON, ";".into()),
        Token::new(TokenType::LET, "let".into()),
        Token::new(TokenType::IDENT, "ten".into()),
        Token::new(TokenType::ASSIGN, "=".into()),
        Token::new(TokenType::INT, "10".into()),
        Token::new(TokenType::SEMICOLON, ";".into()),
        Token::new(TokenType::LET, "let".into()),
        Token::new(TokenType::IDENT, "add".into()),
        Token::new(TokenType::ASSIGN, "=".into()),
        Token::new(TokenType::FUNCTION, "fn".into()),
        Token::new(TokenType::LPAREN, "(".into()),
        Token::new(TokenType::IDENT, "x".into()),
        Token::new(TokenType::COMMA, ",".into()),
        Token::new(TokenType::IDENT, "y".into()),
        Token::new(TokenType::RPAREN, ")".into()),
        Token::new(TokenType::LBRACE, "{".into()),
        Token::new(TokenType::IDENT, "x".into()),
        Token::new(TokenType::PLUS, "+".into()),
        Token::new(TokenType::IDENT, "y".into()),
        Token::new(TokenType::SEMICOLON, ";".into()),
        Token::new(TokenType::RBRACE, "}".into()),
        Token::new(TokenType::SEMICOLON, ";".into()),
        Token::new(TokenType::LET, "let".into()),
        Token::new(TokenType::IDENT, "result".into()),
        Token::new(TokenType::ASSIGN, "=".into()),
        Token::new(TokenType::IDENT, "add".into()),
        Token::new(TokenType::LPAREN, "(".into()),
        Token::new(TokenType::IDENT, "five".into()),
        Token::new(TokenType::COMMA, ",".into()),
        Token::new(TokenType::IDENT, "ten".into()),
        Token::new(TokenType::RPAREN, ")".into()),
        Token::new(TokenType::SEMICOLON, ";".into()),
        Token::new(TokenType::BANG, "!".into()),
        Token::new(TokenType::MINUS, "-".into()),
        Token::new(TokenType::SLASH, "/".into()),
        Token::new(TokenType::ASTERISK, "*".into()),
        Token::new(TokenType::INT, "5".into()),
        Token::new(TokenType::SEMICOLON, ";".into()),
        Token::new(TokenType::INT, "5".into()),
        Token::new(TokenType::LT, "<".into()),
        Token::new(TokenType::INT, "10".into()),
        Token::new(TokenType::GT, ">".into()),
        Token::new(TokenType::INT, "5".into()),
        Token::new(TokenType::SEMICOLON, ";".into()),
        Token::new(TokenType::IF, "if".into()),
        Token::new(TokenType::LPAREN, "(".into()),
        Token::new(TokenType::INT, "5".into()),
        Token::new(TokenType::LT, "<".into()),
        Token::new(TokenType::INT, "10".into()),
        Token::new(TokenType::RPAREN, ")".into()),
        Token::new(TokenType::LBRACE, "{".into()),
        Token::new(TokenType::RETURN, "return".into()),
        Token::new(TokenType::TRUE, "true".into()),
        Token::new(TokenType::SEMICOLON, ";".into()),
        Token::new(TokenType::RBRACE, "}".into()),
        Token::new(TokenType::ELSE, "else".into()),
        Token::new(TokenType::LBRACE, "{".into()),
        Token::new(TokenType::RETURN, "return".into()),
        Token::new(TokenType::FALSE, "false".into()),
        Token::new(TokenType::SEMICOLON, ";".into()),
        Token::new(TokenType::RBRACE, "}".into()),
        Token::new(TokenType::INT, "10".into()),
        Token::new(TokenType::EQ, "==".into()),
        Token::new(TokenType::INT, "10".into()),
        Token::new(TokenType::SEMICOLON, ";".into()),
        Token::new(TokenType::INT, "10".into()),
        Token::new(TokenType::NOTEQ, "!=".into()),
        Token::new(TokenType::INT, "9".into()),
        Token::new(TokenType::SEMICOLON, ";".into()),
    ];

    let mut l = Lexer::new(input);

    for (i, tt) in result.into_iter().enumerate() {
        let tok = l.next_token();
        assert_eq!(tt, tok);
        // println!("{:?}", tok);
    }
}
