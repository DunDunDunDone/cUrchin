use error::{PreprocessorError, SyntaxError};
use lang_c::driver::{Config, Flavor};

pub mod error;

/// Preprocess C source code, expanding preprocessor macros such as `#include` and `#define`.
pub fn preprocess(source_code: &str) -> Result<String, PreprocessorError> {
    // TODO
    Ok(source_code.to_owned())
}

/// Parses preprocessed C code into an AST. See [preprocess] to run the preprocessor before calling this.
pub fn parse(preprocessed_code: String) -> Result<lang_c::driver::Parse, SyntaxError> {
    let config = Config {
        flavor: Flavor::StdC11,
        ..Default::default()
    };
    lang_c::driver::parse_preprocessed(&config, preprocessed_code).map_err(SyntaxError::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noop_parse() {
        let source = r##"
int main(int argc, char** argv) {
    return 0;
}
"##;
        parse(source.to_owned()).expect("valid no-op c code can be parsed");
    }
}
