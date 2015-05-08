pub use super::parse_error::ParseError;

pub enum ParseResult<OutputType> {
	Success(OutputType),
	Failure(ParseError)
}

pub enum ParseOutput<InputType, OutputType> {
	Success(InputType, OutputType),
	Failure(ParseError)
}
