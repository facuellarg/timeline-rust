use sqlx::FromRow;

use crate::timeline::Timeline;

#[derive(FromRow)]
pub struct Event {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub timeline_id: i32,
    pub created_at: Option<time::OffsetDateTime>,
    pub updated_at: Option<time::OffsetDateTime>,
    pub image: Option<String>,
}

pub struct EventDto {
    pub title : Option<String>,
    pub description: Option<String>,
    pub timeline_id: i32,
    pub image: Option<String>,
}

impl Event {
    pub fn new(
        name: Option<String>,
        description: Option<String>,
        timeline_id: i32,
        image: Option<String>,
    ) -> Self {
        Event {
            id: 0, // Assuming id is auto-incremented by the database
            title: name,
            description,
            timeline_id,
            created_at: Some(time::OffsetDateTime::now_utc()),
            updated_at: Some(time::OffsetDateTime::now_utc()),
            image,
        }
    }

    pub async fn create_event(
        pool: &sqlx::PgPool,
        timeline: &Timeline,
    ) -> Result<Event, sqlx::Error> {
        let event = sqlx::query_as!(
        Event,
    "INSERT INTO event (title, description, image, timeline_id) VALUES ($1, $2, $3, $4) RETURNING *",
        "My first event",                                                //title
        Some("This is an event to test if sqlx is working as expected"), // description
        Some("https://example.com/image.jpg"),
        timeline.id
    )
    .fetch_one(pool)
    .await?;
        Ok(event)
    }

    pub async fn get_events(pool: &sqlx::PgPool) -> Result<Vec<Event>, sqlx::Error> {
        let events = sqlx::query_as!(Event, "SELECT * FROM event")
            .fetch_all(pool)
            .await?;
        Ok(events)
    }

    pub async fn get_event_by_id(pool: &sqlx::PgPool, id: i32) -> Result<Event, sqlx::Error> {
        let event = sqlx::query_as!(Event, "SELECT * FROM event WHERE id = $1", id)
            .fetch_one(pool)
            .await?;
        Ok(event)
    }

    pub async fn delete_event(pool: &sqlx::PgPool, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM event WHERE id = $1", id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Event {{ name: {:?}, description: {:?}, time_line_id: {}, datetime: {}, image: {:?} }}",
            self.title,
            self.description,
            "testing timeine", /* self.timeline_id */
            self.created_at.unwrap_or(time::OffsetDateTime::now_utc()),
            self.image
        )
    }
}
