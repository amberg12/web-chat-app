use sqlx::{migrate::MigrateDatabase, sqlite::SqliteQueryResult, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://sqlite.db";

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("ya"),
            Err(e) => panic!("error: {}", e),
        }
    } else {
        println!("exists");
    }
    SqlitePool::connect(DB_URL).await
}

pub async fn create_tables(db: &SqlitePool) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS messages
    (   
        UID INTEGER PRIMARY KEY,
        timestamp INTEGER,
        name VARCHAR(250),
        content VARCHAR(250)
    )
    ",
    )
    .execute(db)
    .await
}
