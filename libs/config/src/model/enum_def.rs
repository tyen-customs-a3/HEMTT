use std::ops::Range;
use crate::{Property, Ident};

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
    pub name: Ident,
    /// The properties (enum values) in the enum
    pub properties: Vec<Property>,
    /// The span of the enum definition
    pub span: Range<usize>,
} 