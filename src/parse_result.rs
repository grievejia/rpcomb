use parse_error::ParseError;

pub enum ParseResult<'a, OutputType> {
	Success(OutputType),
	Failure(ParseError<'a>)
}
