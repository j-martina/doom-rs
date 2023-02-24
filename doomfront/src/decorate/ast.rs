//! Abstract syntax tree nodes.

use rowan::ast::AstNode;

use super::{syn::Syn, SyntaxNode};

/// One of the top-level elements of a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Root {
	// ???
}

impl AstNode for Root {
	type Language = Syn;

	fn can_cast(_kind: <Self::Language as rowan::Language>::Kind) -> bool
	where
		Self: Sized,
	{
		unimplemented!()
	}

	fn cast(_node: rowan::SyntaxNode<Self::Language>) -> Option<Self>
	where
		Self: Sized,
	{
		unimplemented!()
	}

	fn syntax(&self) -> &SyntaxNode {
		unimplemented!()
	}
}
