use internal::{InputStream, ParseOutput};

pub trait Parser<'t> {
	type OutputType;

	fn parse<T: InputStream<'t, T>>(&self, T) -> ParseOutput<T, Self::OutputType>;
}
