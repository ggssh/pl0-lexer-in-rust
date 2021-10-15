use std::{collections::HashMap, usize};

/**
 * token定义及相关函数
 */
// Eq和Hash判断map中是否已有相等元素(其实没有必要)
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum TokenType {
    NULL,       // 未知类型(在不能识别的情况下输出NULL)
    IDENTIFIER, // 标识符
    NUMBER,     // 无符号整数
    PLUS,       // +
    MINUS,      // -
    TIMES,      // *
    SLASH,      // /
    ODDSYM,     // odd
    EQL,        // =
    NEQ,        // #
    LSS,        // <
    LEQ,        // <=
    GTR,        // >
    GEQ,        // >=
    LPAREN,     // (
    RPAREN,     // )
    COMMA,      // ,
    SEMICOLON,  // ;
    PERIOD,     // .
    BECOMES,    // :=
    BEGINSYM,   // begin
    ENDSYM,     // end
    IFSYM,      // if
    THENSYM,    // then
    WHILESYM,   // while
    WRITESYM,   // write
    READSYM,    // read
    DOSYM,      // do
    CALLSYM,    // call
    CONSTSYM,   // const
    VARSYM,     // var
    PROCSYM,    // procedure
}

// 一张 单词种别:对应的种别编码
#[derive(Debug, Clone)]
pub struct TokenMap {
    pub map: HashMap<TokenType, usize>,
}

impl TokenMap {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(TokenType::NULL, 0);
        map.insert(TokenType::IDENTIFIER, 1);
        map.insert(TokenType::NUMBER, 2);
        map.insert(TokenType::PLUS, 3);
        map.insert(TokenType::MINUS, 4);
        map.insert(TokenType::TIMES, 5);
        map.insert(TokenType::SLASH, 6);
        map.insert(TokenType::ODDSYM, 7);
        map.insert(TokenType::EQL, 8);
        map.insert(TokenType::NEQ, 9);
        map.insert(TokenType::LSS, 10);
        map.insert(TokenType::LEQ, 11);
        map.insert(TokenType::GTR, 12);
        map.insert(TokenType::GEQ, 13);
        map.insert(TokenType::LPAREN, 14);
        map.insert(TokenType::RPAREN, 15);
        map.insert(TokenType::COMMA, 16);
        map.insert(TokenType::SEMICOLON, 17);
        map.insert(TokenType::PERIOD, 18);
        map.insert(TokenType::BECOMES, 19);
        map.insert(TokenType::BEGINSYM, 20);
        map.insert(TokenType::ENDSYM, 21);
        map.insert(TokenType::IFSYM, 22);
        map.insert(TokenType::THENSYM, 23);
        map.insert(TokenType::WHILESYM, 24);
        map.insert(TokenType::WRITESYM, 25);
        map.insert(TokenType::READSYM, 26);
        map.insert(TokenType::DOSYM, 27);
        map.insert(TokenType::CALLSYM, 28);
        map.insert(TokenType::CONSTSYM, 29);
        map.insert(TokenType::VARSYM, 30);
        map.insert(TokenType::PROCSYM, 31);
        TokenMap { map }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tokentype: TokenType, // token类型
    pub literal: String,      // token值
}

impl Token {
    pub fn display(self, tokenmap: &TokenMap) {
        match tokenmap.map.get(&self.tokentype) {
            Some(num) => println!("({},\"{}\")", num, self.literal),
            None => todo!(),
        }
    }

    pub fn new<T: Into<String>>(tokentype: TokenType, literal: T) -> Self {
        Token {
            tokentype,
            literal: literal.into(),
        }
    }
}
