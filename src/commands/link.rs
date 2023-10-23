//! This module is used by the link subcommand to link notes together.
//!
//! A single note can be linked to several other notes via a list of prev 
//! pointers. Each prev pointer has an associated branch name that is used
//! when consolidating notes. If link is called without a branch name, 
//! it will attempt to link via the 'default' branch name.
//!
//! # Example
//!
//! ```
//! nexus-cli link note_one.md note_two.md my_branch
//! ```
use std::path::PathBuf;

use crate::{errors::NexusResult, utils::get_note_names};

pub fn link_note(_: PathBuf, _: String, _: String, _: Option<String>) -> NexusResult<()> {
    let _ = get_note_names();

    Ok(())
}
