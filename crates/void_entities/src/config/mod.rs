use std::path::PathBuf;

#[derive(Default)]
pub struct GlobalConfig {
    scope: Option<PathBuf>,
}

impl GlobalConfig {
    pub fn change_scope(&mut self, link: PathBuf) {
        self.scope = Some(link);
    }
    pub fn get_scope(&mut self) -> Option<PathBuf> {
        self.scope.clone()
    }
}
