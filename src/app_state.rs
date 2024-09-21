use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub ip_reader: Arc<maxminddb::Reader<Vec<u8>>>,
}
