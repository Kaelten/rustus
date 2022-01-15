use crate::errors::RustusResult;
use crate::info_storages::FileInfo;

use actix_web::web::Bytes;

use async_trait::async_trait;
use std::fmt::Display;

use tokio::sync::mpsc::UnboundedSender;

#[async_trait]
pub trait Storage: Display {
    /// Prepare storage before starting up server.
    ///
    /// Function to check if configuration is correct
    /// and prepare storage E.G. create connection pool,
    /// or directory for files.
    ///
    /// It MUST throw errors if connection can't
    /// be established or in any other case that might
    /// be a problem later on.
    async fn prepare(&mut self) -> RustusResult<()>;

    /// Get contents of a file.
    ///
    /// This method must return NamedFile since it
    /// is compatible with ActixWeb files interface.
    /// FIXME: change return type to stream.
    ///
    /// # Params
    /// `file_info` - info about current file.
    async fn get_contents(
        &self,
        file_info: FileInfo,
        sender: UnboundedSender<RustusResult<Bytes>>,
    ) -> RustusResult<()>;

    /// Add bytes to the file.
    ///
    /// This method is used to append bytes to some file.
    /// It returns new offset.
    ///
    /// # Errors
    ///
    /// Implementations MUST throw errors at following cases:
    /// * If the info about the file can't be found.
    /// * If the storage is offline.
    ///
    /// # Params
    /// `file_info` - info about current file.
    /// `bytes` - bytes to append to the file.
    async fn add_bytes(&self, file_info: &FileInfo, bytes: &[u8]) -> RustusResult<()>;

    /// Create file in storage.
    ///
    /// This method is used to generate unique file id, create file and store information about it.
    ///
    /// This function must use info storage to store information about the upload.
    ///
    /// # Params
    /// `file_info` - info about current file.
    async fn create_file(&self, file_info: &FileInfo) -> RustusResult<String>;

    /// Remove file from storage
    ///
    /// This method removes file and all associated
    /// object if any.
    ///
    /// # Params
    /// `file_info` - info about current file.
    async fn remove_file(&self, file_info: &FileInfo) -> RustusResult<()>;
}
