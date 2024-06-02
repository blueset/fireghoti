use crate::database::db_conn;
use crate::model::entity::note_watching;
use crate::util::id::gen_id;
use sea_orm::{prelude::*, ActiveValue};

#[crate::export]
pub async fn watch_note(
    watcher_id: &str,
    note_author_id: &str,
    note_id: &str,
) -> Result<(), DbErr> {
    if watcher_id != note_author_id {
        note_watching::Entity::insert(note_watching::ActiveModel {
            id: ActiveValue::set(gen_id()),
            created_at: ActiveValue::set(chrono::Utc::now().into()),
            user_id: ActiveValue::Set(watcher_id.to_string()),
            note_user_id: ActiveValue::Set(note_author_id.to_string()),
            note_id: ActiveValue::Set(note_id.to_string()),
        })
        .exec(db_conn().await?)
        .await?;
    }

    Ok(())
}

#[crate::export]
pub async fn unwatch_note(watcher_id: &str, note_id: &str) -> Result<(), DbErr> {
    let db = db_conn().await?;

    let entry = note_watching::Entity::find()
        .filter(note_watching::Column::UserId.eq(watcher_id))
        .filter(note_watching::Column::NoteId.eq(note_id))
        .one(db)
        .await?;

    if let Some(entry) = entry {
        entry.delete(db).await?;
    }

    Ok(())
}
