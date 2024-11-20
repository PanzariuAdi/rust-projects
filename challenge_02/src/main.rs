use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
enum Token {
    CurlyOpen,
    CurlyClose,
    BracketOpen,
    BracketClose,
    Colon,
    Comma,
    String(String),
}

#[derive(PartialEq, Debug)]
enum ParseError {
    UnexpectedToken(String),
}

#[derive(Debug, Clone, PartialEq)]
enum JsonValue {
    String(String),
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    Null,
}

fn tokenize(json: &str) -> Result<Vec<Token>, Vec<ParseError>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut errors: Vec<ParseError> = Vec::new();
    let mut chars = json.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '{' => tokens.push(Token::CurlyOpen),
            '}' => tokens.push(Token::CurlyClose),
            '[' => tokens.push(Token::BracketOpen),
            ']' => tokens.push(Token::BracketClose),
            ':' => tokens.push(Token::Colon),
            ',' => tokens.push(Token::Comma),
            '"' => {
                let mut key = String::new();

                while let Some(next_char) = chars.next() {
                    if next_char == '"' {
                        break;
                    }
                    key.push(next_char);
                }
                tokens.push(Token::String(key));
            }
            ' ' | '\n' | '\t' => {}
            _ => {
                let mut unexpected_token = String::new();

                while let Some(next_char) = chars.next() {
                    match next_char {
                        ' ' | '\n' | '\t' => {
                            break;
                        }
                        ch => unexpected_token.push(ch),
                    }
                }

                errors.push(ParseError::UnexpectedToken(unexpected_token));
            }
        }
    }

    if errors.len() > 0 {
        return Err(errors);
    }

    Ok(tokens)
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn parse(&mut self) -> Result<JsonValue, String> {
        match self.peek() {
            Some(Token::CurlyOpen) => self.parse_object(),
            _ => Err("JSON must begin with an object or array".to_string()),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        let mut map: HashMap<String, JsonValue> = HashMap::new();
        self.expect(Token::CurlyOpen);

        while let Some(token) = self.peek().cloned() {
            match token {
                Token::CurlyClose => {
                    self.advance();
                    break;
                }
                Token::String(key) => {
                    self.advance(); // consume key
                    self.expect(Token::Colon); // expect ; and consume it
                    let value = self.parse_value()?;

                    map.insert(key, value);
                }
                _ => return Err("Invalid JSON!".to_string()),
            }
        }

        self.expect(Token::CurlyClose);
        Ok(JsonValue::Object(map))
    }

    fn parse_value(&mut self) -> Result<JsonValue, String> {
        if let Some(token) = self.peek().cloned() {
            match token {
                Token::String(value) => {
                    self.advance();
                    Ok(JsonValue::String(value.clone()))
                }
                Token::CurlyOpen => self.parse_object(),
                Token::CurlyClose => {
                    self.advance();
                    Ok(JsonValue::Object(HashMap::new()))
                }
                _ => Err("no more tokens".to_string()),
            }
        } else {
            Err("no more tokens".to_string())
        }
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.current);
        self.current += 1;
        token
    }

    fn expect(&mut self, expected: Token) {
        if self.peek() == Some(&expected) {
            self.advance();
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn tokenize_should_tokenize_brackets() {
        let mut expected: Vec<Token> = Vec::new();
        expected.push(Token::CurlyOpen);
        expected.push(Token::CurlyClose);

        let result = match tokenize("{}") {
            Ok(result) => result,
            Err(_err) => panic!("error at test"),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn tokenize_should_tokenize_key_value() {
        let mut expected: Vec<Token> = Vec::new();
        expected.push(Token::CurlyOpen);
        expected.push(Token::String("key".to_string()));
        expected.push(Token::Colon);
        expected.push(Token::String("value".to_string()));
        expected.push(Token::CurlyClose);

        let result = match tokenize("{\"key\": \"value\"}") {
            Ok(result) => result,
            Err(_err) => panic!("error at test"),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn tokenize_should_tokenize_key_value_comma() {
        let mut expected: Vec<Token> = Vec::new();
        expected.push(Token::CurlyOpen);
        expected.push(Token::String("key".to_string()));
        expected.push(Token::Colon);
        expected.push(Token::String("value".to_string()));
        expected.push(Token::Comma);
        expected.push(Token::CurlyClose);

        let result = match tokenize("{\"key\": \"value\",}") {
            Ok(result) => result,
            Err(_err) => panic!("error at test"),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn tokenize_should_tokenize_empty_object() {
        let mut expected: Vec<Token> = Vec::new();
        expected.push(Token::CurlyOpen);
        expected.push(Token::String("key".to_string()));
        expected.push(Token::Colon);
        expected.push(Token::CurlyOpen);
        expected.push(Token::CurlyClose);
        expected.push(Token::CurlyClose);

        let result = match tokenize("{\"key\": {}}") {
            Ok(result) => result,
            Err(_err) => panic!("error at test"),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn tokenize_should_tokenize_object() {
        let mut expected: Vec<Token> = Vec::new();
        expected.push(Token::CurlyOpen);
        expected.push(Token::String("key".to_string()));
        expected.push(Token::Colon);
        expected.push(Token::CurlyOpen);
        expected.push(Token::String("key".to_string()));
        expected.push(Token::Colon);
        expected.push(Token::String("value".to_string()));
        expected.push(Token::CurlyClose);
        expected.push(Token::CurlyClose);

        let result = match tokenize("{\"key\": {\"key\": \"value\"}}") {
            Ok(result) => result,
            Err(_err) => panic!("error at test"),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn tokenize_should_tokenize_array() {
        let mut expected: Vec<Token> = Vec::new();
        expected.push(Token::CurlyOpen);
        expected.push(Token::String("key".to_string()));
        expected.push(Token::Colon);
        expected.push(Token::BracketOpen);
        expected.push(Token::BracketClose);
        expected.push(Token::CurlyClose);

        let result = match tokenize("{\"key\": []}") {
            Ok(result) => result,
            Err(_err) => panic!("error at test"),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn parser_should_parse_empty_string() {
        let mut tokens: Vec<Token> = Vec::new();
        tokens.push(Token::CurlyOpen);
        tokens.push(Token::CurlyClose);

        let mut parser = Parser::new(tokens);

        let result = parser.parse();
        let expected = JsonValue::Object(HashMap::new());

        assert_eq!(expected, result.expect("test parse empty gone wrong"));
    }

    #[test]
    fn parser_should_parse_key_value() {
        let mut tokens: Vec<Token> = Vec::new();
        tokens.push(Token::CurlyOpen);
        tokens.push(Token::String("key".to_string()));
        tokens.push(Token::Colon);
        tokens.push(Token::String("value".to_string()));
        tokens.push(Token::CurlyClose);

        let mut parser = Parser::new(tokens);
        let mut map = HashMap::new();
        map.insert("key".to_string(), JsonValue::String("value".to_string()));

        let result = parser.parse();
        let expected = JsonValue::Object(map);

        assert_eq!(expected, result.expect("test parse key value gone wrong"));
    }

    #[test]
    fn parser_should_parse_key_empty_object() {
        let mut tokens: Vec<Token> = Vec::new();
        tokens.push(Token::CurlyOpen);
        tokens.push(Token::String("key".to_string()));
        tokens.push(Token::Colon);
        tokens.push(Token::CurlyOpen);
        tokens.push(Token::CurlyClose);
        tokens.push(Token::CurlyClose);

        let mut parser = Parser::new(tokens);
        let mut map = HashMap::new();
        map.insert("key".to_string(), JsonValue::Object(HashMap::new()));

        let result = parser.parse();
        let expected = JsonValue::Object(map);

        assert_eq!(expected, result.expect("test parse key object gone wrong"));
    }

    #[test]
    fn parser_should_parse_key_populated_object() {
        let mut tokens: Vec<Token> = Vec::new();
        tokens.push(Token::CurlyOpen);
        tokens.push(Token::String("key".to_string()));
        tokens.push(Token::Colon);
        tokens.push(Token::CurlyOpen);
        tokens.push(Token::String("key".to_string()));
        tokens.push(Token::Colon);
        tokens.push(Token::String("value".to_string()));
        tokens.push(Token::CurlyClose);
        tokens.push(Token::CurlyClose);

        let mut parser = Parser::new(tokens);

        let mut root = HashMap::new();
        let mut child_1 = HashMap::new();

        child_1.insert("key".to_string(), JsonValue::String("value".to_string()));
        root.insert("key".to_string(), JsonValue::Object(child_1));

        let result = parser.parse();
        let expected = JsonValue::Object(root);

        assert_eq!(expected, result.expect("test parse key object gone wrong"));
    }

    #[test]
    fn parser_should_parse_array() {
        let mut tokens: Vec<Token> = Vec::new();
        tokens.push(Token::CurlyOpen);
        tokens.push(Token::BracketOpen);

        tokens.push(Token::CurlyOpen);
        tokens.push(Token::String("key".to_string()));
        tokens.push(Token::Colon);
        tokens.push(Token::String("value".to_string()));
        tokens.push(Token::CurlyClose);
        tokens.push(Token::Comma);

        tokens.push(Token::CurlyOpen);
        tokens.push(Token::String("key".to_string()));
        tokens.push(Token::Colon);
        tokens.push(Token::String("value".to_string()));
        tokens.push(Token::CurlyClose);

        tokens.push(Token::BracketClose);
        tokens.push(Token::CurlyClose);

        let mut parser = Parser::new(tokens);

        let mut root = HashMap::new();
        let mut child_1 = HashMap::new();

        child_1.insert("key".to_string(), JsonValue::String("value".to_string()));
        root.insert("key".to_string(), JsonValue::Object(child_1));

        let result = parser.parse();
        let expected = JsonValue::Object(root);

        assert_eq!(expected, result.expect("test parse key object gone wrong"));
    }
}
