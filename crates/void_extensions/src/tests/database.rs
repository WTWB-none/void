/*
 * Copyright (C) 2026 Shelkonogov Egor (Paradoxxa) <ghostoftranshumanist@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#[cfg(test)]
mod tests {
    use tempfile::tempdir;
    use void_db::{
        database::GlobalDB,
        initialization::init_db,
        interfaces::{
            void_database_repository::DatabaseRepository,
            void_entity_manipulator::EntityController,
            void_manifest_manipulator::ManifestController,
        },
    };

    use crate::db_connector::{
        plugin_entity::{Plugin, PluginEntityController},
        plugin_manifest::VoidPluginManifestMember,
    };

    #[tokio::test]
    async fn test_entity_creation() {
        let temp_dir = tempdir().unwrap();
        let tempdir_path = temp_dir.path().to_str().unwrap();
        let global_db = GlobalDB::new(init_db(Some(tempdir_path.to_string())).await);
        assert!(
            global_db
                .create_or_connect_database("void_db", "void_db")
                .await
                .is_ok()
        );
        let str = r#"
                    [
                        {
                            "name": "test_plugin",
                            "author": "test_author",
                            "version": "0.0.1"
                        }
                    ]
            "#;
        let items = VoidPluginManifestMember::construct_from_json(str.to_string()).unwrap();
        let plugin_controller = PluginEntityController;

        let assert_value = plugin_controller
            .create_entity(&global_db, items[0].clone(), "temp".to_string())
            .await
            .unwrap();
        let res = plugin_controller
            .get_list_of_entities(&global_db, Some("WHERE name = 'test_plugin'"))
            .await
            .unwrap();
        assert_eq!(assert_value, res[0]);

        assert!(
            global_db
                .delete_database("void_db", "void_db")
                .await
                .is_ok()
        );
        drop(global_db);
    }

    #[tokio::test]
    async fn test_entity_deletion() {
        let temp_dir = tempdir().unwrap();
        let tempdir_path = temp_dir.path().to_str().unwrap();
        let global_db = GlobalDB::new(init_db(Some(tempdir_path.to_string())).await);
        assert!(
            global_db
                .create_or_connect_database("void_db", "void_db")
                .await
                .is_ok()
        );
        let str = r#"
                    [
                        {
                            "name": "test_plugin",
                            "author": "test_author",
                            "version": "0.0.1"
                        }
                    ]
            "#;
        let plugin_controller = PluginEntityController;
        let items = VoidPluginManifestMember::construct_from_json(str.to_string()).unwrap();
        assert!(
            plugin_controller
                .create_entity(&global_db, items[0].clone(), "temp".to_string())
                .await
                .is_ok()
        );
        assert!(
            plugin_controller
                .delete_entity(&global_db, "WHERE name = 'test_plugin'")
                .await
                .is_ok()
        );
        let res = plugin_controller
            .get_list_of_entities(&global_db, None)
            .await
            .unwrap();
        assert!(res.is_empty());
        assert!(
            global_db
                .delete_database("void_db", "void_db")
                .await
                .is_ok()
        );
        drop(global_db);
    }

    #[tokio::test]
    async fn test_entity_update() {
        let temp_dir = tempdir().unwrap();
        let tempdir_path = temp_dir.path().to_str().unwrap();
        let global_db = GlobalDB::new(init_db(Some(tempdir_path.to_string())).await);
        global_db
            .create_or_connect_database("void_db", "void_db")
            .await
            .unwrap();
        let items = vec![
            VoidPluginManifestMember::Name("test_plugin".to_string()),
            VoidPluginManifestMember::Author("test_author".to_string()),
            VoidPluginManifestMember::Version("0.0.1".to_string()),
        ];
        let plugin_controller = PluginEntityController;
        let mut update_value = plugin_controller
            .create_entity(&global_db, items, "temp".to_string())
            .await
            .unwrap();
        update_value.set_author("paradoxxa".to_string());
        let new_res = plugin_controller
            .update_entity(&global_db, vec![update_value])
            .await
            .unwrap();
        let mut assert_value = Plugin::default();
        assert_value.set_name("test_plugin".to_string());
        assert_value.set_author("paradoxxa".to_string());
        assert_value.set_version("0.0.1".to_string());
        assert_value.set_repo_link("temp".to_string());
        assert_eq!(assert_value, new_res[0].clone().unwrap());
        assert!(
            global_db
                .delete_database("void_db", "void_db")
                .await
                .is_ok()
        );
        drop(global_db);
    }

    #[tokio::test]
    async fn test_multiple_entity_creation() {
        let temp_dir = tempdir().unwrap();
        let tempdir_path = temp_dir.path().to_str().unwrap();
        let global_db = GlobalDB::new(init_db(Some(tempdir_path.to_string())).await);
        assert!(
            global_db
                .create_or_connect_database("void_db", "void_db")
                .await
                .is_ok()
        );
        let plugin_entity_controller = PluginEntityController;
        for i in 1..=2 {
            let temp_vec = vec![
                VoidPluginManifestMember::Name(format!("test_plugin{}", i)),
                VoidPluginManifestMember::Author(format!("test_author{}", i)),
                VoidPluginManifestMember::Version(format!("0.0.{}", i)),
            ];
            assert!(
                plugin_entity_controller
                    .create_entity(&global_db, temp_vec, "temp".to_string())
                    .await
                    .is_ok()
            );
        }
        let plugins = plugin_entity_controller
            .get_list_of_entities(&global_db, None)
            .await
            .unwrap();
        assert!(plugins.len() == 2);
        assert!(
            global_db
                .delete_database("void_db", "void_db")
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn test_multiple_entity_deletion() {
        let temp_dir = tempdir().unwrap();
        let tempdir_path = temp_dir.path().to_str().unwrap();
        let global_db = GlobalDB::new(init_db(Some(tempdir_path.to_string())).await);
        assert!(
            global_db
                .create_or_connect_database("void_db", "void_db")
                .await
                .is_ok()
        );
        let plugin_entity_controller = PluginEntityController;
        for i in 1..=2 {
            let temp_vec = vec![
                VoidPluginManifestMember::Name(format!("test_plugin{}", i)),
                VoidPluginManifestMember::Author(format!("test_author{}", i)),
                VoidPluginManifestMember::Version(format!("0.0.{}", i)),
            ];
            assert!(
                plugin_entity_controller
                    .create_entity(&global_db, temp_vec, "temp".to_string())
                    .await
                    .is_ok()
            );
        }
        assert!(
            plugin_entity_controller
                .delete_all_entities(&global_db)
                .await
                .is_ok()
        );
        let plugins = plugin_entity_controller
            .get_list_of_entities(&global_db, None)
            .await
            .unwrap();
        assert!(plugins.is_empty());
        assert!(
            global_db
                .delete_database("void_db", "void_db")
                .await
                .is_ok()
        )
    }

    #[tokio::test]
    async fn test_conditional_delete() {
        let temp_dir = tempdir().unwrap();
        let tempdir_path = temp_dir.path().to_str().unwrap();
        let global_db = GlobalDB::new(init_db(Some(tempdir_path.to_string())).await);
        assert!(
            global_db
                .create_or_connect_database("void_db", "void_db")
                .await
                .is_ok()
        );
        let plugin_entity_controller = PluginEntityController;
        for i in 1..=2 {
            let temp_vec = if i == 1 {
                vec![
                    VoidPluginManifestMember::Name(format!("test_plugin{}", i)),
                    VoidPluginManifestMember::Author(format!("test_author{}", i)),
                    VoidPluginManifestMember::Version(format!("0.0.{}", i)),
                ]
            } else {
                vec![
                    VoidPluginManifestMember::Name(format!("test_plugin{}", i)),
                    VoidPluginManifestMember::Author("paradoxxa".to_string()),
                    VoidPluginManifestMember::Version(format!("0.0.{}", i)),
                ]
            };
            assert!(
                plugin_entity_controller
                    .create_entity(&global_db, temp_vec, "temp".to_string())
                    .await
                    .is_ok()
            );
        }

        assert!(
            plugin_entity_controller
                .delete_entity(&global_db, "WHERE author = 'paradoxxa'")
                .await
                .is_ok()
        );
        let plugins = plugin_entity_controller
            .get_list_of_entities(&global_db, None)
            .await
            .unwrap();
        assert!(plugins.len() == 1);
        assert!(
            global_db
                .delete_database("void_db", "void_db")
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn test_conditional_get() {
        let temp_dir = tempdir().unwrap();
        let tempdir_path = temp_dir.path().to_str().unwrap();
        let global_db = GlobalDB::new(init_db(Some(tempdir_path.to_string())).await);
        assert!(
            global_db
                .create_or_connect_database("void_db", "void_db")
                .await
                .is_ok()
        );
        let plugin_entity_controller = PluginEntityController;
        for i in 1..=2 {
            let temp_vec = if i == 1 {
                vec![
                    VoidPluginManifestMember::Name(format!("test_plugin{}", i)),
                    VoidPluginManifestMember::Author(format!("test_author{}", i)),
                    VoidPluginManifestMember::Version(format!("0.0.{}", i)),
                ]
            } else {
                vec![
                    VoidPluginManifestMember::Name(format!("test_plugin{}", i)),
                    VoidPluginManifestMember::Author("paradoxxa".to_string()),
                    VoidPluginManifestMember::Version(format!("0.0.{}", i)),
                ]
            };
            assert!(
                plugin_entity_controller
                    .create_entity(&global_db, temp_vec, "temp".to_string())
                    .await
                    .is_ok()
            );
        }

        let plugins = plugin_entity_controller
            .get_list_of_entities(&global_db, None)
            .await
            .unwrap();
        let conditional_plugins = plugin_entity_controller
            .get_list_of_entities(&global_db, Some("WHERE author = 'paradoxxa'"))
            .await
            .unwrap();
        assert_ne!(plugins.len(), conditional_plugins.len());
        assert!(
            global_db
                .delete_database("void_db", "void_db")
                .await
                .is_ok()
        );
    }

    #[test]
    fn test_deserialization() {
        let str = r#"
                    [
                        {
                            "name": "test_plugin",
                            "author": "test_author",
                            "version": "0.0.1"
                        }
                    ]
            "#;
        let parser = VoidPluginManifestMember::construct_from_json(str.to_string());
        let parsed = VoidPluginManifestMember::sort(parser.unwrap()[0].clone());
        assert_eq!(
            vec![
                VoidPluginManifestMember::Name("test_plugin".to_string()),
                VoidPluginManifestMember::Author("test_author".to_string()),
                VoidPluginManifestMember::Version("0.0.1".to_string())
            ],
            parsed
        )
    }

    #[test]
    fn test_multiple_deserialization() {
        let str = r#"
            [
                {
                    "name": "test_plugin1",
                    "author": "test_author1",
                    "version": "0.0.1"
                },
                {
                    "name": "test_plugin2",
                    "author": "test_author2",
                    "version": "0.0.2"
                }
            ]
            "#;
        let parsed = VoidPluginManifestMember::construct_from_json(str.to_string());
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap().clone();
        for (i, plugin) in parsed.iter().enumerate() {
            let temp = VoidPluginManifestMember::sort(plugin.clone());
            assert_eq!(
                vec![
                    VoidPluginManifestMember::Name(format!("test_plugin{}", i + 1)),
                    VoidPluginManifestMember::Author(format!("test_author{}", i + 1)),
                    VoidPluginManifestMember::Version(format!("0.0.{}", i + 1))
                ],
                temp
            );
        }
    }

    #[test]
    #[should_panic]
    fn test_deserialization_failure() {
        let str = r#"
            [
                {
                    "name": "test_plugin1",
                    "author": "test_author1",
                    "version": "0.0.1"
                },
                {
                    "name": "test_plugin2",
                    "author": "test_author2",
                }
            "#;
        let parsed = VoidPluginManifestMember::construct_from_json(str.to_string());
        assert!(parsed.is_ok());
    }
}
