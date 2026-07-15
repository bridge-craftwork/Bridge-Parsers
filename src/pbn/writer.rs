//! PBN writer — re-exported from `bridge-encodings`. The previous local copy
//! (which could not emit the auction/contract/supplemental tags the shared
//! reader now parses) has been retired in favor of the shared implementation.

pub use bridge_encodings::pbn::{board_to_pbn, write_pbn, write_pbn_file};
