//! Structures for representing source code at various levels of abstraction.

use std::marker::PhantomData;

use rowan::{ast::AstNode, GreenNode, SyntaxNode};

use crate::{LangExt, ParseError};

/// The (G)ZDoom MAPINFO, DECORATE, and ZScript file formats all support the use
/// of C-like `#include` directives to build a tree of files from a root.
#[derive(Debug)]
#[cfg_attr(feature = "ser_de", derive(serde::Serialize))]
pub struct IncludeTree<L: LangExt> {
	pub files: Vec<ParseTree<L>>,
}

/// Represents a source string. It may not necessarily represent valid code; it
/// contains no semantic information and parsers recover upon encountering errors.
#[derive(Debug)]
#[cfg_attr(feature = "ser_de", derive(serde::Serialize))]
pub struct ParseTree<L: LangExt> {
	#[cfg_attr(feature = "ser_de", serde(skip))]
	raw: RawParseTree<L>,
	zipper: SyntaxNode<L>,
}

impl<L: LangExt> ParseTree<L> {
	#[must_use]
	pub fn new(raw: RawParseTree<L>) -> Self {
		let zipper = SyntaxNode::new_root(raw.root.clone());

		Self { raw, zipper }
	}

	/// The "zipper tree" (or "cursor tree") is a more convenient way of reading
	/// the "raw" green tree accessible via [`Self::raw`], as it contains
	/// parent pointers and identity information.
	///
	/// [`Self::raw`]: RawParseTree::raw
	#[must_use]
	pub fn zipper(&self) -> &SyntaxNode<L> {
		&self.zipper
	}

	/// Returns an iterator over the "top-level" abstract syntax tree nodes.
	///
	/// The highest-level syntactic representation available. Abstract syntax
	/// tree nodes are thin wrappers over [`SyntaxNode`]s that expose methods
	/// allowing language-specific introspection into source details. This is the
	/// most useful part of a `ParseTree` for implementing a semantic checker.
	///
	/// It is important to note that while a language's grammar may mandate
	/// certain syntax elements in certain places, parsers which tolerate or can
	/// recover from errors will necessarily produce broken syntax trees, and so
	/// AST interfaces may return `Option` unintuitively.
	pub fn ast(&self) -> impl Iterator<Item = L::AstRoot> {
		self.zipper().children().filter_map(L::AstRoot::cast)
	}
}

impl<L: LangExt> std::ops::Deref for ParseTree<L> {
	type Target = RawParseTree<L>;

	fn deref(&self) -> &Self::Target {
		&self.raw
	}
}

/// This is a counterpart to [`ParseTree`] which has not pre-constructed a zipper
/// node, and is therefore [`Sync`]. Parsing functions emit these to allow
/// multithreaded parsing if so desired. To convert to a more useful form with the
/// possibility for inspecting an abstract syntax tree, use [`ParseTree::new`].
#[derive(Debug)]
pub struct RawParseTree<L: LangExt> {
	root: GreenNode,
	errors: Vec<ParseError>,
	phantom: PhantomData<L>,
}

impl<L: LangExt> RawParseTree<L> {
	#[must_use]
	pub fn new(root: GreenNode, errors: Vec<ParseError>) -> Self {
		Self {
			root,
			errors,
			phantom: PhantomData,
		}
	}

	/// The "source of truth" for the parsed code.
	#[must_use]
	pub fn raw(&self) -> &GreenNode {
		&self.root
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

	#[must_use]
	pub fn into_errors(self) -> Vec<ParseError> {
		self.errors
	}
}
