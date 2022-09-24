use gloo::storage::Storage as LocalStorage;
use serde::{de::DeserializeOwned, Serialize};

pub struct Storage;

impl Storage {
    pub fn write<T: Serialize>(key: String, value: T) -> Result<(), String> {
        let ls = gloo::storage::LocalStorage::raw();

        match ls.set_item(
            key.as_str(),
            serde_json::to_string(&value).unwrap().as_str(),
        ) {
            Ok(_) => Result::Ok(()),
            Err(_) => Result::Err("Unable to write to local storage".to_string()),
        }
    }

    pub fn read<T: DeserializeOwned>(key: String) -> Option<T> {
        let ls = gloo::storage::LocalStorage::raw();

        let result = ls.get_item(key.as_str()).unwrap_or(None);

        let s: String;
        let ret: Option<T> = match result {
            Some(string) => {
                s = string.clone();
                serde_json::from_str(&s).unwrap_or(None)
            }
            _ => None,
        };

        return ret;
    }
}
