use super::input_position::InputPosition;
use super::input_stream::InputStream;

#[derive(Copy, Clone)]
pub struct StringInputStream<'a> {
	text: &'a str,
	pos: InputPosition
}

impl<'a> StringInputStream<'a> {
	fn create(t: &'a str, p: InputPosition) -> StringInputStream<'a> {
		StringInputStream { text: t, pos : p}
	}

	pub fn new(t: &'a str) -> StringInputStream<'a> {
		StringInputStream::create(t, Default::default())
	}
}

impl<'a> InputStream<'a, StringInputStream<'a>> for StringInputStream<'a> {
	fn get_input(&self) -> &'a str {
		self.text
	}

	fn get_position(&self) -> InputPosition {
		self.pos
	}

	fn consume(&self, n: usize) -> StringInputStream<'a> {
		assert!(n <= self.text.len());
		
		let mut new_line = self.pos.get_line_number();
		let mut new_col = self.pos.get_column_number();
		let eaten_text = &self.text[..n];
		let remaining_text = &self.text[n..];

		for ch in eaten_text.chars() {
			if ch == '\n' {
				new_line += 1;
				new_col = 1;
			}
			else {
				new_col += 1
			}
		}

		StringInputStream::create(remaining_text, InputPosition::new(new_line, new_col))
	}
}

#[test]
fn string_input_stream_test() {
	let text = "abcd\nefg";
	let istream = StringInputStream::new(text);

	assert_eq!(istream.get_input(), text);
	assert_eq!(istream.get_position().get_line_number(), 1);
	assert_eq!(istream.get_position().get_column_number(), 1);

	let istream2 = istream.consume(3);
	assert_eq!(istream2.get_input(), "d\nefg");
	assert_eq!(istream2.get_position().get_line_number(), 1);
	assert_eq!(istream2.get_position().get_column_number(), 4);

	let istream3 = istream2.consume(3);
	assert_eq!(istream3.get_input(), "fg");
	assert_eq!(istream3.get_position().get_line_number(), 2);
	assert_eq!(istream3.get_position().get_column_number(), 2);

	let istream4 = istream3.consume(2);
	assert_eq!(istream4.get_input(), "");
	assert_eq!(istream4.get_position().get_line_number(), 2);
	assert_eq!(istream4.get_position().get_column_number(), 4);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn string_input_stream_excess_consume() {
	let istream = StringInputStream::new("abcde");
	istream.consume(6);
}