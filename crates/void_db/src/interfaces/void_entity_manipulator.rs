/*
 * Copyright (C) 2026 Shelkonogov Egor (Paradoxxa) <ghostoftranshumanist@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, SurrealValue};

use crate::{database::GlobalDB, errors::DbError};

/// # trait that implementation is required to manipulate data inside built in local surrealDB.
/// All functions in error case should return DbError
#[async_trait]
pub trait EntityController {
    type EntityField: Serialize + Deserialize<'static>;
    type Entity: Default + Serialize + SurrealValue;
    /// # One of the important CRUD operations: Create
    /// this function consumes vec of structure fields (in ideal world thay all should be from
    /// one enum for better update handling), writes new record to database (maybe through
    /// transaction), and returns created struct just in case;
    /// also, to mention, if table not exists this method creates one
    async fn create_entity(
        &self,
        global_db: &GlobalDB,
        list_of_items: Vec<Self::EntityField>,
        source: String,
    ) -> Result<Self::Entity, DbError>;

    /// # Second part of CRUD operations: Read
    /// this functions returns the entire universe from your table. Maybe you should use it in
    /// separate thread to avoid blocking your entire application. returns vec of records from
    /// your table. If you want to filter list of returning entities you could use condition
    /// parameter. In case you don't - simply use None instead of condition parameter
    async fn get_list_of_entities(
        &self,
        global_db: &GlobalDB,
        condition: Option<&'static str>,
    ) -> Result<Vec<Self::Entity>, DbError>;

    /// # Third part of CRUD operations: Update
    /// this function updates single record. Returns Updated
    /// record. Consumes vec of updated fields
    async fn update_entity(
        &self,
        global_db: &GlobalDB,
        items: Vec<Self::Entity>,
    ) -> Result<Vec<Option<Self::Entity>>, DbError>;

    /// # And the last part of CRUD operations: Delete.
    /// this function delets one entity selected by filter condition. Consumes Filter condition.
    /// Returns empty result or DbError
    async fn delete_entity(
        &self,
        global_db: &GlobalDB,
        filter_condition: &'static str,
    ) -> Result<(), DbError>;

    /// # Also the last part of CRUD operations: Delete.
    /// this function completely deletes all values from table in void storage.
    /// Require table name.
    /// If anything breaks will return DbError
    async fn delete_all_entities(&self, global_db: &GlobalDB) -> Result<(), DbError>;

    /// # Helper function. Returns entity ID. Should be helpful for some other operatons such as update.
    async fn get_entity_id(&self, global_db: &GlobalDB, name: String) -> Result<RecordId, DbError>;
}
