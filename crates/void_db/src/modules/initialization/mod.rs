use surrealdb::{
    Surreal,
    engine::local::{Db, RocksDb},
};
use void_core::filesystem::config::get_config_folder;

/// # Database initialization
/// commonly you don't need to invoke this function, as soon as db starts when app is in loading
/// state.
/// But if you need some config to manage before database starts you could invoke this manually.
/// it will panic if called before config folder initialization
pub async fn init_db() -> Surreal<Db> {
    let db_path = get_config_folder(Some("db"));
    info!("Initialized DB");
    Surreal::new::<RocksDb>(db_path).await.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_creation() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(init_db());
    }
}
