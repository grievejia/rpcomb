use internal::{ParseOutput, InputStream};
use parser::Parser;

pub struct ParserAdapter<P, F> {
	parser: P,
	func: F
}

impl<'t, P, F, O> Parser<'t> for ParserAdapter<P, F> where P: Parser<'t>, F: Fn(P::OutputType) -> O {
	type OutputType = O;

	fn parse<T: InputStream<'t, T>>(&self, input: T) -> ParseOutput<T, O> {
		match self.parser.parse(input) {
			ParseOutput::Success(i, o) => ParseOutput::Success(i, (self.func)(o)),
			ParseOutput::Failure(e) => ParseOutput::Failure(e)
		}

	}
}

pub fn rule<'t, P, F, O>(p: P, f: F) -> ParserAdapter<P, F> where P: Parser<'t>, F: Fn(P::OutputType) -> O {
	ParserAdapter { parser: p , func: f}
}

#[test]
fn parser_adapter_test() {
	use internal::StringInputStream;
	use parser::strp;

	let input = StringInputStream::new("1234-567");
	let parser = rule(
		strp("1234"),
		|s: &str| s.parse::<u32>().ok().expect("Str to int conversion failed")
	);
	match parser.parse(input) {
		ParseOutput::Success(i, o) => {
			assert_eq!(o, 1234u32);
			assert_eq!(i.get_input(), "-567");
			assert_eq!(i.get_position().get_line_number(), 1);
			assert_eq!(i.get_position().get_column_number(), 5);
		},
		_ => {
			panic!("Parsing should succeed");
		}
	}

	let input2 = StringInputStream::new("1324");
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