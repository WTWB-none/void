use surrealdb::{Surreal, engine::local::Db};

/// # Global handler for database
/// initializes on startup. Simple struct to hold database state in gpui memory.
#[allow(dead_code)]
pub struct GlobalDB {
    database: Option<Surreal<Db>>,
}

impl GlobalDB {
    /// database constructor. Needs awaiting.
    /// returns GlobalDB
    /// ```
    /// use void_entities::GlobalDB;
    /// async fn create_database(){
    ///     let db = GlobalDB::new().await;
    /// }
    /// ```
    pub fn new(db: Surreal<Db>) -> Self {
        Self { database: Some(db) }
    }
}
