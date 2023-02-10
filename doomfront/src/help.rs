//! Helpers for mapping parsing output.

use std::ops::Range;

use rowan::{GreenNode, GreenToken, SyntaxKind};

use crate::ParseOut;

/// Returns a closure to pass to [`chumsky::Parser::map_with_span`].
/// Builds a [`GreenToken`] using a syntax tag and source slice.
pub fn map_tok<L, O>(src: &str, syn: L::Kind) -> impl Fn(O, Range<usize>) -> ParseOut + '_
where
	L: rowan::Language,
	L::Kind: Into<SyntaxKind> + 'static,
{
	move |_, span| ParseOut::Token(GreenToken::new(syn.into(), &src[span]))
}

/// Returns a closure that inserts a [`ParseOut`] node into another [`ParseOut`] node.
/// Pass this closure to [`chumsky::Parser::map`].
pub fn map_node<L>(syn: L::Kind) -> impl Fn(ParseOut) -> ParseOut
where
	L: rowan::Language,
	L::Kind: Into<SyntaxKind> + 'static,
{
	move |n_or_t| ParseOut::Node(GreenNode::new(syn.into(), [n_or_t]))
}

/// Returns a closure that starts a new array of [`ParseOut`] children, to be
/// later turned into a [`GreenNode`] (using [`map_collect`]).
/// Pass this closure to [`chumsky::Parser::map`].
pub fn map_nvec() -> impl Fn(ParseOut) -> Vec<ParseOut> {
	|outp| vec![outp]
}

/// Returns a closure that yields either a one-element vec or an empty vec.
/// Pass this closure to [`chumsky::Parser::map`].
pub fn map_nvec_opt() -> impl Fn(Option<ParseOut>) -> Vec<ParseOut> {
	|opt| match opt {
		Some(outp) => vec![outp],
		None => vec![],
	}
}

/// Returns a closure that adds a [`ParseOut`] to a vec and then returns the vec.
/// Pass this closure to [`chumsky::Parser::map`].
pub fn map_push() -> impl Fn((Vec<ParseOut>, ParseOut)) -> Vec<ParseOut> {
	move |(mut outs, outp)| {
		outs.push(outp);
		outs
	}
}

/// Returns a closure that might push a [`ParseOut`] and then returns the vec.
/// Pass this closure to [`chumsky::Parser::map`].
pub fn map_push_opt() -> impl Fn((Vec<ParseOut>, Option<ParseOut>)) -> Vec<ParseOut> {
	|(mut vec, opt)| {
		if let Some(outp) = opt {
			vec.push(outp);
		}

		vec
	}
}

/// Returns a closure that appends the content of vec 2 to vec 1, and then returns vec 1.
/// Pass this closure to [`chumsky::Parser::map`].
pub fn map_append() -> impl Fn((Vec<ParseOut>, Vec<ParseOut>)) -> Vec<ParseOut> {
	|(mut vec1, mut vec2)| {
		vec1.append(&mut vec2);
		vec1
	}
}

/// Returns a closure that makes a [`GreenNode`] from [`ParseOut`] children.
/// Pass this closure to [`chumsky::Parser::map`].
pub fn map_collect<L>(syn: L::Kind) -> impl Fn(Vec<ParseOut>) -> ParseOut
where
	L: rowan::Language,
	L::Kind: Into<SyntaxKind>,
{
	move |children| ParseOut::Node(GreenNode::new(syn.into(), children))
}

/// Like [`map_collect`] but does not wrap the new node in a [`ParseOut`].
/// For use when creating a source's root node.
pub fn map_finish<L>(syn: L::Kind) -> impl Fn(Vec<ParseOut>) -> GreenNode
where
	L: rowan::Language,
	L::Kind: Into<SyntaxKind>,
{
	move |children| GreenNode::new(syn.into(), children)
}
