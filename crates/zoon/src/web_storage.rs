use crate::*;
use once_cell::race::OnceBox;
use web_sys::Storage;

pub type Result<T> = std::result::Result<T, Error>;

// ------ local_storage ------

pub fn local_storage() -> &'static LocalStorage {
    static LOCAL_STORAGE: OnceBox<SendWrapper<LocalStorage>> = OnceBox::new();
    LOCAL_STORAGE.get_or_init(|| {
        let storage = LocalStorage::try_new().unwrap_throw();
        Box::new(SendWrapper::new(storage))
    })
}

// ------ session_storage ------

pub fn session_storage() -> &'static SessionStorage {
    static SESSION_STORAGE: OnceBox<SendWrapper<SessionStorage>> = OnceBox::new();
    SESSION_STORAGE.get_or_init(|| {
        let storage = SessionStorage::try_new().unwrap_throw();
        Box::new(SendWrapper::new(storage))
    })
}

// ------ Error ------

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("the platform does not support the required WebStorage")]
    StorageNotFoundError,
    #[error("cannot get access to the required WebStorage")]
    GetStorageError(JsValue),
    #[error("cannot insert or update the given key-value pair (error: `{0:?}`)")]
    InsertError(JsValue),
    // #[error("(de)serialization failed (error: `{0}`)")]
    // SerdeError(serde_lite::Error),  // for serde_lite
    // SerdeError(serde::Error),
    #[error("(de)serialization to JSON failed (error: `{0}`)")]
    SerdeJsonError(serde_json::Error),
}

// ------ LocalStorage ------

/// Local Storage  maintains a separate storage area for each given origin
/// that persists even when the browser is closed and reopened.
///
/// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)
pub struct LocalStorage(Storage);

impl WebStorage for LocalStorage {
    fn try_new() -> Result<Self> {
        let storage = window()
            .local_storage()
            .map_err(Error::GetStorageError)?
            .ok_or(Error::StorageNotFoundError);
        Ok(Self(storage?))
    }

    fn storage(&self) -> &Storage {
        &self.0
    }
}

// ------ SessionStorage ------

/// - Session Storage maintains a separate storage area for each given origin
/// that's available for the duration of the page session
/// (as long as the browser is open, including page reloads and restores).
///
/// - Opening multiple tabs/windows with the same URL creates sessionStorage for each tab/window.
///
/// - Data stored in sessionStorage is specific to the protocol of the page.
/// In other words, _`http://example.com`_ will have separate storage than _`https://example.com`_.
///
/// - Storage limit is larger than a cookie (at most 5MB).
///
/// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage)
pub struct SessionStorage(Storage);

impl WebStorage for SessionStorage {
    fn try_new() -> Result<Self> {
        let storage = window()
            .local_storage()
            .map_err(Error::GetStorageError)?
            .ok_or(Error::StorageNotFoundError);
        Ok(Self(storage?))
    }

    fn storage(&self) -> &Storage {
        &self.0
    }
}

// ------ WebStorage ------

/// Web Storage API.
///
/// `LocalStorage` and `SessionStorage` implement this trait.
///
/// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Web_Storage_API)
pub trait WebStorage: Sized {
    /// Creates a new instance.
    ///
    /// # Errors
    ///
    /// Returns error if we cannot get access to the storage - security errors,
    /// browser does not have given storage, user denied access for the current origin, etc.
    ///
    /// - [MDN ref for Local Storage](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)
    /// - [MDN ref for Session Storage](https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage)
    fn try_new() -> Result<Self>;

    /// Get the inner `web_sys::Storage` instance.
    ///
    /// This method is used internally by other methods.
    fn storage(&self) -> &Storage;

    /// Clear all data in the storage.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Storage/clear)
    fn clear(&self) {
        self.storage().clear().unwrap_throw()
    }

    /// Get the number of stored data items.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Storage/length)
    fn len(&self) -> u32 {
        self.storage().length().unwrap_throw()
    }

    /// Returns the key in the given position.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Storage/key)
    fn key(&self, index: u32) -> Option<String> {
        self.storage().key(index).unwrap_throw()
    }

    /// Removes a key.
    ///
    /// If there is no item associated with the given key, this method will do nothing.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Storage/removeItem)
    fn remove(&self, key: impl AsRef<str>) {
        self.storage().remove_item(key.as_ref()).unwrap_throw()
    }

    /// Returns a deserialized value corresponding to the key.
    ///
    /// # Errors
    ///
    /// Returns error when deserialization fails.
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Storage/getItem)
    fn get<T: DeserializeOwned>(&self, key: &str) -> Option<Result<T>> {
        let value = self.storage().get_item(key).unwrap_throw()?;

        fn deserialize<T: DeserializeOwned>(value: &str) -> Result<T> {
            Ok(serde_json::from_str(&value).map_err(Error::SerdeJsonError)?)
        }
        Some(deserialize(&value))
    }

    // serde_lite
    // fn get<T: Deserialize>(&self, key: &str) -> Option<Result<T>> {
    //     let value = self.storage().get_item(key).unwrap_throw()?;

    //     fn deserialize<T: Deserialize>(value: &str) -> Result<T> {
    //         let value = serde_json::from_str(&value).map_err(Error::SerdeJsonError)?;
    //         T::deserialize(&value).map_err(Error::SerdeError)
    //     }
    //     Some(deserialize(&value))
    // }

    /// Insert a key-value pair. The value will be serialized.
    ///
    /// If the key already exists, the value will be updated.
    ///
    /// # Errors
    ///
    /// Returns error if we cannot serialize the value or insert/update the pair.
    ///
    /// The function `web_sys::Storage::set_item` is used under the hood.
    /// A related warning from MDN docs:
    ///
    /// "setItem() may throw an exception if the storage is full.
    /// Particularly, in Mobile Safari (since iOS 5) it always throws when the user enters private mode.
    /// (Safari sets the quota to 0 bytes in private mode, unlike other browsers,
    /// which allow storage in private mode using separate data containers.)
    /// Hence developers should make sure to always catch possible exceptions from setItem()."
    ///
    /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/API/Storage/setItem)
    fn insert<T: Serialize + ?Sized>(&self, key: &str, value: &T) -> Result<()> {
        // let value = T::serialize(value).map_err(Error::SerdeError)?; -- for serde-lite
        let value = serde_json::to_string(&value).map_err(Error::SerdeJsonError)?;
        self.storage()
            .set_item(&key, &value)
            .map_err(Error::InsertError)
    }
}
