use std::path::PathBuf;

use crate::{errors::NexusResult, utils::get_note_names};

pub fn link_note(_: PathBuf, _: String, _: String, _: Option<String>) -> NexusResult<()> {
    let _ = get_note_names();

    Ok(())
}
