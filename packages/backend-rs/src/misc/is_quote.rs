use crate::model::entity::note;

// https://github.com/napi-rs/napi-rs/issues/2060
type Note = note::Model;

#[crate::export]
pub fn is_quote(note: Note) -> bool {
    note.renote_id.is_some() && (note.text.is_some() || note.has_poll || !note.file_ids.is_empty())
}
