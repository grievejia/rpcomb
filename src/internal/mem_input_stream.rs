use super::input_position::InputPosition;
use super::input_stream::InputStream;

#[derive(Copy, Clone)]
pub struct MemoryInputStream<MemType> {
	bytes: MemType,
	pos: InputPosition
}

impl<MemType> MemoryInputStream<MemType> {
	fn create(t: MemType, p: InputPosition) -> MemoryInputStream<MemType> {
		MemoryInputStream { bytes: t, pos : p}
	}

	pub fn new(t: MemType) -> MemoryInputStream<MemType> {
		MemoryInputStream::create(t, Default::default())
	}
}

pub type ByteInputStream<'a> = MemoryInputStream<&'a [u8]>;
pub type StringInputStream<'a> = MemoryInputStream<&'a str>;

impl<'a> InputStream<ByteInputStream<'a>> for ByteInputStream<'a> {
	fn get_input(&self) -> &[u8] {
		self.bytes
	}

	fn get_position(&self) -> InputPosition {
		self.pos
	}

	fn consume(&self, n: usize) -> ByteInputStream<'a> {
		assert!(n <= self.bytes.len());
		
		let new_line = self.pos.get_line_number();
		let new_col = self.pos.get_column_number() + n;
		let remaining_text = &self.bytes[n..];

		ByteInputStream::create(remaining_text, InputPosition::new(new_line, new_col))
	}
}

impl<'a> InputStream<StringInputStream<'a>> for StringInputStream<'a> {
	fn get_input(&self) -> &[u8] {
		self.bytes.as_bytes()
	}

	fn get_position(&self) -> InputPosition {
		self.pos
	}

	fn consume(&self, n: usize) -> StringInputStream<'a> {
		assert!(n <= self.bytes.len());
		
		let mut new_line = self.pos.get_line_number();
		let mut new_col = self.pos.get_column_number();
		let eaten_text = &self.bytes[..n];
		let remaining_text = &self.bytes[n..];

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
fn byte_input_stream_test() {
	let text = "abcd\nefg";
	let istream = ByteInputStream::new(text.as_bytes());

	assert_eq!(istream.get_input(), text.as_bytes());
	assert_eq!(istream.get_position().get_line_number(), 1);
	assert_eq!(istream.get_position().get_column_number(), 1);

	let istream2 = istream.consume(3);
	assert_eq!(istream2.get_input(), "d\nefg".as_bytes());
	assert_eq!(istream2.get_position().get_line_number(), 1);
	assert_eq!(istream2.get_position().get_column_number(), 4);

	let istream3 = istream2.consume(3);
	assert_eq!(istream3.get_input(), "fg".as_bytes());
	assert_eq!(istream3.get_position().get_line_number(), 1);
	assert_eq!(istream3.get_position().get_column_number(), 7);

	let istream4 = istream3.consume(2);
	assert_eq!(istream4.get_input(), "".as_bytes());
	assert_eq!(istream4.get_position().get_line_number(), 1);
	assert_eq!(istream4.get_position().get_column_number(), 9);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn byte_input_stream_excess_consume() {
	let istream = ByteInputStream::new("abcde".as_bytes());
	istream.consume(6);
}

#[test]
fn string_input_stream_test() {
	let text = "abcd\nefg";
	let istream = StringInputStream::new(text);

	assert_eq!(istream.get_input(), text.as_bytes());
	assert_eq!(istream.get_position().get_line_number(), 1);
	assert_eq!(istream.get_position().get_column_number(), 1);

	let istream2 = istream.consume(3);
	assert_eq!(istream2.get_input(), "d\nefg".as_bytes());
	assert_eq!(istream2.get_position().get_line_number(), 1);
	assert_eq!(istream2.get_position().get_column_number(), 4);

	let istream3 = istream2.consume(3);
	assert_eq!(istream3.get_input(), "fg".as_bytes());
	assert_eq!(istream3.get_position().get_line_number(), 2);
	assert_eq!(istream3.get_position().get_column_number(), 2);

	let istream4 = istream3.consume(2);
	assert_eq!(istream4.get_input(), "".as_bytes());
	assert_eq!(istream4.get_position().get_line_number(), 2);
	assert_eq!(istream4.get_position().get_column_number(), 4);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn string_input_stream_excess_consume() {
	let istream = StringInputStream::new("abcde");
	istream.consume(6);
}