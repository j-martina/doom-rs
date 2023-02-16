//! Pieces of an alternative way of build rowan green trees that doesn't require
//! parser combinators to capture the source string, but makes the parsers more
//! cumbersome and incurs extra cost when parsing is finished. Whether this is
//! faster is yet to be tested, and whether it is useful hinges on what becomes
//! of Chumsky's "zero-copy" initiative. For now, all of this is unused.

/// "Builder lexeme". Token output emitted by parsers in a `Vec` to be fed to a
/// [`rowan::GreenNodeBuilder`], which [creates a green tree](build_green_tree).
#[derive(Debug)]
pub enum BLex {
	/// See [`rowan::GreenNodeBuilder::start_node`].
	StartNode(rowan::SyntaxKind),
	/// Like `StartPoint` but uses the last checkpoint on a stack.
	StartNodeAt(rowan::SyntaxKind),
	/// See [`rowan::GreenNodeBuilder::checkpoint`]. The returned checkpoint gets
	/// pushed onto a stack to be consumed by `StartNodeAt`.
	Checkpoint,
	Token(rowan::SyntaxKind, std::ops::Range<usize>),
	/// See [`rowan::GreenNodeBuilder::finish_node`].
	FinishNode,
}

/// Assembles a [`rowan` green tree](rowan::GreenNode) from a stream of
/// instructions for a builder.
#[must_use]
pub fn build_green_tree(source: &str, parsed: Vec<BLex>) -> rowan::GreenNode {
	let mut builder = rowan::GreenNodeBuilder::new();
	let mut cpoints = vec![];

	for p in parsed {
		match p {
			BLex::StartNode(syn) => builder.start_node(syn),
			BLex::StartNodeAt(syn) => builder.start_node_at(
				cpoints
					.pop()
					.expect("Green node builder checkpoint mismatch."),
				syn,
			),
			BLex::Token(syn, span) => builder.token(syn, &source[span]),
			BLex::FinishNode => builder.finish_node(),
			BLex::Checkpoint => cpoints.push(builder.checkpoint()),
		}
	}

	builder.finish()
}
