use chumsky::{primitive, text, Parser};
use rowan::GreenNode;

use crate::{comb, help, ParseError, ParseOut, RawParseTree};

use super::Syn;

pub fn parse(source: &str) -> Result<RawParseTree<Syn>, Vec<ParseError>> {
	parser(source)
		.parse(source)
		.map(|root| RawParseTree::new(root, vec![]))
}

pub fn parse_recov(source: &str) -> Option<RawParseTree<Syn>> {
	let (root, errs) = parser(source).parse_recovery(source);
	root.map(|r| RawParseTree::new(r, errs))
}

pub fn parser(src: &str) -> impl Parser<char, GreenNode, Error = ParseError> + '_ {
	primitive::choice((wsp_ext(src), definition(src)))
		.labelled("0 or more CVar definitions")
		.repeated()
		.map(help::map_finish::<Syn>(Syn::Root))
}

pub fn parser_tolerant(src: &str) -> impl Parser<char, GreenNode, Error = ParseError> + '_ {
	primitive::choice((wsp_ext(src), definition(src), unknown(src)))
		.repeated()
		.map(help::map_finish::<Syn>(Syn::Root))
}

fn definition(src: &str) -> impl Parser<char, ParseOut, Error = ParseError> + '_ {
	flags(src)
		.map(help::map_nvec())
		.then(type_spec(src))
		.map(help::map_push())
		.then(wsp_ext(src))
		.map(help::map_push())
		.then(text::ident().map_with_span(help::map_tok::<Syn, _>(src, Syn::Ident)))
		.map(help::map_push())
		.then(default(src).or_not())
		.map(help::map_push_opt())
		.then(comb::just::<Syn>(";", Syn::Semicolon))
		.map(help::map_push())
		.map(help::map_collect::<Syn>(Syn::Definition))
}

fn flags(src: &str) -> impl Parser<char, ParseOut, Error = ParseError> + '_ {
	primitive::choice((flag(src), wsp_ext(src)))
		.repeated()
		.at_least(1)
		.map(help::map_collect::<Syn>(Syn::Flags))
}

fn flag(src: &str) -> impl Parser<char, ParseOut, Error = ParseError> + '_ {
	primitive::choice((
		comb::just_nc("server").map_with_span(help::map_tok::<Syn, _>(src, Syn::KwServer)),
		comb::just_nc("user").map_with_span(help::map_tok::<Syn, _>(src, Syn::KwUser)),
		comb::just_nc("nosave").map_with_span(help::map_tok::<Syn, _>(src, Syn::KwNoSave)),
		comb::just_nc("noarchive").map_with_span(help::map_tok::<Syn, _>(src, Syn::KwNoArchive)),
		comb::just_nc("cheat").map_with_span(help::map_tok::<Syn, _>(src, Syn::KwCheat)),
		comb::just_nc("latch").map_with_span(help::map_tok::<Syn, _>(src, Syn::KwLatch)),
	))
}

fn type_spec(src: &str) -> impl Parser<char, ParseOut, Error = ParseError> + '_ {
	primitive::choice((
		comb::just_nc("int").map_with_span(help::map_tok::<Syn, _>(src, Syn::TypeInt)),
		comb::just_nc("float").map_with_span(help::map_tok::<Syn, _>(src, Syn::TypeFloat)),
		comb::just_nc("bool").map_with_span(help::map_tok::<Syn, _>(src, Syn::TypeBool)),
		comb::just_nc("color").map_with_span(help::map_tok::<Syn, _>(src, Syn::TypeColor)),
		comb::just_nc("string").map_with_span(help::map_tok::<Syn, _>(src, Syn::TypeString)),
	))
}

fn default(src: &str) -> impl Parser<char, ParseOut, Error = ParseError> + '_ {
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
								emit(ParseError::custom(span, "invalid unicode character"));
								'\u{FFFD}' // unicode replacement character
							},
						)
					}),
			),
	);

	let lit = primitive::choice((
		comb::c_float::<Syn>(src, Syn::LitFloat).labelled("floating-point literal"),
		comb::c_int::<Syn>(src, Syn::LitInt).labelled("integer literal"),
		comb::just::<Syn>("true", Syn::LitTrue),
		comb::just::<Syn>("false", Syn::LitFalse),
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
		.or_not()
		.map(help::map_nvec_opt())
		.then(comb::just::<Syn>("=", Syn::Eq))
		.map(help::map_push())
		.then(wsp_ext(src).or_not())
		.map(help::map_push_opt())
		.then(lit)
		.map(help::map_push())
		.map(help::map_collect::<Syn>(Syn::DefaultDef))
}

fn wsp_ext(src: &str) -> impl Parser<char, ParseOut, Error = ParseError> + '_ {
	comb::wsp_ext::<Syn, _>(src, comb::c_cpp_comment::<Syn>(src))
}

fn unknown(src: &str) -> impl Parser<char, ParseOut, Error = ParseError> + '_ {
	wsp_ext(src)
		.not()
		.repeated()
		.at_least(1)
		.map_with_span(help::map_tok::<Syn, _>(src, Syn::Unknown))
}
