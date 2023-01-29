//! # `subterra`
//!
//! ## About
//!
//! Parser for the [MUS file format](https://doomwiki.org/wiki/MUS),
//! a modified MIDI specification.

/// Checks for a 4-byte magic number at the top of the standard header.
#[must_use]
pub fn is_mus(bytes: &[u8]) -> bool {
	if bytes.len() < 4 {
		return false;
	}

	bytes[0] == b'M' && bytes[1] == b'U' && bytes[2] == b'S' && bytes[3] == 0x1A
}

// Soon!
