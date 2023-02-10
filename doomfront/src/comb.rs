//! Assorted combinators which are broadly applicable elsewhere.
//!
//! These take the whole original source string as argument so that a slice of
//! it can be taken via a parsed span and passed to [`GreenToken::new`], allowing
//! `rowan` tree construction with a bare minimum of allocations.

// Q: Does performance improve if combinators don't have to capture the source?

use chumsky::{primitive, text, Error, Parser};
use rowan::{GreenToken, SyntaxKind};

use crate::{LangComment, LangExt, ParseError, ParseOut};

/// Analogous to [`chumsky::primitive::just`].
/// Emits a [`GreenToken`] wrapped in a [`rowan::NodeOrToken::Token`].
pub fn just<L>(
	inputs: &'static str,
	syn: L::Kind,
) -> impl Parser<char, ParseOut, Error = ParseError>
where
	L: rowan::Language,
	L::Kind: Into<SyntaxKind>,
{
	primitive::just(inputs).map(move |string| ParseOut::Token(GreenToken::new(syn.into(), string)))
}

/// (G)ZDoom's DSLs almost always put their keywords and identifiers in an
/// ASCII-case-insensitive namespace. Chumsky offers no good singular combinator
/// for this, so we have our own.
#[must_use]
pub fn just_nc(string: &'static str) -> impl Parser<char, (), Error = ParseError> {
	text::ident().try_map(move |s: String, span| {
		s.eq_ignore_ascii_case(string)
			.then_some(())
			.ok_or_else(|| ParseError::expected_input_found(span, None, None))
	})
}

/// The most common kind of whitespace;
/// spaces, carriage returns, newlines, and/or tabs, repeated one or more times.
pub fn wsp<L>(src: &str) -> impl Parser<char, ParseOut, Error = ParseError> + '_
where
	L: LangExt,
	L::Kind: Into<SyntaxKind>,
{
	primitive::one_of([' ', '\r', '\n', '\t'])
		.repeated()
		.at_least(1)
		.map_with_span(|_, span| {
			ParseOut::Token(GreenToken::new(L::SYN_WHITESPACE.into(), &src[span]))
		})
}

/// For languages that treat comments as though they were whitespace.
pub fn wsp_ext<'src, L, C>(
	src: &'src str,
	comment: C,
) -> impl Parser<char, ParseOut, Error = ParseError> + 'src
where
	L: LangExt,
	L::Kind: Into<SyntaxKind> + 'static,
	C: Parser<char, ParseOut, Error = ParseError> + 'src,
{
	primitive::choice((wsp::<L>(src), comment))
}

/// Single-line comments delimited by `//`. Used by ACS and (G)ZDoom's languages.
pub fn cpp_comment<L>(src: &str) -> impl Parser<char, ParseOut, Error = ParseError> + '_
where
	L: LangComment,
	L::Kind: Into<SyntaxKind>,
{
	primitive::just("//")
		.then(primitive::take_until(text::newline()))
		.map_with_span(|_, span| {
			ParseOut::Token(GreenToken::new(L::SYN_COMMENT.into(), &src[span]))
		})
}

/// Multi-line comments delimited by `/*` and `*/`.
/// Used by ACS and (G)ZDoom's languages.
pub fn c_comment<L>(src: &str) -> impl Parser<char, ParseOut, Error = ParseError> + '_
where
	L: LangComment,
	L::Kind: Into<SyntaxKind>,
{
	primitive::just("/*")
		.then(primitive::take_until(primitive::just("*/")))
		.map_with_span(|_, span| {
			ParseOut::Token(GreenToken::new(L::SYN_COMMENT.into(), &src[span]))
		})
}

/// Shorthand for `primitive::choice((c_comment::<L>(src), cpp_comment::<L>(src)))`.
pub fn c_cpp_comment<L>(src: &str) -> impl Parser<char, ParseOut, Error = ParseError> + '_
where
	L: LangComment,
	L::Kind: Into<SyntaxKind>,
{
	primitive::choice((c_comment::<L>(src), cpp_comment::<L>(src)))
}

/// Shorthand for `chumsky::primitive::one_of("0123456789")`.
#[must_use]
pub fn ascii_digit() -> chumsky::primitive::OneOf<char, &'static str, ParseError> {
	primitive::one_of("0123456789")
}

/// Shorthand for `chumsky::primitive::one_of("0123456abcdefABCDEF")`.
#[must_use]
pub fn hex_digit() -> chumsky::primitive::OneOf<char, &'static str, ParseError> {
	primitive::one_of("0123456abcdefABCDEF")
}

/// Shorthand for `chumsky::primitive::one_of("01234567")`.
#[must_use]
pub fn oct_digit() -> chumsky::primitive::OneOf<char, &'static str, ParseError> {
	primitive::one_of("01234567")
}

/// C/C++-style integer literals (`z` suffix excluded) for (G)ZDoom.
#[must_use]
pub(crate) fn c_int<L>(
	src: &str,
	syn: L::Kind,
) -> impl Parser<char, ParseOut, Error = ParseError> + '_
where
	L: rowan::Language + 'static,
	L::Kind: Into<SyntaxKind>,
{
	let hex = primitive::just('0')
		.then(primitive::one_of(['x', 'X']))
		.then(
			primitive::one_of("abcdefABCDEF0123456789")
				.repeated()
				.at_least(1),
		)
		.then(primitive::one_of(['u', 'U', 'l', 'L']).or_not())
		.then(primitive::one_of(['u', 'U', 'l', 'L']).or_not())
		.map_with_span(move |_, span| ParseOut::Token(GreenToken::new(syn.into(), &src[span])));

	// Q: (G)ZDoom's lexer accepts invalid octals using 8 and 9, but they then
	// get rejected by the number evaluator. Should this behavior be imitated?

	let oct = primitive::just('0')
		.then(primitive::one_of("01234567").repeated().at_least(1))
		.then(primitive::one_of(['u', 'U', 'l', 'L']).or_not())
		.then(primitive::one_of(['u', 'U', 'l', 'L']).or_not())
		.map_with_span(move |_, span| ParseOut::Token(GreenToken::new(syn.into(), &src[span])));

	let dec = primitive::one_of("0123456789")
		.repeated()
		.at_least(1)
		.then(primitive::one_of(['u', 'U', 'l', 'L']).or_not())
		.then(primitive::one_of(['u', 'U', 'l', 'L']).or_not())
		.map_with_span(move |_, span| ParseOut::Token(GreenToken::new(syn.into(), &src[span])));

	primitive::choice((hex, oct, dec))
}

#[must_use]
pub(crate) fn c_float<L>(
	src: &str,
	syn: L::Kind,
) -> impl Parser<char, ParseOut, Error = ParseError> + '_
where
	L: rowan::Language + 'static,
	L::Kind: Into<SyntaxKind>,
{
	let fl_exp = primitive::one_of(['e', 'E'])
		.then(primitive::one_of(['+', '-']).or_not())
		.then(ascii_digit().repeated().at_least(1));

	let fl_suffix = primitive::one_of(['f', 'F']);

	let no_point = ascii_digit()
		.repeated()
		.at_least(1)
		.then(fl_exp.clone())
		.then(fl_suffix.clone().or_not())
		.map_with_span(move |_, span| ParseOut::Token(GreenToken::new(syn.into(), &src[span])));

	let l_opt = ascii_digit()
		.repeated()
		.then(primitive::just('.'))
		.then(ascii_digit().repeated().at_least(1))
		.then(fl_exp.clone().or_not())
		.then(fl_suffix.clone().or_not())
		.map_with_span(move |_, span| ParseOut::Token(GreenToken::new(syn.into(), &src[span])));

	let r_opt = ascii_digit()
		.repeated()
		.at_least(1)
		.then(primitive::just('.'))
		.then(ascii_digit().repeated())
		.then(fl_exp.or_not())
		.then(fl_suffix.or_not())
		.map_with_span(move |_, span| ParseOut::Token(GreenToken::new(syn.into(), &src[span])));

	primitive::choice((no_point, l_opt, r_opt))
}
