#[derive(Clone, Copy)]
pub struct InputPosition {
	line_number: usize,
	col_number: usize
}

impl InputPosition {
	pub fn new(l: usize, c: usize) -> InputPosition {
		InputPosition { line_number: l, col_number: c }
	}

	pub fn get_line_number(&self) -> usize {
		self.line_number
	}

	pub fn get_column_number(&self) -> usize {
		self.col_number
	}
}

impl Default for InputPosition {
	fn default() -> InputPosition {
		InputPosition::new(1, 1)
	}
}

#[test]
fn position_test() {
	let pos: InputPosition = Default::default();
	assert_eq!(pos.get_line_number(), 1);
	assert_eq!(pos.get_column_number(), 1);

	let pos2 = InputPosition::new(3, 4);
	assert_eq!(pos2.get_line_number(), 3);
	assert_eq!(pos2.get_column_number(), 4);
}

