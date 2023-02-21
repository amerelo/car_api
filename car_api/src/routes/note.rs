use super::AppState;

use axum::extract::{Path, Query, State};
use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::Query as ExtraQuery;
use serde::{Deserialize, Serialize};

use sqlx::types::{time::OffsetDateTime, uuid::Uuid};
use std::sync::Arc;
use time::format_description::well_known::Rfc3339;

#[serde_with::serde_as]
#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct NoteInfo {
    pub note_id: Uuid,
    pub name: String,
    // pub group_name: Option<String>,
    pub content: String,
    pub tags: Option<Vec<String>>,
    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CreateNote {
    pub user_id: Uuid,
    // group_id or group_name ?
    pub name: String,
    pub content: String, // set as optional value
    pub tags: Option<Vec<String>>,
}

pub async fn create_note(
    State(state): State<Arc<AppState>>,
    Json(note): Json<CreateNote>,
) -> impl IntoResponse {
    let response =
        sqlx::query("INSERT INTO note(name, user_id, content, tags) VALUES ($1, $2, $3, $4)")
            .bind(&note.name)
            .bind(&note.user_id)
            .bind(&note.content)
            .bind(&note.tags)
            .execute(&state.pg_pool)
            .await;

    match response {
        Ok(val) => Ok((
            StatusCode::CREATED,
            format!("note created with success {:?}", val),
        )),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("An error happened {:?}", err),
        )),
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UpdateNote {
    pub note_id: Uuid,
    pub user_id: Uuid,
    pub content: Option<String>,
    pub name: Option<String>,
}

pub async fn update_note(
    State(state): State<Arc<AppState>>,
    Json(note): Json<UpdateNote>,
) -> impl IntoResponse {
    let response = sqlx::query(
        "UPDATE note SET (name, content) = (COALESCE($1, note.name), COALESCE($2, note.content)) WHERE note_id=$3 AND user_id=$4"
        )
        .bind(&note.name)
        .bind(&note.content)
        .bind(&note.note_id)
        .bind(&note.user_id)
        .execute(&state.pg_pool)
        .await;

    match response {
        Ok(val) => Ok((
            StatusCode::OK,
            format!("note updated with success {:?}", val),
        )),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("An error happened {:?}", err),
        )),
    }
}

#[derive(Deserialize)]
pub struct SelectedNote {
    pub note_id: Uuid,
    pub user_id: Uuid,
}

pub async fn delete_note(
    State(state): State<Arc<AppState>>,
    Json(note): Json<SelectedNote>,
) -> impl IntoResponse {
    let response = sqlx::query("DELETE FROM note WHERE note_id=$1 AND user_id=$2")
        .bind(note.note_id)
        .bind(note.user_id)
        .execute(&state.pg_pool)
        .await;

    match response {
        Ok(val) => Ok((
            StatusCode::OK,
            format!("note created with success {:?}", val),
        )),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("An error happened {:?}", err),
        )),
    }
}

pub async fn get_note_by_id(
    State(state): State<Arc<AppState>>,
    select_note: Query<SelectedNote>,
) -> impl IntoResponse {
    let notes = sqlx::query_as::<_, NoteInfo>(
        "SELECT * FROM note WHERE note_id=$1 AND user_id=$2 ORDER BY created_at DESC",
    )
    .bind(select_note.note_id)
    .bind(select_note.user_id)
    .fetch_all(&state.pg_pool)
    .await;

    match notes {
        Ok(notes) => {
            for note in notes.iter() {
                println!("- {:#?}", note);
            }

            Ok((StatusCode::OK, format!("note obtained")))
        }
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("An error happened {:?}", err),
        )),
    }
}

#[derive(Deserialize)]
pub struct SelectedUserNote {
    pub user_id: Uuid,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn get_user_notes(
    State(state): State<Arc<AppState>>,
    select_note: ExtraQuery<SelectedUserNote>,
) -> impl IntoResponse {
    let limit = select_note.limit.unwrap_or(20);
    let offset = select_note.offset.unwrap_or(0);

    let notes = sqlx::query_as::<_, NoteInfo>(
        r#"
            SELECT * FROM note
            WHERE user_id=$1
            ORDER BY created_at ASC
            LIMIT $2
            OFFSET $3
        "#, // DESC | ASC
    )
    .bind(select_note.user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.pg_pool)
    .await;

    match notes {
        Ok(notes) => Ok((StatusCode::OK, Json(notes))),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("An error happened {:?}", err),
        )),
    }
}

#[derive(Deserialize)]
pub struct SelectedUserNoteByTags {
    pub user_id: Uuid,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub tags: Vec<String>,
}

pub async fn get_user_notes_by_tags(
    State(state): State<Arc<AppState>>,
    select_note: ExtraQuery<SelectedUserNoteByTags>,
) -> impl IntoResponse {
    let limit = select_note.limit.unwrap_or(20);
    let offset = select_note.offset.unwrap_or(0);
    let tags = select_note.tags.clone();

    let notes = sqlx::query_as::<_, NoteInfo>(
        r#"
            SELECT * FROM note
            WHERE user_id=$1 AND CAST(tags as text[]) @> $2
            ORDER BY created_at ASC
            LIMIT $3
            OFFSET $4
        "#, // DESC | ASC
    )
    .bind(select_note.user_id)
    .bind(tags)
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.pg_pool)
    .await;

    match notes {
        Ok(notes) => Ok((StatusCode::OK, Json(notes))),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("An error happened {:?}", err),
        )),
    }
}
