use super::input_position::InputPosition;

pub trait InputStream<StreamType: InputStream<StreamType>> {
	fn get_input(&self) -> &[u8];
	fn get_position(&self) -> InputPosition;
	fn consume(&self, n: usize) -> StreamType;
}
