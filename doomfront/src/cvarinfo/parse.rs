use chumsky::{primitive, recovery, text, Parser};

use crate::{
	comb,
	ext::{Parser1, ParserVec},
	help, ParseError, ParseOut,
};

use super::{RawParseTree, Syn};

/// Upon encountering any error, the parser will immediately stop.
/// Prefer [`parse_recov`] since it allows providing the user with more
/// actionable information, enabling more fixing of one's code at once.
pub fn parse(source: &str) -> Result<RawParseTree, Vec<ParseError>> {
	let parser = primitive::choice((wsp_ext(source), definition(source)))
		.repeated()
		.collect_g::<Syn, { Syn::Root as u16 }>();

	parser
		.parse(source)
		.map(|root| RawParseTree::new(root, vec![]))
}

/// "Recoverable parse". Unless `source` has no tokens whatsoever, this always
/// emits `Some`, although the returned tree may have errors attached.
///
/// When faced with unexpected input, the parser raises an error and then tries
/// to skip ahead to the next CVar definition, carriage return, or newline.
/// All input between the error location and the next valid thing gets wrapped
/// into a token tagged [`Syn::Unknown`].
#[must_use]
pub fn parse_recov(source: &str) -> Option<RawParseTree> {
	let parser = primitive::choice((
		wsp_ext(source),
		definition(source).recover_with(recovery::skip_parser(recover(source))),
	))
	.repeated()
	.collect_g::<Syn, { Syn::Root as u16 }>();

	let (root, errs) = parser.parse_recovery(source);

	root.map(|r| RawParseTree::new(r, errs))
}

fn definition(src: &str) -> impl Parser<char, ParseOut, Error = ParseError> + Clone + '_ {
	flags(src)
		.start_vec()
		.chain_push(type_spec(src))
		.chain_append(wsp_ext(src).repeated().at_least(1))
		.chain_push(text::ident().map_with_span(help::map_tok::<Syn, _>(src, Syn::Ident)))
		.chain_push_opt(default(src).or_not())
		.chain_push(comb::just::<Syn, _>(';', Syn::Semicolon, src))
		.collect_n::<Syn, { Syn::Definition as u16 }>()
}

fn flags(src: &str) -> impl Parser<char, ParseOut, Error = ParseError> + Clone + '_ {
	primitive::choice((flag(src), wsp_ext(src)))
		.repeated()
		.at_least(1)
		.labelled("flags or scope specifiers")
		.collect_n::<Syn, { Syn::Flags as u16 }>()
}

fn flag(src: &str) -> impl Parser<char, ParseOut, Error = ParseError> + Clone + '_ {
	primitive::choice((
		comb::just_nc("server").map_with_span(help::map_tok::<Syn, _>(src, Syn::KwServer)),
		comb::just_nc("user").map_with_span(help::map_tok::<Syn, _>(src, Syn::KwUser)),
		comb::just_nc("nosave").map_with_span(help::map_tok::<Syn, _>(src, Syn::KwNoSave)),
		comb::just_nc("noarchive").map_with_span(help::map_tok::<Syn, _>(src, Syn::KwNoArchive)),
		comb::just_nc("cheat").map_with_span(help::map_tok::<Syn, _>(src, Syn::KwCheat)),
		comb::just_nc("latch").map_with_span(help::map_tok::<Syn, _>(src, Syn::KwLatch)),
	))
	.labelled("flag keyword")
}

fn type_spec(src: &str) -> impl Parser<char, ParseOut, Error = ParseError> + Clone + '_ {
	primitive::choice((
		comb::just_nc("int").map_with_span(help::map_tok::<Syn, _>(src, Syn::TypeInt)),
		comb::just_nc("float").map_with_span(help::map_tok::<Syn, _>(src, Syn::TypeFloat)),
		comb::just_nc("bool").map_with_span(help::map_tok::<Syn, _>(src, Syn::TypeBool)),
		comb::just_nc("color").map_with_span(help::map_tok::<Syn, _>(src, Syn::TypeColor)),
		comb::just_nc("string").map_with_span(help::map_tok::<Syn, _>(src, Syn::TypeString)),
	))
	.labelled("type specifier")
}

fn default(src: &str) -> impl Parser<char, ParseOut, Error = ParseError> + Clone + '_ {
	let escape = primitive::just('\\').ignore_then(
		primitive::just('\\')
			.or(primitive::just('a').to('\x07'))
			.or(primitive::just('b').to('\x08'))
			.or(primitive::just('c'))
			.or(primitive::just('f').to('\x0C'))
			.or(primitive::just('n').to('\n'))
			.or(primitive::just('r').to('\r'))
			.or(primitive::just('t').to('\t'))
			.or(primitive::just('v'))
			.or(primitive::just('\''))
			.or(primitive::just('"'))
			.or(primitive::just('\\'))
			.ignore_then(
				primitive::filter(|c: &char| c.is_ascii_hexdigit())
					.repeated()
					.exactly(4)
					.collect::<String>()
					.validate(|digits, span, emit| {
						char::from_u32(u32::from_str_radix(&digits, 16).unwrap()).unwrap_or_else(
							|| {
								emit(ParseError::custom(span, "invalid UTF-8 character"));
								'\u{FFFD}' // Unicode replacement character
							},
						)
					}),
			),
	);

	let lit = primitive::choice((
		comb::c_float::<Syn>(src, Syn::LitFloat).labelled("floating-point literal"),
		comb::c_int::<Syn>(src, Syn::LitInt).labelled("integer literal"),
		comb::just::<Syn, _>("true", Syn::LitTrue, src),
		comb::just::<Syn, _>("false", Syn::LitFalse, src),
		primitive::just('"')
			.then(
				primitive::filter(|&c| c != '\\' && c != '"')
					.or(escape)
					.repeated(),
			)
			.then(primitive::just('"'))
			.map_with_span(help::map_tok::<Syn, _>(src, Syn::LitString))
			.labelled("string literal"),
	));

	wsp_ext(src)
		.repeated()
		.at_least(1)
		.chain_push(comb::just::<Syn, _>('=', Syn::Eq, src))
		.chain_append(wsp_ext(src).repeated())
		.chain_push(lit)
		.collect_n::<Syn, { Syn::DefaultDef as u16 }>()
}

fn wsp_ext(src: &str) -> impl Parser<char, ParseOut, Error = ParseError> + Clone + '_ {
	comb::wsp_ext::<Syn, _>(comb::c_cpp_comment::<Syn>(src), src)
}

fn recover(src: &str) -> impl Parser<char, ParseOut, Error = ParseError> + Clone + '_ {
	primitive::take_until(primitive::one_of([';', '\r', '\n']))
		.map_with_span(help::map_tok::<Syn, _>(src, Syn::Unknown))
}
