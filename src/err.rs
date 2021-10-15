use crate::token::{self, token::Token};
use std::usize;

/**
 * 错误处理
 */
pub fn error_handler(cur_line: usize, cur_position: usize, token: Token) {
    eprintln!(
        "unregonized words : {} at {},{}",
        token.literal, cur_line, cur_position
    );
}
