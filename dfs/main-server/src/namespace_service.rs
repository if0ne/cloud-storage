use crate::namespace::Namespace;
use std::ops::Deref;
use std::path::Path;
use tokio::sync::RwLock;

pub struct NamespaceServiceImpl {
    namespace: RwLock<Namespace>,
}

impl NamespaceServiceImpl {
    pub fn new() -> Self {
        Self {
            namespace: RwLock::new(Namespace::new()),
        }
    }

    pub async fn create_small_file(&self, path: impl AsRef<Path>) {
        let mut writer = self.namespace.write().await;
        writer.create_small_file(path);
        tracing::debug!("{:?}", writer.deref());
    }
}
