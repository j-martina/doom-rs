//! Utilities used for other unit tests.

use crate::{LangExt, RawParseTree};

pub(crate) fn assert_no_errors<L: LangExt>(pt: &RawParseTree<L>) {
	assert!(!pt.any_errors(), "Encountered errors: {}", {
		let mut output = String::default();

		for err in pt.errors() {
			output.push_str(&format!("{err:#?}"));
			output.push('\r');
			output.push('\n');
		}

		output
	});
}
