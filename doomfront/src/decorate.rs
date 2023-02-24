//! Parser and syntax trees for the [`DECORATE`](https://zdoom.org/wiki/DECORATE)
//! language defined by (G)ZDoom.
//!
//! DECORATE is a data definition language and pseudo-scripting language for
//! creating new game content.

pub mod ast;
mod syn;

pub use syn::*;

pub type ParseTree = crate::repr::ParseTree<Syn>;
pub type IncludeTree = crate::repr::IncludeTree<Syn>;
pub type RawParseTree = crate::repr::RawParseTree<Syn>;
pub type SyntaxNode = rowan::SyntaxNode<Syn>;
pub type SyntaxToken = rowan::SyntaxToken<Syn>;
pub type Token = rowan::SyntaxToken<Syn>;
