use internal::{ParseError, ParseOutput, InputStream};
use super::parser::Parser;

pub struct ByteParser<'a> {
	text: &'a [u8]
}

impl<'a> ByteParser<'a> {
	pub fn new(t: &[u8]) -> ByteParser {
		ByteParser { text: t }
	}
}

impl<'a, 'b> Parser<'b, &'b [u8]> for ByteParser<'a> {
	fn parse<T: InputStream<T>>(&self, input: &'b T) -> ParseOutput<T, &'b [u8]> {
		let byte_len = self.text.len();
		let input_bytes = &input.get_input()[..byte_len];
		if input_bytes == self.text {
			ParseOutput::Success(input.consume(byte_len), input_bytes)
		}
		else {
			ParseOutput::Failure(ParseError::new("Unexpected byte", input.get_position()))
		}
	}
}

pub fn bytep(b: &[u8]) -> ByteParser {
	ByteParser::new(b)
}

#[test]
fn byte_parser_test() {
	use internal::ByteInputStream;

	let input = ByteInputStream::new("abcde".as_bytes());
	let parser = bytep("abc".as_bytes());
	match parser.parse(&input) {
		ParseOutput::Success(i, o) => {
			assert_eq!(o, "abc".as_bytes());
			assert_eq!(i.get_input(), "de".as_bytes());
			assert_eq!(i.get_position().get_line_number(), 1);
			assert_eq!(i.get_position().get_column_number(), 4);
		},
		_ => {
			panic!("Parsing should succeed");
		}
	}

	let input2 = ByteInputStream::new("abdce".as_bytes());
	match parser.parse(&input2) {
		ParseOutput::Failure(err) => {
			assert_eq!(err.get_error_message(), "Unexpected byte");
			assert_eq!(err.get_error_position().get_line_number(), 1);
			assert_eq!(err.get_error_position().get_column_number(), 1);
		},
		_ => {
			panic!("Parsing should fail");
		}
	}
}