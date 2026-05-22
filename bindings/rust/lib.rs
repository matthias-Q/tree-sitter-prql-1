//! This crate provides PRQL language support for the [tree-sitter][] parsing library.
//!
//! Typically, you will use the [LANGUAGE][] constant to add this language to a
//! tree-sitter [Parser][], and then use the parser to parse some code:
//!
//! ```
//! let code = r#"
//! from employees
//! filter age > 25
//! select {first_name, last_name}
//! "#;
//! let mut parser = tree_sitter::Parser::new();
//! let language = tree_sitter_prql::LANGUAGE;
//! parser
//!     .set_language(&language.into())
//!     .expect("Error loading PRQL parser");
//! let tree = parser.parse(code, None).unwrap();
//! assert!(!tree.root_node().has_error());
//! ```
//!
//! [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
//! [Parser]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Parser.html
//! [tree-sitter]: https://tree-sitter.github.io/

use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_prql() -> *const ();
}

/// The tree-sitter [`LanguageFn`] for this grammar.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_prql) };

/// The content of the [`node-types.json`][] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES: &str = include_str!("../../src/node-types.json");

/// The syntax highlighting query for this language.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../queries/highlights.scm");

/// The injection query for this language.
pub const INJECTIONS_QUERY: &str = include_str!("../../queries/injections.scm");

/// The symbol tagging query for this language.
/// This can be used for code navigation systems.
pub const LOCALS_QUERY: &str = "";

/// The tag query for this language.
pub const TAGS_QUERY: &str = "";

/// Re-export the Language type for convenience
pub use tree_sitter::Language;

/// Get the tree-sitter Language.
///
/// # Deprecated
/// Use [`LANGUAGE`] instead.
#[deprecated(since = "1.0.0", note = "use `LANGUAGE` instead")]
pub fn language() -> Language {
    LANGUAGE.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&LANGUAGE.into())
            .expect("Error loading PRQL parser");
    }

    #[test]
    fn test_simple_query() {
        let code = "from employees\nfilter age > 25";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&LANGUAGE.into())
            .expect("Error loading PRQL parser");
        let tree = parser.parse(code, None).unwrap();
        let root = tree.root_node();
        assert!(!root.has_error());
        assert_eq!(root.kind(), "program");
    }
}
