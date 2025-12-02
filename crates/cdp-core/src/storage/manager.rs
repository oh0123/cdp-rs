use crate::error::Result;
use crate::page::Page;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Storage item representing a key-value pair
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StorageItem {
    pub key: String,
    pub value: String,
}

/// Storage type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageType {
    Local,
    Session,
}

impl StorageType {
    fn as_str(&self) -> &'static str {
        match self {
            StorageType::Local => "localStorage",
            StorageType::Session => "sessionStorage",
        }
    }
}

/// Storage manager trait for localStorage and sessionStorage operations
#[async_trait]
pub trait StorageManager {
    /// Gets all items from the specified storage type
    async fn get_storage_items(
        self: &Arc<Self>,
        storage_type: StorageType,
    ) -> Result<Vec<StorageItem>>;

    /// Gets a specific item from storage by key
    async fn get_storage_item(
        self: &Arc<Self>,
        storage_type: StorageType,
        key: &str,
    ) -> Result<Option<String>>;

    /// Sets an item in storage
    async fn set_storage_item(
        self: &Arc<Self>,
        storage_type: StorageType,
        key: &str,
        value: &str,
    ) -> Result<()>;

    /// Removes an item from storage by key
    async fn remove_storage_item(
        self: &Arc<Self>,
        storage_type: StorageType,
        key: &str,
    ) -> Result<()>;

    /// Clears all items from the specified storage type
    async fn clear_storage(self: &Arc<Self>, storage_type: StorageType) -> Result<()>;

    /// Gets the number of items in storage
    async fn get_storage_length(self: &Arc<Self>, storage_type: StorageType) -> Result<usize>;
}

#[async_trait]
impl StorageManager for Page {
    async fn get_storage_items(
        self: &Arc<Self>,
        storage_type: StorageType,
    ) -> Result<Vec<StorageItem>> {
        let storage_name = storage_type.as_str();
        let script = format!(
            r#"
            (() => {{
                const storage = {};
                const items = [];
                for (let i = 0; i < storage.length; i++) {{
                    const key = storage.key(i);
                    if (key !== null) {{
                        items.push({{ key: key, value: storage.getItem(key) }});
                    }}
                }}
                return items;
            }})()
            "#,
            storage_name
        );

        let main_frame = self.main_frame().await?;
        let result = main_frame.evaluate(&script).await?;

        let items: Vec<StorageItem> = serde_json::from_value(result)?;
        Ok(items)
    }

    async fn get_storage_item(
        self: &Arc<Self>,
        storage_type: StorageType,
        key: &str,
    ) -> Result<Option<String>> {
        let storage_name = storage_type.as_str();
        let script = format!(
            r#"
            (() => {{
                const storage = {};
                return storage.getItem({});
            }})()
            "#,
            storage_name,
            serde_json::to_string(key)?
        );

        let main_frame = self.main_frame().await?;
        let result = main_frame.evaluate(&script).await?;

        if result.is_null() {
            Ok(None)
        } else {
            Ok(Some(result.as_str().unwrap_or("").to_string()))
        }
    }

    async fn set_storage_item(
        self: &Arc<Self>,
        storage_type: StorageType,
        key: &str,
        value: &str,
    ) -> Result<()> {
        let storage_name = storage_type.as_str();
        let script = format!(
            r#"
            (() => {{
                const storage = {};
                storage.setItem({}, {});
            }})()
            "#,
            storage_name,
            serde_json::to_string(key)?,
            serde_json::to_string(value)?
        );

        let main_frame = self.main_frame().await?;
        main_frame.evaluate(&script).await?;
        Ok(())
    }

    async fn remove_storage_item(
        self: &Arc<Self>,
        storage_type: StorageType,
        key: &str,
    ) -> Result<()> {
        let storage_name = storage_type.as_str();
        let script = format!(
            r#"
            (() => {{
                const storage = {};
                storage.removeItem({});
            }})()
            "#,
            storage_name,
            serde_json::to_string(key)?
        );

        let main_frame = self.main_frame().await?;
        main_frame.evaluate(&script).await?;
        Ok(())
    }

    async fn clear_storage(self: &Arc<Self>, storage_type: StorageType) -> Result<()> {
        let storage_name = storage_type.as_str();
        let script = format!(
            r#"
            (() => {{
                const storage = {};
                storage.clear();
            }})()
            "#,
            storage_name
        );

        let main_frame = self.main_frame().await?;
        main_frame.evaluate(&script).await?;
        Ok(())
    }

    async fn get_storage_length(self: &Arc<Self>, storage_type: StorageType) -> Result<usize> {
        let storage_name = storage_type.as_str();
        let script = format!(
            r#"
            (() => {{
                const storage = {};
                return storage.length;
            }})()
            "#,
            storage_name
        );

        let main_frame = self.main_frame().await?;
        let result = main_frame.evaluate(&script).await?;

        let length = result.as_u64().unwrap_or(0) as usize;
        Ok(length)
    }
}

/// Helper methods for convenient access to localStorage
#[async_trait]
pub trait LocalStorageExt {
    async fn get_local_storage(self: &Arc<Self>) -> Result<HashMap<String, String>>;
    async fn get_local_storage_item(self: &Arc<Self>, key: &str) -> Result<Option<String>>;
    async fn set_local_storage_item(self: &Arc<Self>, key: &str, value: &str) -> Result<()>;
    async fn remove_local_storage_item(self: &Arc<Self>, key: &str) -> Result<()>;
    async fn clear_local_storage(self: &Arc<Self>) -> Result<()>;
}

#[async_trait]
impl LocalStorageExt for Page {
    async fn get_local_storage(self: &Arc<Self>) -> Result<HashMap<String, String>> {
        let items = self.get_storage_items(StorageType::Local).await?;
        Ok(items
            .into_iter()
            .map(|item| (item.key, item.value))
            .collect())
    }

    async fn get_local_storage_item(self: &Arc<Self>, key: &str) -> Result<Option<String>> {
        self.get_storage_item(StorageType::Local, key).await
    }

    async fn set_local_storage_item(self: &Arc<Self>, key: &str, value: &str) -> Result<()> {
        self.set_storage_item(StorageType::Local, key, value).await
    }

    async fn remove_local_storage_item(self: &Arc<Self>, key: &str) -> Result<()> {
        self.remove_storage_item(StorageType::Local, key).await
    }

    async fn clear_local_storage(self: &Arc<Self>) -> Result<()> {
        self.clear_storage(StorageType::Local).await
    }
}

/// Helper methods for convenient access to sessionStorage
#[async_trait]
pub trait SessionStorageExt {
    async fn get_session_storage(self: &Arc<Self>) -> Result<HashMap<String, String>>;
    async fn get_session_storage_item(self: &Arc<Self>, key: &str) -> Result<Option<String>>;
    async fn set_session_storage_item(self: &Arc<Self>, key: &str, value: &str) -> Result<()>;
    async fn remove_session_storage_item(self: &Arc<Self>, key: &str) -> Result<()>;
    async fn clear_session_storage(self: &Arc<Self>) -> Result<()>;
}

#[async_trait]
impl SessionStorageExt for Page {
    async fn get_session_storage(self: &Arc<Self>) -> Result<HashMap<String, String>> {
        let items = self.get_storage_items(StorageType::Session).await?;
        Ok(items
            .into_iter()
            .map(|item| (item.key, item.value))
            .collect())
    }

    async fn get_session_storage_item(self: &Arc<Self>, key: &str) -> Result<Option<String>> {
        self.get_storage_item(StorageType::Session, key).await
    }

    async fn set_session_storage_item(self: &Arc<Self>, key: &str, value: &str) -> Result<()> {
        self.set_storage_item(StorageType::Session, key, value)
            .await
    }

    async fn remove_session_storage_item(self: &Arc<Self>, key: &str) -> Result<()> {
        self.remove_storage_item(StorageType::Session, key).await
    }

    async fn clear_session_storage(self: &Arc<Self>) -> Result<()> {
        self.clear_storage(StorageType::Session).await
    }
}
