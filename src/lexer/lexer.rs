pub struct Lexer {
  input: Vec<char>,
  position: usize,
  read_position: usize,
  ch: char
}

impl Lexer {
  pub fn new(input: &str) -> Lexer {
    let mut lexer = Lexer {
      input: input.chars().collect(),
      position: 0,
      read_position: 0,
      ch: Default::default()
    };

    lexer.read_char();

    return lexer;
  }

  fn read_char(&mut self) {
    if self.read_position >= self.input.len() {
      self.ch = '\0';
    } else {
      self.ch = self.input[self.read_position];
    }

    self.position = self.read_position;
    self.read_position += 1;
  }
}
