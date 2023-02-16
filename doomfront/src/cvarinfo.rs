//! Parser and syntax trees for the [`CVARINFO`](https://zdoom.org/wiki/CVARINFO)
//! lump defined by ZDoom-family source ports.
//!
//! Console variables or "CVars" are ZDoom's way of storing user preferences
//! and the de facto solution for persistent storage.

pub mod ast;
mod parse;
mod syn;
#[cfg(test)]
mod test;

pub use parse::*;
pub use syn::*;

pub type ParseTree = crate::repr::ParseTree<Syn>;
pub type IncludeTree = crate::repr::IncludeTree<Syn>;
pub type RawParseTree = crate::repr::RawParseTree<Syn>;
pub type SyntaxNode = rowan::SyntaxNode<Syn>;
pub type SyntaxToken = rowan::SyntaxToken<Syn>;
pub type Token = rowan::SyntaxToken<Syn>;
