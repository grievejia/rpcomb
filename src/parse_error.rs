use input_stream::TextPosition;

pub struct ParseError<'a> {
	error_msg: &'a str,
	pos: TextPosition
}

impl<'a> ParseError<'a> {
	pub fn new(m: &str, p: TextPosition) -> ParseError {
		ParseError { error_msg: m, pos: p }
	}

	pub fn get_error_message(&'a self) -> &'a str {
		self.error_msg
	}

	pub fn get_error_position(&'a self) -> &'a TextPosition {
		&self.pos
	}
}