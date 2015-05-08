use super::input_position::InputPosition;

pub struct ParseError {
	error_msg: &'static str,
	pos: InputPosition
}

impl ParseError {
	pub fn new(m: &'static str, p: InputPosition) -> ParseError {
		ParseError { error_msg: m, pos: p }
	}

	pub fn get_error_message(&self) -> &str {
		self.error_msg
	}

	pub fn get_error_position(&self) -> &InputPosition {
		&self.pos
	}
}