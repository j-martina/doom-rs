//! Syntax tags.

use crate::{LangComment, LangExt};

use super::ast;

/// DECORATE syntax nodes, from low-level primitives to high-level composites.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "ser_de", derive(serde::Serialize))]
#[repr(u16)]
pub enum Syn {
	// High-level composites ///////////////////////////////////////////////////
	/// `enum {};`
	EnumDef,
	/// `NAME = expr`
	EnumVariant,
	/// e.g. `expr + expr`
	ExprBinary,
	/// `expr()`
	ExprCall,
	/// `expr[expr]`; array element access.
	ExprIndex,
	/// e.g. `expr++` or `expr?`
	ExprPostfix,
	/// e.g. `++expr`
	ExprPrefix,
	/// `expr = expr ? expr : expr`
	ExprTernary,
	/// Syntax node with just a [`Syn::Ident`] token as a child.
	/// Used as part of function declarations, variable bindings, et cetera.
	/// Not to be confused with the [name literal](Syn::LitName).
	Name,
	/// `ident:`. Used in actor state definition blocks.
	/// Distinct from [`Syn::Name`] since it does not introduce a symbol into a scope.
	Label,
	Literal,

	// Soon!

	// Keywords ////////////////////////////////////////////////////////////////
	KwAction,
	KwActor,
	KwBreak,
	KwConst,
	KwContinue,
	KwDo,
	KwElse,
	KwEnum,
	KwFail,
	KwFor,
	KwIf,
	KwGoto,
	KwNative,
	KwReplaces,
	KwReturn,
	KwStates,
	KwStop,
	KwSuper,
	KwVar,
	KwWhile,

	// Literals ////////////////////////////////////////////////////////////////
	/// The exact string `false`.
	LitFalse,
	LitFloat,
	LitInt,
	/// A string delimited by single-quotes (`'`).
	LitName,
	LitString,
	/// The exact string `true`.
	LitTrue,

	// Glyphs, composite glyphs, glyph-adjacent ////////////////////////////////
	/// `&`
	Ampersand,
	/// `&&`
	Ampersand2,
	/// `<`
	AngleL,
	/// `<<`
	AngleL2,
	/// `<<=`
	AngleL2Eq,
	/// `<=`
	AngleLEq,
	/// `>`
	AngleR,
	/// `>>`
	AngleR2,
	/// `>>=`
	AngleR2Eq,
	/// `>>>`
	AngleR3,
	/// `>>>=`
	AngleR3Eq,
	/// `>=`
	AngleREq,
	/// `*`
	Asterisk,
	/// `*=`
	AsteriskEq,
	/// `!`
	Bang,
	/// `!=`
	BangEq,
	/// `{`
	BraceL,
	/// `}`
	BraceR,
	/// `[`
	BracketL,
	/// `]`
	BracketR,
	/// `^`
	Caret,
	/// `^=`
	CaretEq,
	/// `:`
	Colon,
	/// `::`
	Colon2,
	/// `,`
	Comma,
	/// `=`
	Eq,
	/// `==`
	Eq2,
	/// `~` a.k.a tilde.
	Grave,
	/// `-`
	Minus,
	/// `-=`
	MinusEq,
	/// `--`
	Minus2,
	/// `(`
	ParenL,
	/// `)`
	ParenR,
	/// `.`
	Period,
	/// `|`
	Pipe,
	/// `|=`
	PipeEq,
	/// `%=`
	PercentEq,
	/// `||`
	Pipe2,
	/// `+`
	Plus,
	/// `++`
	Plus2,
	/// `+=`
	PlusEq,
	/// `?`
	Question,
	/// `;`
	Semicolon,
	/// `/`
	Slash,
	/// `/=`
	SlashEq,

	// Miscellaneous ///////////////////////////////////////////////////////////
	/// DECORATE comments use C++ syntax and are treated like whitespace.
	Comment,
	/// C-style; an ASCII letter or underscore, then any number of ASCII letters,
	/// ASCII digits, or underscores. Assigned only to tokens.
	/// Can be used in [`Syn::Name`] or [`Syn::Label`] nodes.
	Ident,
	/// Input that the lexer considered to be invalid.
	Unknown,
	/// Spaces, carriage returns, newlines, and tabs are ignored by DECORATE.
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
	type AstRoot = ast::Root;
}

impl LangComment for Syn {
	const SYN_COMMENT: Self::Kind = Self::Comment;
}

impl Syn {
	/// Alternatively "is whitespace or comment".
	#[must_use]
	pub fn is_trivia(&self) -> bool {
		matches!(self, Syn::Comment | Syn::Whitespace)
	}
}
