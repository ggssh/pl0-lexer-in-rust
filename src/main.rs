use crate::token::token::TokenMap;

mod common;
mod err;
mod lexer;
mod token;

fn main() {
    println!("Hello, world!");
}

// 以下为TDD部分
#[test]
fn read_in_test() {
    let s = common::read_in("README.md".to_owned());
    println!("{}", &s);
}
#[test]
fn lexer_new_test() {
    let input = common::read_in("test/test3.pl0".to_owned());
    println!("{}", input);
    let mut l = lexer::lexer::Lexer::new(input);
    let token_map = TokenMap::new();
    // bugfix : 读第一个token时会返回NULL
    let a = l.get_next_token();
    a.display(&token_map);
    // println!("{:#?}", &a);
    let a = l.get_next_token();
    a.display(&token_map);
    // println!("{:#?}", &a);
    let a = l.get_next_token();
    a.display(&token_map);
    // println!("{:#?}", &a);
    let a = l.get_next_token();
    a.display(&token_map);
    // println!("{:#?}", &a);
    let a = l.get_next_token();
    a.display(&token_map);
    let a = l.get_next_token();
    a.display(&token_map);
    let a = l.get_next_token();
    a.display(&token_map);
    let a = l.get_next_token();
    a.display(&token_map);
    let a = l.get_next_token();
    a.display(&token_map);
    // println!("{:#?}", &a);
    // println!("{:#?}", &l.cur_char);
    // l.get_char();
    // println!("{:#?}", &l.cur_char);
    // l.get_char();
    // println!("{:#?}", &l.cur_char);
    // l.get_char();
    // println!("{:#?}", &l.cur_char);
}
