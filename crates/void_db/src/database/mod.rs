/*
 * Copyright (C) 2026 Shelkonogov Egor (Paradoxxa) <ghostoftranshumanist@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::{DbError, interfaces::void_database_repository::DatabaseRepository};
use async_trait::async_trait;
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
    /// ```ignore
    /// use void_entities::GlobalDB;
    /// async fn create_database(){
    ///     let db = GlobalDB::new().await;
    /// }
    /// ```
    pub fn new(db: Surreal<Db>) -> Self {
        Self { database: Some(db) }
    }
}

#[async_trait]
impl DatabaseRepository for GlobalDB {
    async fn create_or_connect_database(
        &self,
        namespace: &'static str,
        database_name: &'static str,
    ) -> Result<(), crate::DbError> {
        match &self.database {
            Some(db) => {
                db.use_ns(namespace)
                    .use_db(database_name)
                    .await
                    .map_err(|e| DbError::CreateError(e.to_string()))?;
                Ok(())
            }
            None => Err(DbError::OperationNotPermitted),
        }
    }

    async fn delete_database(
        &self,
        namespace: &'static str,
        database_name: &'static str,
    ) -> Result<(), DbError> {
        match &self.database {
            Some(db) => {
                let db_delete_query = String::from("REMOVE DATABASE ") + database_name + ";";
                let ns_delete_query = String::from("REMOVE NAMESPACE ") + namespace + ";";
                db.query(db_delete_query)
                    .await
                    .map_err(|e| DbError::DeleteError(e.to_string()))?
                    .check()
                    .map_err(|e| DbError::DeleteError(e.to_string()))?;
                db.query(ns_delete_query)
                    .await
                    .map_err(|e| DbError::DeleteError(e.to_string()))?
                    .check()
                    .map_err(|e| DbError::DeleteError(e.to_string()))?;
                Ok(())
            }
            None => Err(DbError::OperationNotPermitted),
        }
    }
    fn get_database(&self) -> Result<Surreal<Db>, DbError> {
        match &self.database {
            Some(db) => Ok(db.clone()),
            None => Err(DbError::OperationNotPermitted),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        DbError, database::GlobalDB, initialization::init_db,
        interfaces::void_database_repository::DatabaseRepository,
    };
    use serde::{Deserialize, Serialize};
    use serial_test::serial;
    use surrealdb::types::{RecordId, SurrealValue};
    #[derive(Serialize, Deserialize, SurrealValue)]
    struct TestStruct {
        name: &'static str,
    }
    #[derive(Debug, Deserialize, SurrealValue)]
    struct Record {
        #[allow(dead_code)]
        id: RecordId,
    }
    #[tokio::test]
    #[serial]
    async fn test_generate_table_and_namespace() {
        let db = GlobalDB::new(init_db(None).await);
        db.create_or_connect_database("test_ns", "test_db")
            .await
            .unwrap();
        if let Some(db) = db.database.as_ref() {
            let _: Result<Option<Record>, surrealdb::Error> = db
                .create("person")
                .content(TestStruct { name: "test" })
                .await;
        }
        db.delete_database("test_ns", "test_db").await.unwrap();
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_database_that_doesnt_exist() {
        let db = GlobalDB::new(init_db(None).await);
        let res = db.delete_database("something", "something").await;
        assert_eq!(
            Err(DbError::DeleteError(String::from(
                "Specify a namespace to use"
            ))),
            res
        );
    }
}
