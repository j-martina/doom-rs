//! AST nodes for representing expressions.

use rowan::ast::AstNode;

use crate::{
	decorate::{Syn, SyntaxNode, SyntaxToken},
	simple_astnode,
};

use super::{Literal, Name};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
	Binary(ExprBinary),
	Call(ExprCall),
	Index(ExprIndex),
	Literal(Literal),
	Name(Name),
	Prefix(ExprPrefix),
	Postfix(ExprPostfix),
	Ternary(ExprTernary),
}

impl AstNode for Expression {
	type Language = Syn;

	fn can_cast(kind: <Self::Language as rowan::Language>::Kind) -> bool
	where
		Self: Sized,
	{
		matches!(
			kind,
			Syn::ExprBinary
				| Syn::ExprCall | Syn::ExprIndex
				| Syn::Name | Syn::Literal
				| Syn::ExprPostfix
				| Syn::ExprPrefix
				| Syn::ExprTernary
		)
	}

	fn cast(node: SyntaxNode) -> Option<Self>
	where
		Self: Sized,
	{
		match node.kind() {
			Syn::ExprBinary => Some(Self::Binary(ExprBinary(node))),
			Syn::ExprCall => Some(Self::Call(ExprCall(node))),
			Syn::ExprIndex => Some(Self::Index(ExprIndex(node))),
			Syn::Literal => Some(Self::Literal(Literal(node))),
			Syn::Name => Some(Self::Name(Name(node))),
			Syn::ExprPostfix => Some(Self::Postfix(ExprPostfix(node))),
			Syn::ExprPrefix => Some(Self::Prefix(ExprPrefix(node))),
			Syn::ExprTernary => Some(Self::Ternary(ExprTernary(node))),
			_ => None,
		}
	}

	fn syntax(&self) -> &SyntaxNode {
		match self {
			Self::Binary(inner) => &inner.0,
			Self::Call(inner) => &inner.0,
			Self::Index(inner) => &inner.0,
			Self::Literal(inner) => &inner.0,
			Self::Name(inner) => &inner.0,
			Self::Prefix(inner) => &inner.0,
			Self::Postfix(inner) => &inner.0,
			Self::Ternary(inner) => &inner.0,
		}
	}
}

impl Expression {
	#[must_use]
	pub fn into_literal(self) -> Option<Literal> {
		match self {
			Self::Literal(lit) => Some(lit),
			_ => None,
		}
	}
}

/// Wraps a node tagged [`Syn::ExprBinary`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct ExprBinary(SyntaxNode);

simple_astnode!(Syn, ExprBinary, Syn::ExprBinary);

impl ExprBinary {
	#[must_use]
	pub fn left(&self) -> Expression {
		Expression::cast(self.0.first_child().unwrap()).unwrap()
	}

	#[must_use]
	pub fn operator(&self) -> SyntaxToken {
		self.0
			.children_with_tokens()
			.find_map(|n_or_t| {
				if n_or_t.kind().is_trivia() {
					None
				} else {
					n_or_t.into_token()
				}
			})
			.unwrap()
	}

	#[must_use]
	pub fn right(&self) -> Expression {
		Expression::cast(self.0.last_child().unwrap()).unwrap()
	}
}

/// Wraps a node tagged [`Syn::ExprCall`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct ExprCall(SyntaxNode);

simple_astnode!(Syn, ExprCall, Syn::ExprCall);

/// Wraps a node tagged [`Syn::ExprIndex`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct ExprIndex(SyntaxNode);

simple_astnode!(Syn, ExprIndex, Syn::ExprIndex);

/// Wraps a node tagged [`Syn::ExprPostfix`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct ExprPostfix(SyntaxNode);

simple_astnode!(Syn, ExprPostfix, Syn::ExprPostfix);

/// Wraps a node tagged [`Syn::ExprPrefix`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct ExprPrefix(SyntaxNode);

simple_astnode!(Syn, ExprPrefix, Syn::ExprPrefix);

/// Wraps a node tagged [`Syn::ExprTernary`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct ExprTernary(SyntaxNode);

simple_astnode!(Syn, ExprTernary, Syn::ExprTernary);
