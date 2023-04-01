//! Error representations, primarily related to parsing failure.

use crate::data::Span;

/// Represents a failure in parsing at some point in the combinator chain. Parsing fails eagerly;
/// that is, the first failure point hit will exit with an error. This type returns an error that
/// includes information on the segment of bytes being parsed and the location of the cursor at
/// time of failure.

pub type ParseError<'a> = nom::Err<nom::error::Error<Span<'a>>>;
