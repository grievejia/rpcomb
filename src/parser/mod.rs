pub use self::string_parser::strp;
pub use self::regex_parser::regexp;
pub use self::parser::Parser;

pub mod parser;
mod string_parser;
mod regex_parser;