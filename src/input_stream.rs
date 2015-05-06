#[derive(Clone, Copy)]
pub struct TextPosition {
	line_number: usize,
	col_number: usize
}

impl TextPosition {
	pub fn new(l: usize, c: usize) -> TextPosition {
		TextPosition { line_number: l, col_number: c }
	}

	pub fn get_line_number(&self) -> usize {
		self.line_number
	}

	pub fn get_column_number(&self) -> usize {
		self.col_number
	}
}

impl Default for TextPosition {
	fn default() -> TextPosition {
		TextPosition::new(1, 1)
	}
}

#[derive(Clone, Copy)]
pub struct InputStream<'a> {
	text: &'a str,
	pos: TextPosition
}

impl<'a> InputStream<'a> {
	pub fn construct(t: &str, p: TextPosition) -> InputStream {
		InputStream { text: t, pos: p }
	}

	pub fn new(t: &str) -> InputStream {
		InputStream::construct(t, Default::default())
	}

	pub fn consume(&self, n: usize) -> InputStream {
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

		InputStream::construct(remaining_text, TextPosition::new(new_line, new_col))
	}

	pub fn get_text(&self) -> &'a str {
		self.text
	}

	pub fn get_text_position(&'a self) -> &'a TextPosition {
		&self.pos
	}

	pub fn is_empty(&self) -> bool {
		self.text.is_empty()
	}
}

#[test]
fn position_test() {
	let pos: TextPosition = Default::default();
	assert_eq!(pos.get_line_number(), 1);
	assert_eq!(pos.get_column_number(), 1);

	let pos2 = TextPosition::new(3, 4);
	assert_eq!(pos2.get_line_number(), 3);
	assert_eq!(pos2.get_column_number(), 4);
}

#[test]
fn input_stream_test() {
	let text = "abcd\nefg";
	let istream = InputStream::new(text);

	assert_eq!(istream.get_text(), text);
	assert_eq!(istream.get_text_position().get_line_number(), 1);
	assert_eq!(istream.get_text_position().get_column_number(), 1);
	assert!(!istream.is_empty());

	let istream2 = istream.consume(3);
	assert_eq!(istream2.get_text(), "d\nefg");
	assert_eq!(istream2.get_text_position().get_line_number(), 1);
	assert_eq!(istream2.get_text_position().get_column_number(), 4);
	assert!(!istream2.is_empty());

	let istream3 = istream2.consume(3);
	assert_eq!(istream3.get_text(), "fg");
	assert_eq!(istream3.get_text_position().get_line_number(), 2);
	assert_eq!(istream3.get_text_position().get_column_number(), 2);
	assert!(!istream3.is_empty());

	let istream4 = istream3.consume(2);
	assert_eq!(istream4.get_text(), "");
	assert_eq!(istream4.get_text_position().get_line_number(), 2);
	assert_eq!(istream4.get_text_position().get_column_number(), 4);
	assert!(istream4.is_empty());
}

#[test]
#[should_panic]
fn input_stream_excess_consume() {
	let istream = InputStream::new("abcde");
	istream.consume(6);
}