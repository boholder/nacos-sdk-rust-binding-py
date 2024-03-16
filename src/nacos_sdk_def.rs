use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use nacos_sdk::api::naming::NamingService;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(remote = "NamingService")]
pub(crate) struct NamingServiceDef {
    nacos_grpc_client: Arc<NacosGrpcClientDef>,
    namespace: String,
    // redo_task_executor: Arc<RedoTaskExecutorDef>,
    // service_info_updater: ServiceInfoUpdater,
    client_id: String,
    // naming_cache: Arc<Cache<ServiceInfo>>,
    // observer: ServiceInfoObserver,
}

struct NacosGrpcClientDef {
    app_name: String,
    send_request: Arc<dyn SendRequest + Send + Sync + 'static>,
}

// pub(crate) struct RedoTaskExecutorDef {
//     map: Arc<RwLock<HashMap<String, Arc<dyn RedoTask>>>>,
//     id: String,
// }

