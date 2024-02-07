use crate::lexer::Lexer;
use crate::token::Token;
use crate::token::TokenType;

#[test]
fn test_next_token() {
  let input = "=+(){},;";

  let tests = vec![
    Token {
      token_type: TokenType::ASSIGN,
      literal: format!("=")
    },
    Token {
      token_type: TokenType::PLUS,
      literal: format!("+")
    },
    Token {
      token_type: TokenType::LPARAN,
      literal: format!("(")
    },
    Token {
      token_type: TokenType::RPARAN,
      literal: format!(")")
    },
    Token {
      token_type: TokenType::LBRACE,
      literal: format!("{{")
    },
    Token {
      token_type: TokenType::RBRACE,
      literal: format!("}}")
    },
    Token {
      token_type: TokenType::COMMA,
      literal: format!(",")
    },
    Token {
      token_type: TokenType::SEMICOLON,
      literal: format!(";")
    },
  ];

  let mut lexer = Lexer::new(input);

  for (index, expected_token) in tests.into_iter().enumerate() {
    let recv_token = lexer.next_token();

    assert_eq!(
      expected_token.token_type,
      recv_token.token_type,
      "tests[{index}] - TokenType wrong. expected={:?}. got={:?}",
      expected_token.token_type,
      recv_token.token_type
    );

    assert_eq!(
      expected_token.literal,
      recv_token.literal,
      "tests[{index}] - literal wrong. expected={}, got={}",
      expected_token.literal,
      recv_token.literal,
    )
  }
}