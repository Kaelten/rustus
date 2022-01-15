use actix_web::{web, HttpRequest, HttpResponse, Responder};

use tokio::sync::mpsc::unbounded_channel;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::errors::RustusError;
use crate::State;

/// Retrieve actual file.
///
/// This method allows you to download files directly from storage.
pub async fn get_file(request: HttpRequest, state: web::Data<State>) -> impl Responder {
    let file_id_opt = request.match_info().get("file_id").map(String::from);
    if let Some(file_id) = file_id_opt {
        let file_info = state.info_storage.get_info(file_id.as_str()).await?;
        if file_info.storage != state.data_storage.to_string() {
            return Err(RustusError::FileNotFound);
        }
        let (tx, rx_body) = unbounded_channel();
        tokio::spawn(async move {
            state
                .data_storage
                .get_contents(file_info.clone(), tx.clone())
                .await
        });
        Ok(HttpResponse::Ok().streaming(UnboundedReceiverStream::new(rx_body)))
    } else {
        Err(RustusError::FileNotFound)
    }
}
