use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser {
    /// lexer 是指向词法分析器实例的指针，在该实例上重复调用NextToken()能不断获取输入中的下一个词法单元
    lexer: Lexer,
    /// curr_token 和 peek_token 的行为与词法分析器中的两个“指针”position 和 readPosition 完全
    /// 相同，但它们分别指向输入中的当前词法单元和下一个词法单元，而不是输入中的字符。
    /// 查看 curr_token（当前正在检查的词法单元）是为了决定下一步该怎么做，
    /// 如果 curr_token 没有提供足够的信息，还需要根据 peek_token 来做决策。
    curr_token: Token,
    peek_token: Token,
}

impl Parser {

    pub fn new(lexer: Lexer) -> Self {
        let parser = Parser{lexer};
    }

}