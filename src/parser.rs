use input_stream::InputStream;

pub struct ParserOutput<'a, OutputType> {
	attr: Option<OutputType>,
	remaining_input: InputStream<'a>
}

impl<'a, OutputType> ParserOutput<'a, OutputType> {
	pub fn succ(a: OutputType, i: InputStream) -> ParserOutput<OutputType> {
		ParserOutput { attr: Some(a), remaining_input: i }
	}

	pub fn fail(i: InputStream) -> ParserOutput<OutputType> {
		ParserOutput { attr: None, remaining_input: i }
	}

	pub fn is_valid(&self) -> bool {
		self.attr.is_some()
	}

	pub fn get_output(&self) -> &Option<OutputType> {
		&self.attr
	}

	pub fn get_remaining_input(&self) -> &InputStream {
		&self.remaining_input
	}
}

trait Parser<'a, OutputType> {
	fn parse(&self, &'a InputStream) -> ParserOutput<'a, OutputType>;
}

pub struct StringParser<'a> {
	text: &'a str
}

impl<'a> StringParser<'a> {
	pub fn new(t: &str) -> StringParser {
		StringParser { text: t }
	}
}

impl<'a, 'b> Parser<'b, &'b str> for StringParser<'a> {
	fn parse(&self, input: &'b InputStream) -> ParserOutput<'b, &'b str> {
		if input.get_text().starts_with(self.text) {
			let consume_length = self.text.len();
			let matched_text = &input.get_text()[..consume_length];
			let next_input = input.consume(consume_length);
			ParserOutput::succ(matched_text, next_input)
		}
		else {
			let failed_input = InputStream::clone(input);
			ParserOutput::fail(failed_input)
		}
	}
}

#[test]
fn string_parser_test() {
	let input = InputStream::new("abcde");
	let parser = StringParser::new("abc");
	let result = parser.parse(&input);
	assert!(result.is_valid());
	assert_eq!(result.get_output().unwrap(), "abc");
	assert_eq!(result.get_remaining_input().get_text(), "de");
	assert_eq!(result.get_remaining_input().get_text_position().get_line_number(), 1);
	assert_eq!(result.get_remaining_input().get_text_position().get_column_number(), 4);

	let input2 = InputStream::new("abdce");
	let result2 = parser.parse(&input2);
	assert!(!result2.is_valid());
	assert_eq!(result2.get_remaining_input().get_text(), input2.get_text());
	assert_eq!(result2.get_remaining_input().get_text_position().get_line_number(), 1);
	assert_eq!(result2.get_remaining_input().get_text_position().get_column_number(), 1);
}