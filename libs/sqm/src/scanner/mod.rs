use std::ops::Range;
use crate::lexer::Token;

use std::collections::HashMap;

/// Represents a class boundary in the token stream
#[derive(Debug, Clone)]
pub struct ClassBoundary {
    /// Token range for this class
    pub range: Range<usize>,
    /// Parent class ID if this is a nested class
    pub parent_id: Option<usize>,
    /// Depth in the class hierarchy (0 = root)
    pub depth: usize,
    /// Name of the class
    pub name: String,
    /// Token range for the class contents (between braces)
    pub contents_range: Range<usize>,
}

/// Represents all class boundaries found in a token stream
#[derive(Debug, Default)]
pub struct BoundaryMap {
    /// All class boundaries indexed by their position in discovery order
    pub boundaries: Vec<ClassBoundary>,
}

impl BoundaryMap {
    /// Creates a new empty boundary map
    pub fn new() -> Self {
        Self {
            boundaries: Vec::new(),
        }
    }

    /// Gets all root-level class boundaries
    pub fn root_classes(&self) -> impl Iterator<Item = &ClassBoundary> {
        self.boundaries.iter().filter(|b| b.parent_id.is_none())
    }

    /// Get all children of a given class
    pub fn children_of(&self, parent_start: usize) -> Vec<&ClassBoundary> {
        let parent_idx = self.boundaries.iter()
            .position(|b| b.range.start == parent_start);
            
        if let Some(idx) = parent_idx {
            self.boundaries.iter()
                .filter(|b| b.parent_id == Some(idx))
                .collect()
        } else {
            Vec::new()
        }
    }
}

/// Error types that can occur during boundary scanning
#[derive(Debug)]
pub enum ScanError {
    /// Unexpected token encountered
    UnexpectedToken {
        position: usize,
        expected: &'static str,
        found: Token,
    },
    /// Unclosed class definition
    UnclosedClass {
        class_start: usize,
    },
    /// Missing class name
    MissingClassName {
        position: usize,
    },
    /// Invalid input
    InvalidInput {
        message: String,
    },
} 