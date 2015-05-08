use internal::{InputStream, ParseOutput};

pub trait Parser<'t, OutputType> {
	fn parse<T: InputStream<T>>(&self, &'t T) -> ParseOutput<T, OutputType>;
}
