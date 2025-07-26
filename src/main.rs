use event::Event;
use sqlx::migrate::MigrateDatabase;
use timeline::Timeline;
//docker run --name timeline-db -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres

pub mod event;
pub mod timeline;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        eprintln!("DATABASE_URL environment variable is not set.");
        std::process::exit(1);
    });

    if !sqlx::Postgres::database_exists(&database_url).await? {
        sqlx::Postgres::create_database(&database_url).await?;
        println!("Database created successfully!");
    }

    let pool = sqlx::PgPool::connect_lazy(&database_url)?;
    println!("Connected to the database successfully!");

    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("Database migrations applied successfully!");

    let timeline = Timeline::create_timeline(&pool).await?;
    println!("Created timeline: {}", timeline);

    let new_event = Event::create_event(&pool, &timeline).await?;
    println!("Created event: {}", new_event);

    let events = sqlx::query_as!(event::Event, "SELECT * FROM event")
        .fetch_all(&pool)
        .await?;

    for event in events {
        println!("Event id: {}, Event: {}", event.id, event);
    }

    let result = sqlx::query!("DELETE FROM event").execute(&pool).await?;
    println!("Deleted {} rows", result.rows_affected());

    Ok(())
    //ping the database to ensure the connection is valid
    // if let Err(e) = sqlx::query!("SELECT 1").execute(&pool).await {
    //     eprintln!("Failed to ping the database: {}", e);
    //     return;
    // }
}
