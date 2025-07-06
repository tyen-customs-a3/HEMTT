use chumsky::error::Simple;
use crate::lexer::{Token, LexError};
use crate::scanner::ScanError;

/// Errors that can occur during parsing
#[derive(Debug)]
pub enum ParseError {
    /// Error in the lexer
    Lexer(Simple<char>),
    /// Error in the scanner
    Scanner(ScanError),
    /// Error in the parser
    Parser(Simple<Token>),
    /// Unexpected token
    UnexpectedToken(Token),
    /// Unexpected end of file
    UnexpectedEof,
    /// Expected equals sign but got something else
    ExpectedEquals,
    /// Expected closing bracket but got something else
    ExpectedClosingBracket,
    /// Expected opening brace but got something else
    ExpectedOpenBrace,
    /// Expected equals or closing bracket but got something else
    ExpectedEqualsOrBracket,
    /// Lexical analysis error from new lexer
    LexError(LexError),
    /// Error in the scanner
    ScanError(ScanError),
    /// Input file is too large to process safely
    InputTooLarge(usize),
}

/// Build a nice error message from a parsing error
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn emit_diagnostics(src: &str, error: &ParseError) -> String {
    match error {
        ParseError::Lexer(e) => {
            // Get error context
            let (line, col) = line_col_for_pos(src, e.span().start);
            let line_str = get_line(src, line);
            
            // Build error message
            let mut err_msg = format!("Lexer error at line {}, column {}:\n", line+1, col+1);
            err_msg.push_str(&line_str);
            err_msg.push_str(&" ".repeat(col));
            err_msg.push('^');
            err_msg.push_str(&format!("{e}"));
            
            err_msg
        }
        ParseError::LexError(e) => {
            match e {
                LexError::UnexpectedChar { pos, found } => {
                    // Get error context
                    let (line, col) = line_col_for_pos(src, *pos);
                    let line_str = get_line(src, line);
                    
                    // Build error message
                    let mut err_msg = format!("Unexpected character '{}' at line {}, column {}:\n", 
                                             found, line+1, col+1);
                    err_msg.push_str(&line_str);
                    err_msg.push_str(&" ".repeat(col));
                    err_msg.push('^');
                    
                    err_msg
                }
                LexError::UnterminatedString { start_pos } => {
                    // Get error context
                    let (line, col) = line_col_for_pos(src, *start_pos);
                    let line_str = get_line(src, line);
                    
                    // Build error message
                    let mut err_msg = format!("Unterminated string starting at line {}, column {}:\n", 
                                             line+1, col+1);
                    err_msg.push_str(&line_str);
                    err_msg.push_str(&" ".repeat(col));
                    err_msg.push('^');
                    
                    err_msg
                }
                LexError::InvalidToken { pos, message } => {
                    // Get error context
                    let (line, col) = line_col_for_pos(src, *pos);
                    let line_str = get_line(src, line);
                    
                    // Build error message
                    let mut err_msg = format!("Invalid token at line {}, column {}: {}\n", 
                                             line+1, col+1, message);
                    err_msg.push_str(&line_str);
                    err_msg.push_str(&" ".repeat(col));
                    err_msg.push('^');
                    
                    err_msg
                }
            }
        }
        ParseError::Scanner(e) | ParseError::ScanError(e) => {
            match e {
                ScanError::UnexpectedToken { position, expected, found } => {
                    // Get error context by finding the line and column for the token position
                    let (line, col) = find_token_position(src, *position);
                    let line_str = get_line(src, line);
                    
                    // Build error message with more context
                    let mut err_msg = format!("Scanner error at line {}, column {}: Expected {} but found {:?}\n", 
                                             line+1, col+1, expected, found);
                    err_msg.push_str(&line_str);
                    err_msg.push_str(&" ".repeat(col));
                    err_msg.push('^');
                    
                    err_msg
                }
                ScanError::UnclosedClass { class_start } => {
                    // Get error context for the class start position
                    let (line, col) = find_token_position(src, *class_start);
                    let line_str = get_line(src, line);
                    
                    // Build error message with more context
                    let mut err_msg = format!("Unclosed class starting at line {}, column {}:\n", 
                                            line+1, col+1);
                    err_msg.push_str(&line_str);
                    err_msg.push_str(&" ".repeat(col));
                    err_msg.push('^');
                    err_msg.push_str(" Missing closing brace for this class");
                    
                    err_msg
                }
                ScanError::MissingClassName { position } => {
                    // Get error context
                    let (line, col) = find_token_position(src, *position);
                    let line_str = get_line(src, line);
                    
                    // Build error message
                    let mut err_msg = format!("Missing class name at line {}, column {}:\n", 
                                            line+1, col+1);
                    err_msg.push_str(&line_str);
                    err_msg.push_str(&" ".repeat(col));
                    err_msg.push('^');
                    err_msg.push_str(" Expected a class name here");
                    
                    err_msg
                }
                ScanError::InvalidInput { message } => {
                    format!("Scanner error: {message}")
                }
            }
        }
        ParseError::Parser(e) => {
            let mut expected_tokens = Vec::new();
            for expected in e.expected() {
                match expected {
                    Some(token) => expected_tokens.push(format!("{token:?}")),
                    None => expected_tokens.push("end of file".to_string()),
                }
            }
            
            let found = e.found().map_or_else(
                || "end of file".to_string(),
                |token| format!("{token:?}")
            );
            
            // Get error context for better error messages
            let (line, col) = line_col_for_pos(src, e.span().start);
            let line_str = get_line(src, line);
            
            // Build error message
            let mut err_msg = format!(
                "Parser error at line {}, column {}: Expected one of {} but found {}\n",
                line+1, col+1, expected_tokens.join(", "), found
            );
            err_msg.push_str(&line_str);
            err_msg.push_str(&" ".repeat(col));
            err_msg.push('^');
            
            err_msg
        }
        ParseError::UnexpectedToken(token) => {
            format!("Unexpected token: {token:?}")
        }
        ParseError::UnexpectedEof => {
            "Unexpected end of file".to_string()
        }
        ParseError::ExpectedEquals => {
            "Expected equals sign".to_string()
        }
        ParseError::ExpectedClosingBracket => {
            "Expected closing bracket".to_string()
        }
        ParseError::ExpectedOpenBrace => {
            "Expected opening brace".to_string()
        }
        ParseError::ExpectedEqualsOrBracket => {
            "Expected equals sign or bracket".to_string()
        }
        ParseError::InputTooLarge(size) => {
            format!("Input file is too large: {} bytes (maximum 100MB allowed)", size)
        }
    }
}

/// Gets the line and column for a position in the source text
fn line_col_for_pos(src: &str, pos: usize) -> (usize, usize) {
    let mut line = 0;
    let mut col = 0;
    
    for (i, c) in src.char_indices() {
        if i >= pos {
            break;
        }
        
        if c == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    
    (line, col)
}

/// Gets a specific line from the source text
fn get_line(src: &str, line_num: usize) -> String {
    src.lines().nth(line_num).unwrap_or("").to_string() + "\n"
}

/// Find position of a token in the source by token index
fn find_token_position(src: &str, token_pos: usize) -> (usize, usize) {
    // This is an approximation based on token length estimation
    let lex_result = crate::lexer::tokenize(src);
    let tokens_until_position = lex_result.0.iter().take(token_pos + 1).collect::<Vec<_>>();
    
    // Estimate the byte position by counting token lengths
    let mut estimated_pos = 0;
    for token in &tokens_until_position {
        match token {
            // Single-character tokens: all have length 1
            Token::OpenBrace | Token::CloseBrace | Token::OpenBracket | 
            Token::CloseBracket | Token::Semicolon | Token::Equals | Token::Comma => {
                estimated_pos += 1;
            },
            // Word tokens
            Token::Define => estimated_pos += "#define".len(),
            Token::Class => estimated_pos += "class".len(),
            Token::Version => estimated_pos += "version".len(),
            // Value tokens
            Token::Identifier(s) | Token::NumberLit(s) => estimated_pos += s.len(),
            Token::StringLit(s) => estimated_pos += s.len() + 2, // +2 for quotes
            Token::Comment(_) => estimated_pos += 2, // "//" prefix
        }
        
        // Add whitespace approximation
        estimated_pos += 1;
    }
    
    // Convert estimated byte position to line and column
    line_col_for_pos(src, estimated_pos.min(src.len().saturating_sub(1)))
} 