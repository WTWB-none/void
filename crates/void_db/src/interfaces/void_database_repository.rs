/*
 * Copyright (C) 2026 Shelkonogov Egor (Paradoxxa) <ghostoftranshumanist@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::DbError;
use async_trait::async_trait;
use surrealdb::{Surreal, engine::local::Db};

/// # Trait for managing databases in surrealDB under Void.
/// Requires implementation for all functions.
#[async_trait]
pub trait DatabaseRepository {
    /// Basically creates database and namespace inside surrealDB for later usage;
    /// You don't need to use any sql for this. Simply use use_ns() and use_db() surrealDB
    /// functions.
    async fn create_or_connect_database(
        &self,
        namespace: &'static str,
        database_name: &'static str,
    ) -> Result<(), DbError>;

    /// For deleting namespace and database you need to write some basic sql:
    /// REMOVE DATABASE IF EXISTS smth;
    /// REMOVE NAMESPACE IF EXISTS smth;
    /// for executing this query you could use query() function from surrealDB;
    async fn delete_database(
        &self,
        namespace: &'static str,
        database_name: &'static str,
    ) -> Result<(), DbError>;

    fn get_database(&self) -> Result<Surreal<Db>, DbError>;
}
