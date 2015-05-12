use internal::{ParseError, ParseOutput, InputStream};
use super::parser::Parser;
use regex::Regex;

pub struct RegexParser {
	reg_exp: Regex
}

impl RegexParser {
	pub fn new(t: &str) -> RegexParser {
		let re = match Regex::new(t) {
			Ok(re) => re,
			Err(err) => panic!("RegexParser error: {}", err)
		};
		RegexParser { reg_exp: re }
	}
}

impl<'t> Parser<'t> for RegexParser {
	type OutputType = &'t str;

	fn parse<T: InputStream<'t, T>>(&self, input: T) -> ParseOutput<T, &'t str> {
		let ref input_str = input.get_input();
		match self.reg_exp.find(input_str) {
			Some((0, end)) => {
				let ref ret_str = input_str[..end];
				ParseOutput::Success(input.consume(end), ret_str)
			}
			_ => ParseOutput::Failure(ParseError::new("Unexpected token", input.get_position()))
		}
	}
}

pub fn regexp(s: &str) -> RegexParser {
	RegexParser::new(s)
}

#[test]
fn regex_parser_test() {
	use internal::StringInputStream;

	let input = StringInputStream::new("1234-567");
	let parser = regexp(r"[:digit:]+");
	match parser.parse(input) {
		ParseOutput::Success(i, o) => {
			assert_eq!(o, "1234");
			assert_eq!(i.get_input(), "-567");
			assert_eq!(i.get_position().get_line_number(), 1);
			assert_eq!(i.get_position().get_column_number(), 5);
		},
		_ => {
			panic!("Parsing should succeed");
		}
	}

	let input2 = StringInputStream::new("abdce");
	match parser.parse(input2) {
		ParseOutput::Failure(err) => {
			assert_eq!(err.get_error_position().get_line_number(), 1);
			assert_eq!(err.get_error_position().get_column_number(), 1);
		},
		_ => {
			panic!("Parsing should fail");
		}
	}
}