/*
 * Copyright (C) 2026 Shelkonogov Egor (Paradoxxa) <ghostoftranshumanist@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use async_trait::async_trait;
use serde::Serialize;
use surrealdb::types::{RecordId, SurrealValue, ToSql};
use void_db::{
    DbError,
    interfaces::{
        void_database_repository::DatabaseRepository, void_entity_manipulator::EntityController,
    },
};

use crate::db_connector::plugin_manifest::VoidPluginManifestMember;

pub struct PluginEntityController;

#[derive(Default, Debug, PartialEq, Clone, Serialize, SurrealValue)]
pub struct Plugin {
    name: Option<String>,
    author: Option<String>,
    version: Option<String>,
    repo_link: Option<String>,
}

#[derive(Debug, Serialize, SurrealValue)]
pub struct PluginId {
    id: RecordId,
}

impl Plugin {
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn set_author(&mut self, author: String) {
        self.author = Some(author);
    }

    pub fn set_version(&mut self, version: String) {
        self.version = Some(version);
    }

    pub fn set_repo_link(&mut self, link: String) {
        self.repo_link = Some(link);
    }
}

#[async_trait]
impl EntityController for PluginEntityController {
    type Entity = Plugin;
    type EntityField = VoidPluginManifestMember;
    async fn create_entity(
        &self,
        global_db: &void_db::database::GlobalDB,
        list_of_items: Vec<Self::EntityField>,
        source: String,
    ) -> Result<Self::Entity, void_db::DbError> {
        let db = global_db.get_database()?;
        let mut entity = Plugin::default();
        for field in list_of_items {
            match field {
                VoidPluginManifestMember::Name(name) => entity.set_name(name),
                VoidPluginManifestMember::Author(author) => entity.set_author(author),
                VoidPluginManifestMember::Version(version) => entity.set_version(version),
            }
            entity.set_repo_link(source.clone());
        }
        if entity.name.is_some() && entity.author.is_some() && entity.version.is_some() {
            let record: Option<Plugin> = db
                .create("plugins")
                .content(entity)
                .await
                .map_err(|e| DbError::CreateError(e.to_string()))?;
            Ok(record.unwrap())
        } else {
            Err(DbError::CreateError(
                "not enough data for entity creation".to_string(),
            ))
        }
    }

    async fn get_list_of_entities(
        &self,
        global_db: &void_db::database::GlobalDB,
        condition: Option<&'static str>,
    ) -> Result<Vec<Self::Entity>, void_db::DbError> {
        let db = global_db.get_database()?;
        match condition {
            Some(con) => {
                let records: Vec<Plugin> = db
                    .query(format!("SELECT * FROM plugins {};", con).as_str())
                    .await
                    .map_err(|e| DbError::ReadError(e.to_string()))?
                    .take(0)
                    .map_err(|e| DbError::ReadError(e.to_string()))?;
                return Ok(records);
            }
            None => {
                let records: Vec<Plugin> = db
                    .select("plugins")
                    .await
                    .map_err(|e| DbError::ReadError(e.to_string()))?;
                return Ok(records);
            }
        }
    }

    async fn update_entity(
        &self,
        global_db: &void_db::database::GlobalDB,
        items: Vec<Self::Entity>,
    ) -> Result<Vec<Option<Self::Entity>>, void_db::DbError> {
        let db = global_db.get_database()?;
        let mut records = Vec::<Option<Self::Entity>>::new();
        for plugin in items {
            let id = self
                .get_entity_id(global_db, plugin.name.clone().unwrap())
                .await?
                .key
                .to_sql()
                .to_string();
            records.push(
                db.update(("plugins", id.as_str()))
                    .content::<Plugin>(plugin)
                    .await
                    .map_err(|e| DbError::UpdateError(e.to_string()))?,
            );
        }
        Ok(records)
    }

    async fn delete_entity(
        &self,
        global_db: &void_db::database::GlobalDB,
        filter_condition: &'static str,
    ) -> Result<(), void_db::DbError> {
        let db = global_db.get_database()?;
        db.query(format!("DELETE plugins {};", filter_condition).as_str())
            .await
            .map_err(|e| DbError::DeleteError(e.to_string()))?;
        Ok(())
    }

    async fn delete_all_entities(
        &self,
        global_db: &void_db::database::GlobalDB,
    ) -> Result<(), void_db::DbError> {
        let db = global_db.get_database()?;
        db.delete::<Vec<Self::Entity>>("plugins")
            .await
            .map_err(|e| DbError::DeleteError(e.to_string()))?;
        Ok(())
    }

    async fn get_entity_id(
        &self,
        global_db: &void_db::database::GlobalDB,
        name: String,
    ) -> Result<RecordId, DbError> {
        let db = global_db.get_database()?;
        let query = format!("SELECT id FROM plugins WHERE name = '{}'", name);
        let res: Vec<PluginId> = db
            .query(query.as_str())
            .await
            .map_err(|e| DbError::ReadError(e.to_string()))?
            .take(0)
            .map_err(|e| DbError::ReadError(e.to_string()))?;
        Ok(res[0].id.clone())
    }
}
