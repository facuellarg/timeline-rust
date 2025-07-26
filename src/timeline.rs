use sqlx::FromRow;

#[derive(FromRow)]
pub struct Timeline {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub created_at: Option<time::OffsetDateTime>,
    pub updated_at: Option<time::OffsetDateTime>,
}

impl std::fmt::Display for Timeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Timeline {{ id: {}, title: {:?}, description: {:?}, created_at: {:?}, updated_at: {:?} }}",
            self.id, self.title, self.description, self.created_at, self.updated_at
        )
    }
}

impl Timeline {
    pub fn new(title: String, description: String) -> Self {
        Timeline {
            id: 0, // Assuming id is auto-incremented by the database
            title,
            description: Some(description),
            created_at: Some(time::OffsetDateTime::now_utc()),
            updated_at: Some(time::OffsetDateTime::now_utc()),
        }
    }

    pub async fn create_timeline(pool: &sqlx::PgPool) -> Result<Timeline, sqlx::Error> {
        let timeline = sqlx::query_as!(
            Timeline,
            "INSERT INTO timeline (title, description) VALUES ($1, $2) RETURNING *",
            "My first timeline",
            "This is my first timeline",
        )
        .fetch_one(pool)
        .await?;
        Ok(timeline)
    }
}
