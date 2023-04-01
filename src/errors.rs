use data::Span;

pub type ParseError<'a> = nom::Err<nom::error::Error<Span<'a>>>;
