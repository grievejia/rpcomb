use internal::{ParseOutput, InputStream};
use parser::Parser;

pub struct TokenParser<P> {
	parser: P
}

impl<'t, P> Parser<'t> for TokenParser<P> where P: Parser<'t> {
	type OutputType = P::OutputType;

	fn parse<T: InputStream<'t, T>>(&self, input: T) -> ParseOutput<T, Self::OutputType> {
		let mut i = 0;
		for ch in input.get_input().chars() {
			match ch {
				' ' | '\n' | '\t' => i += 1,
				_ => break
			}
		}

		let new_input = input.consume(i);
		self.parser.parse(new_input)
	}
}

pub fn tokenp<'t, P: Parser<'t>>(p: P) -> TokenParser<P> {
	TokenParser { parser: p }
}

#[test]
fn token_parser_test() {
	use internal::StringInputStream;
	use parser::strp;

	let input = StringInputStream::new("  abcde");
	let parser = tokenp(strp("abc"));

	match parser.parse(input) {
		ParseOutput::Success(i, o) => {
			assert_eq!(o, "abc");
			assert_eq!(i.get_input(), "de");
			assert_eq!(i.get_position().get_line_number(), 1);
			assert_eq!(i.get_position().get_column_number(), 6);
		},
		_ => {
			panic!("Parsing should succeed");
		}
	}

	let input2 = StringInputStream::new("  abdce");
	match parser.parse(input2) {
		ParseOutput::Failure(err) => {
			assert_eq!(err.get_error_position().get_line_number(), 1);
			assert_eq!(err.get_error_position().get_column_number(), 3);
		},
		_ => {
			panic!("Parsing should fail");
		}
	}
}