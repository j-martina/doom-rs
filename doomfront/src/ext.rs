//! Traits extending [`chumsky::Parser`] with `doomfront`-specific helpers.

#![allow(clippy::type_complexity)] // Each complex type is only used once.

use arrayvec::ArrayVec;
use chumsky::{
	combinator::{Map, Then},
	Parser,
};
use rowan::{GreenNode, SyntaxKind};
use smallvec::{smallvec, SmallVec};

use crate::{ParseError, ParseOut};

// Q: Do functions like `Parser1::remap` cause i-cache bloat?

/// Makes it easy to create a new collection from a single [`ParseOut`].
pub trait Parser1: Parser<char, ParseOut, Error = ParseError> {
	/// Creates a parser that puts a [`ParseOut`] into a new `Vec`.
	fn start_vec(self) -> Map<Self, fn(ParseOut) -> Vec<ParseOut>, ParseOut>
	where
		Self: Sized,
	{
		self.map(|n_or_t| vec![n_or_t])
	}

	/// Creates a parser that puts a node into a new `SmallVec`.
	/// For use when parsing syntax groups whose size in tokens and nodes is
	/// unbounded but will almost always be small.
	fn start_small<const N: usize>(
		self,
	) -> Map<Self, fn(ParseOut) -> SmallVec<[ParseOut; N]>, ParseOut>
	where
		Self: Sized,
	{
		self.map(|n_or_t| smallvec![n_or_t])
	}

	/// Creates a parser that puts a node into a new `ArrayVec`.
	/// For use when parsing syntax groups whose size in tokens and nodes is fixed.
	fn start_arr<const N: usize>(self) -> Map<Self, fn(ParseOut) -> ArrayVec<ParseOut, N>, ParseOut>
	where
		Self: Sized,
	{
		self.map(|n_or_t| {
			let mut ret = ArrayVec::new();
			ret.push(n_or_t);
			ret
		})
	}

	/// For creating a parser that wraps a node or token inside a new node.
	/// `K` should be cast down from one of the variants of `L::Kind`;
	/// this function panics unconditionally if `K` is not a valid `L::Kind`.
	fn remap<L, const K: u16>(self) -> Map<Self, fn(ParseOut) -> ParseOut, ParseOut>
	where
		Self: Sized,
		L: rowan::Language,
		L::Kind: Into<SyntaxKind> + 'static,
	{
		self.map(|n_or_t| {
			let raw = L::kind_from_raw(SyntaxKind(K));
			ParseOut::Node(GreenNode::new(raw.into(), [n_or_t]))
		})
	}
}

impl<T> Parser1 for T where T: Parser<char, ParseOut, Error = ParseError> {}

/// Makes it easy to create a new collection from an `Option` that may hold a [`ParseOut`].
pub trait ParserOpt: Parser<char, Option<ParseOut>, Error = ParseError> {
	fn start_vec(self) -> Map<Self, fn(Option<ParseOut>) -> Vec<ParseOut>, Option<ParseOut>>
	where
		Self: Sized,
	{
		self.map(|opt| match opt {
			Some(n_or_t) => vec![n_or_t],
			None => vec![],
		})
	}

	fn start_small<const N: usize>(
		self,
	) -> Map<Self, fn(Option<ParseOut>) -> SmallVec<[ParseOut; N]>, Option<ParseOut>>
	where
		Self: Sized,
	{
		self.map(|opt| match opt {
			Some(n_or_t) => smallvec![n_or_t],
			None => smallvec![],
		})
	}

	fn start_arr<const N: usize>(
		self,
	) -> Map<Self, fn(Option<ParseOut>) -> ArrayVec<ParseOut, N>, Option<ParseOut>>
	where
		Self: Sized,
	{
		self.map(|opt| match opt {
			Some(n_or_t) => {
				let mut ret = ArrayVec::default();
				ret.push(n_or_t);
				ret
			}
			None => ArrayVec::default(),
		})
	}
}

impl<T> ParserOpt for T where T: Parser<char, Option<ParseOut>, Error = ParseError> {}

/// Makes it easy to add to the end of a vec of [`ParseOut`]s, or to
/// convert one into a new [`ParseOut::Node`].
pub trait ParserVec: Parser<char, Vec<ParseOut>, Error = ParseError> {
	/// Shorthand for `parser.then(other).map(closure)` where `closure`
	/// pushes the output from `other` to the vec and then returns the vec.
	fn chain_push<P>(
		self,
		other: P,
	) -> Map<Then<Self, P>, fn((Vec<ParseOut>, ParseOut)) -> Vec<ParseOut>, (Vec<ParseOut>, ParseOut)>
	where
		Self: Sized,
		P: Parser<char, ParseOut, Error = Self::Error>,
	{
		self.then(other).map(|(mut vec, n_or_t)| {
			vec.push(n_or_t);
			vec
		})
	}

	/// Pushes the option to the vec if it is `Some`, or else does nothing.
	/// Either way the vec is returned.
	fn chain_push_opt<P>(
		self,
		other: P,
	) -> Map<
		Then<Self, P>,
		fn((Vec<ParseOut>, Option<ParseOut>)) -> Vec<ParseOut>,
		(Vec<ParseOut>, Option<ParseOut>),
	>
	where
		Self: Sized,
		P: Parser<char, Option<ParseOut>, Error = Self::Error>,
	{
		self.then(other).map(|(mut vec, opt)| {
			if let Some(n_or_t) = opt {
				vec.push(n_or_t);
			}

			vec
		})
	}

	/// Appends the elements in the second mapped vec to the first,
	/// and then returns the first.
	fn chain_append<P>(
		self,
		other: P,
	) -> Map<
		Then<Self, P>,
		fn((Vec<ParseOut>, Vec<ParseOut>)) -> Vec<ParseOut>,
		(Vec<ParseOut>, Vec<ParseOut>),
	>
	where
		Self: Sized,
		P: Parser<char, Vec<ParseOut>, Error = Self::Error>,
	{
		self.then(other).map(|(mut vec, mut consumed)| {
			vec.append(&mut consumed);
			vec
		})
	}

	/// Appends the elements in the mapped [`SmallVec`] to the first,
	/// and then returns the first.
	fn chain_append_small<P, const N: usize>(
		self,
		other: P,
	) -> Map<
		Then<Self, P>,
		fn((Vec<ParseOut>, SmallVec<[ParseOut; N]>)) -> Vec<ParseOut>,
		(Vec<ParseOut>, SmallVec<[ParseOut; N]>),
	>
	where
		Self: Sized,
		P: Parser<char, SmallVec<[ParseOut; N]>, Error = Self::Error>,
	{
		self.then(other).map(|(mut vec, consumed)| {
			consumed.into_iter().for_each(|n_or_t| vec.push(n_or_t));
			vec
		})
	}

	/// `K` should be cast down from one of the variants of `L::Kind`;
	/// this function panics unconditionally if `K` is not a valid `L::Kind`.
	fn collect_n<L, const K: u16>(self) -> Map<Self, fn(Vec<ParseOut>) -> ParseOut, Vec<ParseOut>>
	where
		Self: Sized,
		L: rowan::Language,
		L::Kind: Into<SyntaxKind> + 'static,
	{
		self.map(|vec| {
			let raw = L::kind_from_raw(SyntaxKind(K));
			ParseOut::Node(GreenNode::new(raw.into(), vec))
		})
	}

	/// Like `collect_n` but does not wrap the emitted [`GreenNode`] in a
	/// [`ParseOut`]. For use when creating the root of a parse tree.
	///
	/// `K` should be cast down from one of the variants of `L::Kind`;
	/// this function panics unconditionally if `K` is not a valid `L::Kind`.
	fn collect_g<L, const K: u16>(self) -> Map<Self, fn(Vec<ParseOut>) -> GreenNode, Vec<ParseOut>>
	where
		Self: Sized,
		L: rowan::Language,
		L::Kind: Into<SyntaxKind> + 'static,
	{
		self.map(|vec| {
			let raw = L::kind_from_raw(SyntaxKind(K));
			GreenNode::new(raw.into(), vec)
		})
	}
}

impl<T> ParserVec for T where T: Parser<char, Vec<ParseOut>, Error = ParseError> {}

/// Makes it easy to add to the end of a [`SmallVec`] of [`ParseOut`]s, or to
/// convert one into a new [`ParseOut::Node`].
pub trait ParserSmall<const N: usize>:
	Parser<char, SmallVec<[ParseOut; N]>, Error = ParseError>
{
	/// Shorthand for `parser.then(other).map(closure)` where `closure`
	/// pushes the output from `other` to the vec and then returns the vec.
	fn chain_push<P>(
		self,
		other: P,
	) -> Map<
		Then<Self, P>,
		fn((SmallVec<[ParseOut; N]>, ParseOut)) -> SmallVec<[ParseOut; N]>,
		(SmallVec<[ParseOut; N]>, ParseOut),
	>
	where
		Self: Sized,
		P: Parser<char, ParseOut, Error = Self::Error>,
	{
		self.then(other).map(|(mut vec, n_or_t)| {
			vec.push(n_or_t);
			vec
		})
	}

	/// Pushes the option to the vec if it is `Some`, or else does nothing.
	/// Either way the vec is returned.
	fn chain_push_opt<P>(
		self,
		other: P,
	) -> Map<
		Then<Self, P>,
		fn((SmallVec<[ParseOut; N]>, Option<ParseOut>)) -> SmallVec<[ParseOut; N]>,
		(SmallVec<[ParseOut; N]>, Option<ParseOut>),
	>
	where
		Self: Sized,
		P: Parser<char, Option<ParseOut>, Error = Self::Error>,
	{
		self.then(other).map(|(mut vec, opt)| {
			if let Some(n_or_t) = opt {
				vec.push(n_or_t);
			}

			vec
		})
	}

	/// Appends the elements in the second mapped vec to the first,
	/// and then returns the first.
	fn chain_append<P>(
		self,
		other: P,
	) -> Map<
		Then<Self, P>,
		fn((SmallVec<[ParseOut; N]>, SmallVec<[ParseOut; N]>)) -> SmallVec<[ParseOut; N]>,
		(SmallVec<[ParseOut; N]>, SmallVec<[ParseOut; N]>),
	>
	where
		Self: Sized,
		P: Parser<char, SmallVec<[ParseOut; N]>, Error = Self::Error>,
	{
		self.then(other).map(|(mut vec, mut consumed)| {
			vec.append(&mut consumed);
			vec
		})
	}

	/// `K` should be cast down from one of the variants of `L::Kind`;
	/// this function panics unconditionally if `K` is not a valid `L::Kind`.
	fn collect_n<L, const K: u16>(
		self,
	) -> Map<Self, fn(SmallVec<[ParseOut; N]>) -> ParseOut, SmallVec<[ParseOut; N]>>
	where
		Self: Sized,
		L: rowan::Language,
		L::Kind: Into<SyntaxKind> + 'static,
	{
		self.map(|vec| {
			let raw = L::kind_from_raw(SyntaxKind(K));
			ParseOut::Node(GreenNode::new(raw.into(), vec))
		})
	}
}

impl<T, const N: usize> ParserSmall<N> for T where
	T: Parser<char, SmallVec<[ParseOut; N]>, Error = ParseError>
{
}

/// Makes it easy to add to the end of an [`ArrayVec`] of [`ParseOut`]s, or to
/// convert one into a new [`ParseOut::Node`].
pub trait ParserArray<const N: usize>:
	Parser<char, ArrayVec<ParseOut, N>, Error = ParseError>
{
	/// Shorthand for `parser.then(other).map(closure)` where `closure`
	/// pushes the output from `other` to the vec and then returns the vec.
	fn chain_push<P>(
		self,
		other: P,
	) -> Map<
		Then<Self, P>,
		fn((ArrayVec<ParseOut, N>, ParseOut)) -> ArrayVec<ParseOut, N>,
		(ArrayVec<ParseOut, N>, ParseOut),
	>
	where
		Self: Sized,
		P: Parser<char, ParseOut, Error = Self::Error>,
	{
		self.then(other).map(|(mut vec, n_or_t)| {
			vec.push(n_or_t);
			vec
		})
	}

	/// Pushes the option to the vec if it is `Some`, or else does nothing.
	/// Either way the vec is returned.
	fn chain_push_opt<P>(
		self,
		other: P,
	) -> Map<
		Then<Self, P>,
		fn((ArrayVec<ParseOut, N>, Option<ParseOut>)) -> ArrayVec<ParseOut, N>,
		(ArrayVec<ParseOut, N>, Option<ParseOut>),
	>
	where
		Self: Sized,
		P: Parser<char, Option<ParseOut>, Error = Self::Error>,
	{
		self.then(other).map(|(mut vec, opt)| {
			if let Some(n_or_t) = opt {
				vec.push(n_or_t);
			}

			vec
		})
	}

	fn chain_append<P>(
		self,
		other: P,
	) -> Map<
		Then<Self, P>,
		fn((ArrayVec<ParseOut, N>, ArrayVec<ParseOut, N>)) -> ArrayVec<ParseOut, N>,
		(ArrayVec<ParseOut, N>, ArrayVec<ParseOut, N>),
	>
	where
		Self: Sized,
		P: Parser<char, ArrayVec<ParseOut, N>, Error = Self::Error>,
	{
		self.then(other).map(|(mut vec, consumed)| {
			for n_or_t in consumed {
				vec.push(n_or_t);
			}

			vec
		})
	}

	/// `K` should be cast down from one of the variants of `L::Kind`;
	/// this function panics unconditionally if `K` is not a valid `L::Kind`.
	fn collect_n<L, const K: u16>(
		self,
	) -> Map<Self, fn(ArrayVec<ParseOut, N>) -> ParseOut, ArrayVec<ParseOut, N>>
	where
		Self: Sized,
		L: rowan::Language,
		L::Kind: Into<SyntaxKind> + 'static,
	{
		self.map(|vec| {
			let raw = L::kind_from_raw(SyntaxKind(K));
			ParseOut::Node(GreenNode::new(raw.into(), vec))
		})
	}
}

impl<T, const N: usize> ParserArray<N> for T where
	T: Parser<char, ArrayVec<ParseOut, N>, Error = ParseError>
{
}
