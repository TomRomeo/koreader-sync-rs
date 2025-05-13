use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use crate::api::syncs::update_progress::DocumentProgress;
use crate::db::Database;

pub struct PostgresDB {
    connection_string: String,
    pool: PgPool
}

impl PostgresDB {
    pub async fn new(connection_string: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_string).await?;
        Ok(PostgresDB {
            connection_string: connection_string.to_string(),
            pool: pool,
        })
    }
    
    pub async fn migrate(&self) -> Result<(), sqlx::Error> {
        sqlx::migrate!().run(&self.pool).await?;
        Ok(())
    }
}

#[async_trait]
impl Database for PostgresDB {
    async fn get_progress(&self, username: &str, id: &str) -> Result<DocumentProgress, sqlx::Error> {
        // Simulate a database query
        println!("get progress for user: {}, document: {}", username, id);
        let row = sqlx::query_as::<_, DocumentProgress>("SELECT * FROM Syncs WHERE \"user\" = $1 AND document = $2")
            .bind(username)
            .bind(id)
            .fetch_one(&self.pool).await?;
        println!("{:?}", row);
        Ok(row)
    }
    async fn update_progress(&self, username: &str, document: &str, percentage: &str, progress: &str, device: &str, device_id: &str) -> Result<(), sqlx::Error> {
        // Simulate a database query
        sqlx::query("INSERT INTO Syncs (\"user\", document, percentage, progress, device, device_id) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (document, \"user\") DO UPDATE SET percentage = $3, progress = $4, device = $5, device_id = $6, timestamp = now();")
            .bind(username)
            .bind(document)
            .bind(percentage)
            .bind(progress)
            .bind(device)
            .bind(device_id)
            .execute(&self.pool).await?;
        Ok(())
    }
    async fn create_user(&self, username: &str, password: &str) -> Result<(), sqlx::Error> {
        // Simulate a database query
        sqlx::query("INSERT INTO Users (username, password) VALUES ($1, $2)")
            .bind(username)
            .bind(password)
            .execute(&self.pool).await?;
        Ok(())
    }
    async fn validate_password(&self, username: &str, password: &str) -> Result<bool, sqlx::Error> {
        // Simulate a database query
        let _: () = sqlx::query_as("SELECT * FROM Users WHERE username = $1 AND password = $2")
            .bind(username)
            .bind(password)
            .fetch_one(&self.pool).await?;
        Ok(true)
    }
}