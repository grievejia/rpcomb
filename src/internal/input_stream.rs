use super::input_position::InputPosition;

pub trait InputStream<'t, StreamType: InputStream<'t, StreamType>> {
	fn get_input(&self) -> &'t str;
	fn get_position(&self) -> InputPosition;
	fn consume(&self, n: usize) -> StreamType;
}
