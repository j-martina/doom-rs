//! # `doomfront`
//!
//! ## About
//!
//! Comprehensive suite of frontends for domain-specific languages written for
//! Doom's source ports.
//!
//! Within this documentation, the term "lump" is used as a catch-all term for
//! a filesystem entry of some kind, whether that be a real file, a WAD archive
//! entry, or some other compressed archive entry.
//!
//! [`chumsky`] is used to build parsers up from combinators.
//! [`rowan`], used by [rust-analyzer], provides the basis for syntax representation.
//! It is recommended that you read its [overview] to understand the conceptual
//! foundation for the structures emitted by `doomfront`.
//!
//! `doomfront` is explicitly designed to be easy to extend.
//! Both of the aforementioned crates get re-exported in service of this.
//!
//! [rust-analyzer]: https://rust-analyzer.github.io/
//! [overview]: https://github.com/rust-lang/rust-analyzer/blob/master/docs/dev/syntax.md

pub extern crate chumsky;
pub extern crate rowan;

pub mod cvarinfo;

pub mod comb;
pub mod ext;
pub mod help;
mod repr;

#[cfg(test)]
mod test;

pub use repr::*;

/// Combinator parsers compose [green trees] manually instead of using the provided
/// [builder]. This type alias makes it more convenient to write these parsers.
///
/// [green trees]: rowan::GreenNode
/// [builder]: rowan::GreenNodeBuilder
pub type ParseOut = rowan::NodeOrToken<rowan::GreenNode, rowan::GreenToken>;
pub type ParseError = chumsky::error::Simple<char>;

/// Trait adding to `rowan::Language` with useful extras.
pub trait LangExt: rowan::Language {
	const SYN_WHITESPACE: Self::Kind;
	type AstRoot: rowan::ast::AstNode<Language = Self>;
}

/// Trait for language syntaxes that support some kind of comment.
pub trait LangComment: rowan::Language {
	const SYN_COMMENT: Self::Kind;
}

/// The most basic implementors for [`rowan::ast::AstNode`] are newtypes
/// (single-element tuple structs) which map to a single syntax tag. Automatically
/// generating `AstNode` implementations for these is trivial.
#[macro_export]
macro_rules! simple_astnode {
	($lang:ty, $node:ty, $syn_kind:expr) => {
		impl AstNode for $node {
			type Language = $lang;

			fn can_cast(kind: <Self::Language as rowan::Language>::Kind) -> bool
			where
				Self: Sized,
			{
				kind == $syn_kind
			}

			fn cast(node: rowan::SyntaxNode<Self::Language>) -> Option<Self>
			where
				Self: Sized,
			{
				if node.kind() == $syn_kind {
					Some(Self(node))
				} else {
					None
				}
			}

			fn syntax(&self) -> &rowan::SyntaxNode<Self::Language> {
				&self.0
			}
		}
	};
}
