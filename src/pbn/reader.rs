//! PBN reader — re-exported from `bridge-encodings`, the org's single
//! full-fidelity PBN parser. This crate previously carried its own lossy copy
//! (dropped contract/auction/play/commentary and all supplemental tags); that
//! implementation has been retired in favor of the shared one, which returns
//! the same `bridge_types::Board` this crate already uses.

pub use bridge_encodings::pbn::{read_pbn, read_pbn_file, TagPair};
