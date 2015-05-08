use internal::{ParseError, ParseOutput, InputStream};
use super::parser::Parser;
use std::str;

pub struct StringParser<'a> {
	text: &'a str
}

impl<'a> StringParser<'a> {
	pub fn new(t: &str) -> StringParser {
		StringParser { text: t }
	}
}

impl<'a, 'b> Parser<'b, &'b str> for StringParser<'a> {
	fn parse<T: InputStream<T>>(&self, input: &'b T) -> ParseOutput<T, &'b str> {
		let str_len = self.text.len();
		let input_str = str::from_utf8(&input.get_input()[..str_len]).ok().expect("Expect legal UTF8 encoding");
		if input_str == self.text {
			ParseOutput::Success(input.consume(str_len), input_str)
		}
		else {
			ParseOutput::Failure(ParseError::new("Unexpected byte", input.get_position()))
		}
	}
}

pub fn strp(b: &str) -> StringParser {
	StringParser::new(b)
}

#[test]
fn string_parser_test() {
	use internal::StringInputStream;

	let input = StringInputStream::new("abcde");
	let parser = strp("abc");
	match parser.parse(&input) {
		ParseOutput::Success(i, o) => {
			assert_eq!(o, "abc");
			assert_eq!(i.get_input(), "de".as_bytes());
			assert_eq!(i.get_position().get_line_number(), 1);
			assert_eq!(i.get_position().get_column_number(), 4);
		},
		_ => {
			panic!("Parsing should succeed");
		}
	}

	let input2 = StringInputStream::new("abdce");
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