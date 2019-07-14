
#[derive(Debug)]
pub struct Position {
    pub line: usize,
    pub column: usize
}

impl Position {
    fn new(line: usize, column: usize) -> Position {
        Position {
            line,
            column
        }
    }
}

#[derive(Debug)]
pub struct TokenInfo {
    pub token: Token,
    pub position: Position
}

impl TokenInfo {
    fn new(token: Token, position: Position) -> TokenInfo {
        TokenInfo { token, position }
    }
}

#[derive(Debug)]
pub enum Token {
    NUMBER { value: i32 },
    OPERATOR { value: char }
}

pub struct Tokenizer {
    source: Vec<String>, //Lines
    line: usize,
    column: usize
}

#[derive(Debug)]
pub struct TokenError {
    message: &'static str,
    position: Position
}

impl TokenError {
    fn new(message: &'static str, position: Position) -> TokenError {
        TokenError { message, position }
    }
}

impl Tokenizer {

    pub fn new(source: Vec<String>) -> Tokenizer {
        Tokenizer { source, line: 0, column: 0 }
    }

    pub fn tokenize(&mut self) -> (Vec<TokenInfo>, Vec<TokenError>) {
        let mut tokens: Vec<TokenInfo> = Vec::new();
        let mut errors: Vec<TokenError> = Vec::new();
        while self.line < self.source.len() {
            self.column = 0;
            let (mut toks, mut errs) = self.tokenize_line();
            tokens.append(&mut toks);
            errors.append(&mut errs);
            self.line += 1;
        }
        (tokens, errors)
    }

    fn tokenize_line(&mut self) -> (Vec<TokenInfo>, Vec<TokenError>) {
        let mut tokens: Vec<TokenInfo> = Vec::new();
        let mut errors: Vec<TokenError> = Vec::new();

        self.skip_spaces();

        while self.line_remaining() {
            let result = if self.peek().is_numeric() {
                self.read_number()
            } else if self.is_operator() {
                self.read_operator()
            } else {
                self.consume();
                Err(TokenError::new("Invalid token.",
                                    Position::new(self.line, self.column)))
            };
            match result {
                Ok(token) => tokens.push(token),
                Err(err) => errors.push(err)
            }
            self.skip_spaces();
        }

        (tokens, errors)
    }

    fn read_number(&mut self) -> Result<TokenInfo, TokenError> {
        let column = self.column;
        let mut value: i32 = 0;
        while self.line_remaining() && self.peek().is_numeric() {
            match self.consume().to_digit(10) {
                Some(n) => value = value * 10 + (n as i32),
                None => return Err(
                    TokenError::new("Error parsing integer",
                                    Position::new(self.line, column)))
            }
        }
        Ok(TokenInfo {
            token: Token::NUMBER { value },
            position: Position::new(self.line, column)
        })
    }

    fn read_operator(&mut self) -> Result<TokenInfo, TokenError> {
        let column = self.column;
        let value = self.consume();
        Ok(TokenInfo::new(Token::OPERATOR { value }, Position::new(self.line, column)))
    }

    fn skip_spaces(&mut self) {
        let line = &self.source[self.line];
        while self.column < line.len() && self.is_ignored() {
            self.column += 1;
        }
    }

    #[inline]
    fn consume(&mut self) -> char {
        let c = self.peek();
        self.column += 1;
        c
    }

    fn is_ignored(&self) -> bool {
        match self.peek() {
            ' ' => true,
            '\r' => true,
            '\t' => true,
            _ => false
        }
    }

    fn is_operator(&self) -> bool {
        match self.peek() {
            '+' => true,
            '-' => true,
            '*' => true,
            '/' => true,
            _ => false
        }
    }

    #[inline]
    fn peek(&self) -> char {
        let line = &self.source[self.line];
        line.chars().nth(self.column).unwrap()
    }

    fn line_remaining(&self) -> bool {
        self.column < self.source[self.line].len()
    }


}