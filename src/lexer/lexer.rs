pub struct Lexer {
  input: Vec<char>,
  position: usize,
  read_position: usize,
  ch: char
}

impl Lexer {
  pub fn new(input: &str) -> Lexer {
    let lexer = Lexer {
      input: input.chars().collect(),
      position: 0,
      read_position: 0,
      ch: Default::default()
    };

    return lexer;
  }
}
