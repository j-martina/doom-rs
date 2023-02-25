//! Abstract syntax tree nodes.

mod expr;
mod lit;

use rowan::ast::AstNode;

use crate::simple_astnode;

use super::{syn::Syn, SyntaxNode, SyntaxToken};

pub use self::{expr::*, lit::*};

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

/// Wraps a node tagged [`Syn::Name`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Name(SyntaxNode);

simple_astnode!(Syn, Name, Syn::Name);

impl Name {
	#[must_use]
	pub fn ident(&self) -> SyntaxToken {
		self.0.first_token().unwrap()
	}
}

/// Wraps a node tagged [`Syn::EnumDef`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct EnumDef(SyntaxNode);

simple_astnode!(Syn, EnumDef, Syn::EnumDef);

impl EnumDef {
	pub fn variants(&self) -> impl Iterator<Item = EnumVariant> {
		self.0.children().filter_map(EnumVariant::cast)
	}
}

/// Wraps a node tagged [`Syn::EnumVariant`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct EnumVariant(SyntaxNode);

simple_astnode!(Syn, EnumVariant, Syn::EnumVariant);

impl EnumVariant {
	#[must_use]
	pub fn name(&self) -> Name {
		Name::cast(self.0.first_child().unwrap()).unwrap()
	}

	#[must_use]
	pub fn expr(&self) -> Option<Expression> {
		self.0
			.last_child()
			.map(|node| Expression::cast(node).unwrap())
	}
}
