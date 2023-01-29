//! Parser and syntax trees for the [`CVARINFO`](https://zdoom.org/wiki/CVARINFO)
//! lump defined by ZDoom-family source ports.
//!
//! Console variables or "CVars" are ZDoom's way of storing user preferences
//! and the de facto solution for persistent storage.

pub mod ast;
mod parse;
mod syn;
#[cfg(test)]
mod test;

pub use parse::*;
pub use syn::*;
