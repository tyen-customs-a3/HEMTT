use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Define,
    Class,
    Version,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Semicolon,
    Equals,
    Comma,
    Identifier(String),
    StringLit(String),
    NumberLit(String),
    Comment(String),
}

impl Hash for Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Self::Identifier(s) | Self::StringLit(s) | Self::NumberLit(s) | Self::Comment(s) => s.hash(state),
            _ => {}
        }
    }
}

/// Error during lexical analysis
#[derive(Debug, Clone)]
pub enum LexError {
    /// Unexpected character
    UnexpectedChar {
        pos: usize,
        found: char,
    },
    /// Unterminated string
    UnterminatedString {
        start_pos: usize,
    },
    /// Invalid token
    InvalidToken {
        pos: usize,
        message: String,
    },
}

/// Position information for a token
#[derive(Debug, Clone)]
pub struct TokenPosition {
    /// Byte offset in source
    pub byte_offset: usize,
    /// Length in bytes
    pub byte_length: usize,
}

/// Integrated lexer and scanner that efficiently processes input in a single pass
#[derive(Default)]
pub struct IntegratedLexer {
    /// Token stream
    tokens: Vec<Token>,
    /// Errors encountered during lexing
    errors: Vec<LexError>,
    /// Mapping from token index to byte position
    token_positions: Vec<TokenPosition>,
    /// The original input text
    input: String,
}

impl IntegratedLexer {
    /// Create a new integrated lexer
    #[must_use]
    pub const fn new() -> Self {
        Self {
            tokens: Vec::new(),
            errors: Vec::new(),
            token_positions: Vec::new(),
            input: String::new(),
        }
    }

    /// Process the input and generate tokens
    #[allow(clippy::too_many_lines)]
    pub fn lex(&mut self, input: &str) -> &[Token] {
        self.input = input.to_string();
        self.tokens.clear();
        self.errors.clear();
        self.token_positions.clear();
        
        let mut pos = 0;
        let chars: Vec<char> = input.chars().collect();
        
        while pos < chars.len() {
            if let Some(&c) = chars.get(pos) {
                let token_start = pos;
                
                // Skip whitespace
                if c.is_whitespace() {
                    pos += 1;
                    continue;
                }
                
                // Handle comments
                if pos + 1 < chars.len() && c == '/' && chars.get(pos + 1) == Some(&'/') {
                    pos += 2; // Skip '//'
                    
                    // Find the end of the line
                    let mut comment = String::new();
                    while pos < chars.len() && chars.get(pos) != Some(&'\n') {
                        if let Some(ch) = chars.get(pos) {
                            comment.push(*ch);
                        }
                        pos += 1;
                    }
                    
                    self.add_token(Token::Comment(comment), token_start, pos - token_start);
                    continue;
                }
                
                // Handle symbols
                match c {
                    '{' => {
                        self.add_token(Token::OpenBrace, token_start, 1);
                        pos += 1;
                    }
                    '}' => {
                        self.add_token(Token::CloseBrace, token_start, 1);
                        pos += 1;
                    }
                    '[' => {
                        self.add_token(Token::OpenBracket, token_start, 1);
                        pos += 1;
                    }
                    ']' => {
                        self.add_token(Token::CloseBracket, token_start, 1);
                        pos += 1;
                    }
                    ';' => {
                        self.add_token(Token::Semicolon, token_start, 1);
                        pos += 1;
                    }
                    '=' => {
                        self.add_token(Token::Equals, token_start, 1);
                        pos += 1;
                    }
                    ',' => {
                        self.add_token(Token::Comma, token_start, 1);
                        pos += 1;
                    }
                    '#' => {
                        // Handle #define
                        if pos + 7 < chars.len() && 
                           &chars[pos..pos+7].iter().collect::<String>() == "#define" &&
                           (pos + 7 >= chars.len() || {
                               #[allow(clippy::unnecessary_map_or)]
                               let is_whitespace = chars.get(pos + 7).map_or(true, |c| c.is_whitespace());
                               is_whitespace
                           }) {
                            self.add_token(Token::Define, token_start, 7);
                            pos += 7;
                        } else {
                            self.errors.push(LexError::UnexpectedChar { pos, found: c });
                            pos += 1;
                        }
                    }
                    '"' => {
                        // Handle string literals with enhanced escape sequence support
                        pos += 1; // Skip opening quote
                        let mut string = String::new();
                        
                        while pos < chars.len() {
                            match chars.get(pos) {
                                Some(&'"') => {
                                    // Check for double quote (escaped quote)
                                    if pos + 1 < chars.len() && chars[pos + 1] == '"' {
                                        // For SQM format, we need to preserve the exact string representation
                                        string.push('"');
                                        string.push('"');
                                        pos += 2; // Skip both quotes
                                        continue;
                                    }
                                    // Single quote - end of string
                                    break;
                                }
                                Some(&ch) => {
                                    // For SQM format, we preserve all characters literally, including backslashes
                                    string.push(ch);
                                    pos += 1;
                                }
                                None => {
                                    self.errors.push(LexError::UnterminatedString { start_pos: token_start });
                                    return &self.tokens;
                                }
                            }
                        }
                        
                        if pos < chars.len() && chars[pos] == '"' {
                            self.add_token(Token::StringLit(string), token_start, pos - token_start + 1);
                            pos += 1; // Skip closing quote
                        } else {
                            self.errors.push(LexError::UnterminatedString { start_pos: token_start });
                        }
                    }
                    '-' | '0'..='9' => {
                        // Handle number literals
                        let mut is_negative = false;
                        
                        if c == '-' {
                            is_negative = true;
                            pos += 1;
                            if pos >= chars.len() || !chars.get(pos).is_some_and(|c| c.is_ascii_digit()) {
                                // Not a number, treat as an error
                                self.errors.push(LexError::InvalidToken { 
                                    pos: token_start, 
                                    message: "Expected digit after '-'".to_string() 
                                });
                                continue;
                            }
                        }
                        
                        // Parse integer part
                        let int_start = pos;
                        while pos < chars.len() && chars.get(pos).is_some_and(|c| c.is_ascii_digit()) {
                            pos += 1;
                        }
                        
                        // Parse fractional part
                        if pos < chars.len() && chars.get(pos) == Some(&'.') {
                            pos += 1;
                            while pos < chars.len() && chars.get(pos).is_some_and(|c| c.is_ascii_digit()) {
                                pos += 1;
                            }
                        }
                        
                        // Handle scientific notation (e.g., 1.23e+10 or 4.56e-3)
                        if pos + 1 < chars.len() && (chars.get(pos) == Some(&'e') || chars.get(pos) == Some(&'E')) {
                            // We've seen 'e' or 'E', save this position in case it's not a valid scientific notation
                            let e_pos = pos;
                            pos += 1;
                            
                            // Allow for + or - after 'e'
                            if pos < chars.len() && (chars.get(pos) == Some(&'+') || chars.get(pos) == Some(&'-')) {
                                pos += 1;
                            }
                            
                            // Must have at least one digit after e[+-]
                            if pos < chars.len() && chars.get(pos).is_some_and(|c| c.is_ascii_digit()) {
                                // Parse exponent digits
                                while pos < chars.len() && chars.get(pos).is_some_and(|c| c.is_ascii_digit()) {
                                    pos += 1;
                                }
                            } else {
                                // No digits found after 'e' or 'e+' or 'e-', so reset position to before 'e'
                                pos = e_pos;
                            }
                        }
                        
                        let num_str = if is_negative {
                            let digits = &input[int_start..pos];
                            format!("-{digits}")
                        } else {
                            input[token_start..pos].to_string()
                        };
                        
                        self.add_token(Token::NumberLit(num_str), token_start, pos - token_start);
                    }
                    'a'..='z' | 'A'..='Z' | '_' => {
                        // Handle identifiers and keywords
                        let word_start = pos;
                        while pos < chars.len() && (chars.get(pos).is_some_and(|c| c.is_alphanumeric()) || chars.get(pos) == Some(&'_')) {
                            pos += 1;
                        }
                        
                        let word = &input[word_start..pos];
                        match word {
                            "class" => self.add_token(Token::Class, token_start, 5),
                            "version" => self.add_token(Token::Version, token_start, 7),
                            _ => self.add_token(Token::Identifier(word.to_string()), token_start, word.len()),
                        }
                    }
                    _ => {
                        // Generate error for truly unexpected characters
                        // Only skip whitespace and common non-problematic characters
                        if c.is_whitespace() || c == '\\' {
                            pos += 1;
                        } else {
                            self.errors.push(LexError::UnexpectedChar { pos, found: c });
                            pos += 1;
                        }
                    }
                }
            } else {
                // Somehow pos is out of range
                break;
            }
        }
        
        &self.tokens
    }

    /// Get any lexical errors
    #[must_use]
    pub fn errors(&self) -> &[LexError] {
        &self.errors
    }

    /// Get token positions
    #[must_use]
    pub fn token_positions(&self) -> &[TokenPosition] {
        &self.token_positions
    }

    /// Add a token to the stream with its position information
    fn add_token(&mut self, token: Token, byte_offset: usize, byte_length: usize) {
        self.tokens.push(token);
        self.token_positions.push(TokenPosition {
            byte_offset,
            byte_length,
        });
    }

    /// Get the byte position for a token at a specific index
    #[must_use]
    pub fn get_token_byte_position(&self, token_idx: usize) -> Option<usize> {
        self.token_positions.get(token_idx).map(|pos| pos.byte_offset)
    }

    /// Get the token index for a byte position
    #[must_use]
    pub fn get_token_at_byte_position(&self, byte_pos: usize) -> Option<usize> {
        for (idx, pos) in self.token_positions.iter().enumerate() {
            if byte_pos >= pos.byte_offset && byte_pos < pos.byte_offset + pos.byte_length {
                return Some(idx);
            }
        }
        None
    }

    /// Scan the token stream for class boundaries
    /// 
    /// # Errors
    /// Returns a `ScanError` if there are issues with the structure of the token stream,
    /// such as unclosed classes or unexpected tokens
    /// 
    /// # Panics
    /// Will panic if the stack is empty but shouldn't be - indicates internal logic error
    #[allow(clippy::too_many_lines)]
    pub fn scan_boundaries(&self) -> Result<crate::scanner::BoundaryMap, crate::scanner::ScanError> {
        let mut map = crate::scanner::BoundaryMap::new();
        let tokens_len = self.tokens.len();

        // Stack to track class hierarchies - (class_token_pos, class_name, depth, boundary_index, parent_boundary_index)
        let mut stack: Vec<(usize, String, usize, usize, Option<usize>)> = Vec::new();
        
        // Iterate through tokens sequentially
        let mut pos = 0;
        
        while pos < tokens_len {
            match self.tokens.get(pos) {
                Some(Token::Class) => {
                    let class_token_pos = pos;
                    pos += 1;
                    
                    // Find class name
                    if pos >= tokens_len {
                        return Err(crate::scanner::ScanError::MissingClassName { position: class_token_pos + 1 });
                    }
                    
                    let class_name = match self.tokens.get(pos) {
                        Some(Token::Identifier(name)) => name.clone(),
                        _ => return Err(crate::scanner::ScanError::MissingClassName { position: pos }),
                    };
                    
                    pos += 1;
                    
                    // Find opening brace
                    if pos >= tokens_len || !matches!(self.tokens.get(pos), Some(Token::OpenBrace)) {
                        return Err(crate::scanner::ScanError::UnexpectedToken {
                            position: pos,
                            expected: "opening brace",
                            found: if pos < tokens_len {
                                self.tokens[pos].clone()
                            } else {
                                Token::Identifier("EOF".to_string())
                            },
                        });
                    }
                    
                    let brace_token_pos = pos;
                    pos += 1;
                    
                    // Content starts after the opening brace
                    let content_start_pos = pos;
                    
                    // Determine depth and parent for this class
                    let depth = stack.len();
                    
                    // Get parent boundary index
                    let parent_boundary_index = if stack.is_empty() {
                        None
                    } else {
                        // The parent's boundary index is in the 3rd position in our stack tuple
                        Some(stack.last().expect("Stack shouldn't be empty here").3)
                    };
                    
                    // Add class boundary to the map (initially with placeholder contents range)
                    let boundary_index = map.boundaries.len();
                    map.boundaries.push(crate::scanner::ClassBoundary {
                        range: class_token_pos..(brace_token_pos + 1), // Initial range includes up to opening brace
                        parent_id: parent_boundary_index,
                        depth,
                        name: class_name.clone(),
                        contents_range: content_start_pos..content_start_pos, // Temporary range, will update later
                    });
                    
                    // Push class info to stack with boundary index and parent info
                    stack.push((class_token_pos, class_name, depth, boundary_index, parent_boundary_index));
                }
                Some(Token::CloseBrace) => {
                    if !stack.is_empty() {
                        let (class_token_pos, _, _, boundary_index, _) = stack.pop().expect("Stack shouldn't be empty");
                        
                        // Update the class boundary with complete range
                        if let Some(boundary) = map.boundaries.get_mut(boundary_index) {
                            // Complete range from class token to closing brace (inclusive)
                            boundary.range = class_token_pos..(pos + 1);
                            
                            // Content range from after open brace to before close brace
                            let content_start = boundary.contents_range.start;
                            boundary.contents_range = content_start..pos;
                            
                            // Check for empty class and fix content range
                            if content_start == pos {
                                boundary.contents_range = content_start..content_start;
                            }
                        }
                    }
                    pos += 1;
                }
                _ => pos += 1,
            }
        }
        
        // Check for unclosed classes
        if !stack.is_empty() {
            let (class_token_pos, _, _, _, _) = stack.last().expect("Stack shouldn't be empty");
            return Err(crate::scanner::ScanError::UnclosedClass { class_start: *class_token_pos });
        }
        
        // Debug output for verification - only in debug mode or when testing
        #[cfg(any(debug_assertions, test))]
        {
            debug!("Generated boundaries:");
            for (i, boundary) in map.boundaries.iter().enumerate() {
                debug!("[{}] Class: {}, Range: {:?}, Contents: {:?}, Parent: {:?}, Depth: {}",
                    i, boundary.name, boundary.range, boundary.contents_range, boundary.parent_id, boundary.depth);
                
                // Verify tokens in content range
                debug!("  Content tokens for {}:", boundary.name);
                let content_range = boundary.contents_range.clone();
                for token_idx in content_range {
                    if token_idx < self.tokens.len() {
                        debug!("    Token[{}]: {:?}", token_idx, self.tokens[token_idx]);
                    }
                }
            }
        }
        
        Ok(map)
    }
}

/// Process input text and return tokens and any errors
/// 
/// A direct replacement for the legacy lex function using `IntegratedLexer`.
pub fn tokenize(input: &str) -> (Vec<Token>, Vec<LexError>) {
    let mut lexer = IntegratedLexer::new();
    let tokens = lexer.lex(input).to_vec();
    let errors = lexer.errors().to_vec();
    (tokens, errors)
}