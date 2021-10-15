#[derive(Debug)]
pub struct Lexer {
    cur_token: String,
    cur_position: usize,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            cur_token: String::from(""),
            cur_position: 0,
        }
    }
}
