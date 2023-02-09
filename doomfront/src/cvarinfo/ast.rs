//! Abstract syntax tree nodes.

use rowan::{ast::AstNode, NodeOrToken, SyntaxNode, SyntaxToken};

use crate::simple_astnode;

use super::Syn;

/// Abstract syntax tree node representing a whole CVar definition.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct CVar(SyntaxNode<Syn>);

impl CVar {
	/// Everything preceding the storage type specifier.
	#[must_use]
	pub fn flags(&self) -> Flags {
		self.0
			.first_child()
			.unwrap()
			.children()
			.find_map(Flags::cast)
			.unwrap()
	}

	/// The storage type specifier follows the flags and scope specifier, and
	/// precededes the identifier.
	///
	/// Its kind will be one of the following:
	/// - [`Syn::TypeInt`]
	/// - [`Syn::TypeFloat`]
	/// - [`Syn::TypeBool`]
	/// - [`Syn::TypeColor`]
	/// - [`Syn::TypeString`]
	#[must_use]
	pub fn type_spec(&self) -> SyntaxToken<Syn> {
		self.0
			.children_with_tokens()
			.find_map(|n_or_t| {
				if matches!(
					n_or_t.kind(),
					Syn::TypeInt
						| Syn::TypeFloat | Syn::TypeBool
						| Syn::TypeString | Syn::TypeColor
				) {
					n_or_t.into_token()
				} else {
					None
				}
			})
			.unwrap()
	}

	/// The identifier given to this CVar, after the type specifier.
	#[must_use]
	pub fn name(&self) -> SyntaxToken<Syn> {
		self.0
			.children_with_tokens()
			.find_map(|n_or_t| {
				if let NodeOrToken::Token(t) = n_or_t {
					if let Syn::Ident = t.kind() {
						Some(t)
					} else {
						None
					}
				} else {
					None
				}
			})
			.unwrap()
	}

	#[must_use]
	pub fn default(&self) -> Option<Default> {
		self.0.children().find_map(Default::cast)
	}
}

simple_astnode!(Syn, CVar, Syn::Definition);

/// Abstract syntax tree node representing the scope specifier and qualifiers.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Flags(SyntaxNode<Syn>);

impl Flags {
	/// The kind of the returned token will be one of the following:
	/// - [`Syn::KwServer`]
	/// - [`Syn::KwUser`]
	/// - [`Syn::KwNoSave`]
	#[must_use]
	pub fn scope(&self) -> SyntaxToken<Syn> {
		self.0
			.children_with_tokens()
			.find_map(|n_or_t| {
				if matches!(n_or_t.kind(), Syn::KwServer | Syn::KwUser | Syn::KwNoSave) {
					n_or_t.into_token()
				} else {
					None
				}
			})
			.unwrap()
	}

	/// The kinds of the yielded tokens (if any) will each be one of the following:
	/// - [`Syn::KwNoArchive`]
	/// - [`Syn::KwCheat`]
	/// - [`Syn::KwLatch`]
	pub fn qualifiers(&self) -> impl Iterator<Item = SyntaxToken<Syn>> {
		self.0.children_with_tokens().filter_map(|n_or_t| {
			if matches!(
				n_or_t.kind(),
				Syn::KwNoArchive | Syn::KwCheat | Syn::KwLatch
			) {
				n_or_t.into_token()
			} else {
				None
			}
		})
	}
}

simple_astnode!(Syn, Flags, Syn::Flags);

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Default(SyntaxNode<Syn>);

impl Default {
	/// The kind of the returned token will be one of the following:
	/// [`Syn::LitFalse`]
	/// [`Syn::LitTrue`]
	/// [`Syn::LitInt`]
	/// [`Syn::LitFloat`]
	/// [`Syn::LitString`]
	#[must_use]
	pub fn literal(&self) -> SyntaxToken<Syn> {
		self.0.last_token().unwrap()
	}
}

simple_astnode!(Syn, Default, Syn::DefaultDef);
