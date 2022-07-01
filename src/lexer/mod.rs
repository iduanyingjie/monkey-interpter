use crate::token::{Token, TokenType};

#[cfg(test)]
mod tests;

/// Lexer 中的大多数字段很容易理解，但 position 和 readPosition 的含义可能让人
/// 困惑。两者都可以用作索引来访问 input 中的字符，例如 l.input[l.readPosition]。
/// 这里之所以用两个“指针”来指向所输入的字符串，是因为词法分析器除了查看当前
/// 字符，还需要进一步“查看”字符串，即查看字符串中的下一个字符。readPosition
/// 始终指向所输入字符串中的“下一个”字符，position 则指向所输入字符串中与 ch
/// 字节对应的字符。
struct Lexer {
    chars: Vec<char>,
    position: usize,      // 所输入字符串中的当前位置（指向当前字符）
    read_position: usize, // 所输入字符串中的当前读取位置（指向当前字符之后的一个字符）
    ch: char,             // 当前正在查看的字符
}

impl Lexer {
    fn new(input: &str) -> Self {
        let mut l = Lexer {
            chars: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: 0 as char,
        };

        l.read_char();

        l
    }

    fn read_char(&mut self) {
        while let Some(ch) = self.chars.get(self.read_position) {
            if ch.is_whitespace() {
                self.read_position += 1;
            } else {
                self.ch = *ch;
                self.position = self.read_position;
                self.read_position += 1;
                break;
            }
        }

        if let Some(ch) = self.chars.get(self.read_position) {
            self.ch = *ch;
            self.position = self.read_position;
            self.read_position += 1;
        } else {
            self.ch = 0 as char;
        }
    }

    pub fn next_token(&mut self) -> Token {
        let ch = self.ch;
        let (tok, need_read_char) = match ch {
            '=' => (Token::from_char(TokenType::ASSIGN, ch), true),
            ';' => (Token::from_char(TokenType::SEMICOLON, ch), true),
            '(' => (Token::from_char(TokenType::LPAREN, ch), true),
            ')' => (Token::from_char(TokenType::RPAREN, ch), true),
            ',' => (Token::from_char(TokenType::COMMA, ch), true),
            '+' => (Token::from_char(TokenType::PLUS, ch), true),
            '{' => (Token::from_char(TokenType::LBRACE, ch), true),
            '}' => (Token::from_char(TokenType::RBRACE, ch), true),
            c if c.is_letter() => {
                let literal = self.read_identifier();
                let token_type = TokenType::lookup_ident(&literal);
                (Token::new(token_type, literal), false)
            }
            c if c.is_numeric() => {
                let literal = self.read_number();
                (Token::new(TokenType::INT, literal), false)
            }
            _ => (Token::from_char(TokenType::ILLEGAL, ch), true),
        };

        if need_read_char {
            self.read_char();
        }

        tok
    }

    /// 先处理标识符和关键字。对于这两者，词法分析器需要识别当前字符是否为字母。
    /// 如果是，则还需要读取标识符/关键字的剩余部分，直到遇见非字母字符为止。读取完
    /// 该标识符/关键字之后，还需要判断它到底是标识符还是关键字，以便使用正确的
    /// token.TokenType。
    /// readIdentifier()函数顾名思义，就是读入一个标识符并前移词法分析器的扫描
    /// 位置，直到遇见非字母字符。
    fn read_identifier(&mut self) -> String {
        let prev = self.position;
        while self.ch.is_letter() {
            self.read_char();
        }
        self.chars
            .get(prev..self.position)
            .unwrap()
            .iter()
            .collect::<String>()
    }

    fn read_number(&mut self) -> String {
        let prev = self.position;
        while self.ch.is_numeric() {
            self.read_char();
        }
        self.chars
            .get(prev..self.position)
            .unwrap()
            .iter()
            .collect::<String>()
    }
}

trait IsLetter {
    fn is_letter(&self) -> bool;
}

impl IsLetter for char {
    fn is_letter(&self) -> bool {
        match self {
            '_' => true,
            _ => self.is_alphabetic(),
        }
    }
}
