use crate::token::token::*;

/**
 * 词法分析程序主体
 */
const CHAR0: char = 0 as char;

#[derive(Debug)]
pub struct Lexer {
    // todo:为了更好测试,先改为pub
    pub cur_char: char,   // lexer
    cur_line: usize,      // lexer当前处理行数
    cur_position: usize,  // lexer当前处理的该行的位置
    read_position: usize, // lexer处理的总位置
    input: String,        // lexer读取的文本
}

impl Lexer {
    // 获取一个lexer
    pub fn new<T: Into<String>>(input: T) -> Self {
        Lexer {
            cur_char: CHAR0,
            cur_position: 0,
            cur_line: 1,
            read_position: 0,
            input: input.into(),
        }
    }
    // 获取一个字符并更新位置等信息
    pub fn get_char(&mut self) {
        self.cur_char = self.input.chars().nth(self.read_position).unwrap_or(CHAR0);
        if self.cur_char == '\n' {
            self.cur_line += 1;
            self.cur_position = 0;
        } else {
            self.cur_position = self.read_position;
        }
        self.read_position += 1;
    }
    // 获取一个token
    pub fn get_next_token(&mut self) -> Token {
        let t: Token;
        // 跳过空格
        self.skip_whitespace();
        match self.cur_char {
            '+' => t = Token::new(TokenType::PLUS, self.cur_char),
            '-' => t = Token::new(TokenType::MINUS, self.cur_char),
            '*' => t = Token::new(TokenType::TIMES, self.cur_char),
            '/' => t = Token::new(TokenType::SLASH, self.cur_char),
            '=' => t = Token::new(TokenType::EQL, self.cur_char),
            '#' => t = Token::new(TokenType::NEQ, self.cur_char),
            '<' => {
                if self.peek_char() == '=' {
                    let cur_ch = self.cur_char;
                    self.get_char(); //更新当前字符
                    let mut temp = cur_ch.to_string();
                    temp.push(self.cur_char); //将新匹配的字符加到以匹配的字符后面作为一个token新的literal
                    t = Token::new(TokenType::LEQ, temp);
                } else {
                    t = Token::new(TokenType::LSS, self.cur_char);
                }
            }
            '>' => {
                // 与<=相同
                if self.peek_char() == '=' {
                    let cur_ch = self.cur_char;
                    self.get_char();
                    let mut temp = cur_ch.to_string();
                    temp.push(self.cur_char);
                    t = Token::new(TokenType::GEQ, temp);
                } else {
                    t = Token::new(TokenType::GTR, self.cur_char);
                }
            }
            '(' => t = Token::new(TokenType::LPAREN, self.cur_char),
            ')' => t = Token::new(TokenType::RPAREN, self.cur_char),
            ',' => t = Token::new(TokenType::COMMA, self.cur_char),
            ';' => t = Token::new(TokenType::SEMICOLON, self.cur_char),
            '.' => t = Token::new(TokenType::PERIOD, self.cur_char),
            ':' => {
                if self.peek_char() == '=' {
                    let cur_ch = self.cur_char;
                    self.get_char();
                    let mut temp = cur_ch.to_string();
                    temp.push(self.cur_char);
                    t = Token::new(TokenType::BECOMES, temp);
                } else {
                    t = Token::new(TokenType::NULL, CHAR0);
                }
            }
            _ => t = Token::new(TokenType::NULL, CHAR0),
        }
        // Token {
        //     tokentype: TokenType::NULL,
        //     literal: String::from(""),
        // }
        self.get_char();
        t
    }

    // 跳过空格
    fn skip_whitespace(&mut self) {
        loop {
            match self.cur_char {
                ' ' | '\n' | '\r' | '\t' => self.get_char(),
                _ => break,
            }
        }
    }
    // 判断是否为字母
    fn is_alpha(ch: char) -> bool {
        match ch {
            'a'..='z' | 'A'..='Z' => true,
            _ => false,
        }
    }
    // 判断是否为数字
    fn is_digit(ch: char) -> bool {
        match ch {
            '0'..='9' => true,
            _ => false,
        }
    }
    // 获取当前位置的一个字符(针对二元操作符)
    fn peek_char(&self) -> char {
        self.input.chars().nth(self.read_position).unwrap_or(CHAR0)
    }
}
