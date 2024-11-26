pub struct SystemProtocolHandler {
    protocols: SystemProtocols,
    data_manager: Arc<SystemDataManager>,
    ml_manager: Arc<Mutex<MLManager>>,
}

impl SystemProtocolHandler {
    pub async

