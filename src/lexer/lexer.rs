use crate::token::{Token, TokenType};

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

  fn new_token(&self, token_type: TokenType, literal: char) -> Token {
    return Token {
      token_type: token_type,
      literal: literal.to_string()
    };
  }

  pub fn next_token(&mut self) -> Token {
    let token = match self.ch {
      '=' => self.new_token(TokenType::ASSIGN, self.ch),
      '+' => self.new_token(TokenType::PLUS, self.ch),
      '(' => self.new_token(TokenType::LPARAN, self.ch),
      ')' => self.new_token(TokenType::RPARAN, self.ch),
      '{' => self.new_token(TokenType::LBRACE, self.ch),
      '}' => self.new_token(TokenType::RBRACE, self.ch),
      ',' => self.new_token(TokenType::COMMA, self.ch),
      ';' => self.new_token(TokenType::SEMICOLON, self.ch),
      '\0' => self.new_token(TokenType::EOF, self.ch),
      _ => self.new_token(TokenType::ILLEGAL, self.ch)
    };

    self.read_char();

    return token;
  }
}
