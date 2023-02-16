//! Helpers for mapping parsing output.

use std::ops::Range;

use rowan::{GreenNode, GreenToken, SyntaxKind};

use crate::ParseOut;

/// Returns a closure to pass to [`chumsky::Parser::map_with_span`].
/// Builds a [`GreenToken`] using a syntax tag and source slice.
pub fn map_tok<L, O>(src: &str, syn: L::Kind) -> impl Fn(O, Range<usize>) -> ParseOut + Clone + '_
where
	L: rowan::Language,
	L::Kind: Into<SyntaxKind> + 'static,
{
	move |_, span| ParseOut::Token(GreenToken::new(syn.into(), &src[span]))
}

/// Returns a closure that inserts a [`ParseOut`] node or token into another
/// [`ParseOut`] node. Pass this closure to [`chumsky::Parser::map`].
pub fn map_node<L>(syn: L::Kind) -> impl Fn(ParseOut) -> ParseOut + Clone
where
	L: rowan::Language,
	L::Kind: Into<SyntaxKind> + 'static,
{
	move |n_or_t| ParseOut::Node(GreenNode::new(syn.into(), [n_or_t]))
}
