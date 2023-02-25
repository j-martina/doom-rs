//! AST nodes for representing literals.

use std::num::ParseIntError;

use rowan::ast::AstNode;

use crate::{
	decorate::{Syn, SyntaxNode, SyntaxToken},
	simple_astnode,
};

/// Wraps a node tagged [`Syn::Literal`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Literal(pub(super) SyntaxNode);

simple_astnode!(Syn, Literal, Syn::Literal);

/// Wrapper around a [`SyntaxToken`] with convenience functions.
/// See [`Syn::Literal`]'s documentation to see possible token tags.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct LitToken(SyntaxToken);

impl LitToken {
	/// If this wraps a [`Syn::LitTrue`] or [`Syn::LitFalse`] token,
	/// this returns the corresponding value. Otherwise this returns `None`.
	#[must_use]
	pub fn bool(&self) -> Option<bool> {
		match self.0.kind() {
			Syn::LitTrue => Some(true),
			Syn::LitFalse => Some(false),
			_ => None,
		}
	}

	#[must_use]
	pub fn float(&self) -> Option<f64> {
		if !matches!(self.0.kind(), Syn::LitFloat) {
			return None;
		}

		let text = self.0.text();

		let end = text.len()
			- text
				.chars()
				.rev()
				.position(|c| !c.eq_ignore_ascii_case(&'f'))
				.unwrap();

		text[..end].parse::<f64>().ok()
	}

	/// Returns `None` if this is not tagged with [`Syn::LitInt`].
	/// Returns `Some(Err)` if integer parsing fails,
	/// such as if the written value is too large to fit into a `u64`.
	#[must_use]
	pub fn int(&self) -> Option<Result<u64, ParseIntError>> {
		if !matches!(self.0.kind(), Syn::LitInt) {
			return None;
		}

		let text = self.0.text();

		let radix = if text.len() > 2 {
			match &text[0..2] {
				"0x" => 16,
				_ => 10,
			}
		} else {
			10
		};

		// Identify the span between the prefix and suffix.
		let start = if radix != 10 { 2 } else { 0 };

		let end = text.len()
			- text
				.chars()
				.rev()
				.position(|c| !(c.eq_ignore_ascii_case(&'u') || c.eq_ignore_ascii_case(&'l')))
				.unwrap();

		Some(u64::from_str_radix(&text[start..end], radix))
	}

	/// If this wraps a [`Syn::LitName`] token, this returns the string's
	/// content with the delimiting single-quotation marks stripped away.
	/// Otherwise this returns `None`.
	#[must_use]
	pub fn name(&self) -> Option<&str> {
		if self.0.kind() == Syn::LitString {
			let text = self.0.text();
			let start = text.chars().position(|c| c == '\'').unwrap();
			let end = text.chars().rev().position(|c| c == '\'').unwrap();
			text.get((start + 1)..(text.len() - end - 1))
		} else {
			None
		}
	}

	/// If this wraps a [`Syn::LitString`] token, this returns the string's
	/// content with the delimiting double-quotation marks stripped away.
	/// Otherwise this returns `None`.
	#[must_use]
	pub fn string(&self) -> Option<&str> {
		if self.0.kind() == Syn::LitString {
			let text = self.0.text();
			let start = text.chars().position(|c| c == '"').unwrap();
			let end = text.chars().rev().position(|c| c == '"').unwrap();
			text.get((start + 1)..(text.len() - end - 1))
		} else {
			None
		}
	}

	#[must_use]
	pub fn syntax(&self) -> &SyntaxToken {
		&self.0
	}
}
