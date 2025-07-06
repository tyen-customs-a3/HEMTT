use std::ops::Range;
use crate::{Ident, Value};

/// An entry in an enum definition
/// This represents a single enum value like `destructengine = 2`
#[derive(Debug, Clone, PartialEq)]
pub struct EnumEntry {
    /// The name of the enum value
    name: Ident,
    /// The value assigned to the enum entry
    value: Value,
    /// The span of this enum entry
    span: Range<usize>,
}

impl EnumEntry {
    /// Create a new enum entry
    pub fn new(name: Ident, value: Value, span: Range<usize>) -> Self {
        Self { name, value, span }
    }
    
    /// Get the name of the enum entry
    pub fn name(&self) -> &Ident {
        &self.name
    }
    
    /// Get the value of the enum entry
    pub fn value(&self) -> &Value {
        &self.value
    }
    
    /// Get the span of the enum entry
    pub fn span(&self) -> &Range<usize> {
        &self.span
    }
}

impl std::fmt::Display for EnumEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.name.value, self.value)
    }
}

/// An enum definition in a config file
/// ```cpp
/// enum {
///     destructengine = 2,
///     destructdefault = 6,
///     destructwreck = 7,
///     destructtree = 3,
///     destructtent = 4,
///     stabilizedinaxisx = 1,
///     stabilizedinaxesxyz = 4,
///     stabilizedinaxisy = 2,
///     stabilizedinaxesboth = 3,
///     destructno = 0,
///     stabilizedinaxesnone = 0,
///     destructman = 5,
///     destructbuilding = 1
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct EnumDef {
    /// The name of the enum (always "enum")
    name: Ident,
    /// The entries (enum values) in the enum
    entries: Vec<EnumEntry>,
    /// The span of the enum definition
    span: Range<usize>,
}

impl EnumDef {
    /// Create a new enum definition
    pub fn new(name: Ident, entries: Vec<EnumEntry>, span: Range<usize>) -> Self {
        Self { name, entries, span }
    }
    
    /// Get the name of the enum
    pub const fn name(&self) -> &Ident {
        &self.name
    }
    
    /// Get the entries in the enum
    pub fn entries(&self) -> &[EnumEntry] {
        &self.entries
    }
    
    /// Get the span of the enum definition
    pub fn span(&self) -> &Range<usize> {
        &self.span
    }
}

impl std::fmt::Display for EnumDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "enum {{")?;
        for (i, entry) in self.entries.iter().enumerate() {
            write!(f, "    {}", entry)?;
            if i < self.entries.len() - 1 {
                writeln!(f, ",")?;
            } else {
                writeln!(f)?;
            }
        }
        write!(f, "}};")
    }
}