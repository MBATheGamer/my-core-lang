use super::TokenType;

pub fn lookup_identifier(identifier: &String) -> TokenType {
  return match identifier.as_str() {
    "fn" => TokenType::FUNCTION,
    "let" => TokenType::LET,
    _ => TokenType::IDENTIFIER
  };
}