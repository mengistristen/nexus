use std::path::PathBuf;

use crate::utils::get_note_names;

pub fn link_note(_: PathBuf, _: String, _: String, _: Option<String>) {
    let _ = get_note_names();
}
