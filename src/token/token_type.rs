#[derive(Debug, PartialEq)]
pub enum TokenType {
  ILLEGAL,
  EOF,

  IDENTIFIER,
  INT,

  ASSIGN,
  PLUS,

  COMMA,
  SEMICOLON,

  LPARAN,
  RPARAN,
  LBRACE,
  RBRACE,

  FUNCTION,
  LET
}
