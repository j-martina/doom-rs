//! Syntax tags.

use crate::{LangComment, LangExt};

use super::ast::CVar;

/// CVARINFO syntax nodes, from low-level primitives to high-level composites.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "ser_de", derive(serde::Serialize))]
#[repr(u16)]
pub enum Syn {
	/// A whole CVar definition.
	Definition,
	/// An `=` followed by a literal to optionally set a custom default value.
	DefaultDef,
	/// The set of flags qualifying a definition, scope specifiers included.
	Flags,
	/// The type specifier is always followed by the identifier.
	TypeSpec,

	/// The boolean literal `false`.
	LitFalse,
	/// A (G)ZDoom (i.e. C/C++-style) floating-point literal.
	LitFloat,
	/// A (G)ZDoom (i.e. C/C++-style) integer literal.
	LitInt,
	/// Delimited by double quotes. Also used for defining default color values.
	LitString,
	/// The boolean literal `true`.
	LitTrue,

	/// The type specifier `bool`.
	TypeBool,
	/// The type specifier `color`.
	TypeColor,
	/// The type specifier `float`.
	TypeFloat,
	/// The type specifier `int`.
	TypeInt,
	/// The type specifier `string`.
	TypeString,

	/// The configuration flag `cheat`.
	KwCheat,
	/// The configuration flag `noarchive`.
	KwNoArchive,
	/// The scope specifier `nosave`.
	KwNoSave,
	/// The configuration flag `latch`.
	KwLatch,
	/// The scope specifier `server`.
	KwServer,
	/// The scope specifier `user`.
	KwUser,

	/// The `=` character.
	Eq,
	/// A name for a defined CVar.
	Ident,
	/// The `;` character, used as a terminator.
	Semicolon,

	/// CVARINFO comments use C++ syntax and are treated like whitespace.
	Comment,
	/// Input that the lexer considered to be invalid.
	Unknown,
	/// Spaces, carriage returns, newlines, and tabs are ignored by CVARINFO.
	Whitespace,
	/// The top-level node, representing the whole file.
	Root, // Ensure this is always the last variant!
}

impl From<Syn> for rowan::SyntaxKind {
	fn from(value: Syn) -> Self {
		Self(value as u16)
	}
}

impl rowan::Language for Syn {
	type Kind = Self;

	fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
		assert!(raw.0 <= Self::Root as u16);
		unsafe { std::mem::transmute::<u16, Syn>(raw.0) }
	}

	fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
		kind.into()
	}
}

impl LangExt for Syn {
	const SYN_WHITESPACE: Self::Kind = Self::Whitespace;
	type AstRoot = CVar;
}

impl LangComment for Syn {
	const SYN_COMMENT: Self::Kind = Self::Comment;
}
