pub use self::byte_parser::bytep;
pub use self::string_parser::strp;
pub use self::parser::Parser;

pub mod parser;
mod byte_parser;
mod string_parser;