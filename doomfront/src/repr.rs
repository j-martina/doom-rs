//! Structures for representing source code at various levels of abstraction.

use rowan::{ast::AstNode, GreenNode, SyntaxNode};

use crate::{LangExt, ParseError};

/// The (G)ZDoom MAPINFO, DECORATE, and ZScript file formats all support the use
/// of C-like `#include` directives to build a tree of files from a root.
#[derive(Debug)]
pub struct IncludeTree<L: LangExt> {
	pub files: Vec<ParseTree<L>>,
}

/// Represents a source string. It may not necessarily represent valid code; it
/// contains no semantic information and parsers recover upon encountering errors.
#[derive(Debug)]
pub struct ParseTree<L: LangExt> {
	green: GreenNode,
	zipper: SyntaxNode<L>,
	errors: Vec<ParseError>,
}

impl<L: LangExt> ParseTree<L> {
	#[must_use]
	pub fn new(root: GreenNode, errors: Vec<ParseError>) -> Self {
		let zipper = SyntaxNode::new_root(root.clone());

		Self {
			green: root,
			zipper,
			errors,
		}
	}

	/// The "zipper tree" (or "cursor tree") is a more convenient way of reading
	/// the "raw" green tree accessible via [`Self::raw`]. This tree contains
	/// parent pointers and identity information
	#[must_use]
	pub fn zipper(&self) -> &SyntaxNode<L> {
		&self.zipper
	}

	/// The "source of truth" for the parsed code.
	#[must_use]
	pub fn raw(&self) -> &GreenNode {
		&self.green
	}

	/// Returns an iterator over the "top-level" abstract syntax tree nodes.
	///
	/// The highest-level syntactic representation available. Abstract syntax
	/// tree nodes are thin wrappers over [`SyntaxNode`]s that expose methods
	/// allowing language-specific introspection into source details. This is the
	/// most useful part of a `ParseTree` for implementing a semantic checker.
	pub fn ast(&self) -> impl Iterator<Item = L::AstRoot> {
		self.zipper().children().filter_map(L::AstRoot::cast)
	}

	/// Were any errors encountered when parsing a token stream?
	#[must_use]
	pub fn any_errors(&self) -> bool {
		!self.errors.is_empty()
	}

	/// Errors encountered while parsing a token stream.
	#[must_use]
	pub fn errors(&self) -> &[ParseError] {
		&self.errors
	}
}
