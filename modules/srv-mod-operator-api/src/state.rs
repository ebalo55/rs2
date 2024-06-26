use std::sync::Arc;

use srv_mod_config::SharedConfig;
use srv_mod_database::Pool;

pub type ApiServerSharedState = Arc<ApiServerState>;

/// The shared state for the API server
#[derive(Debug, Clone)]
pub struct ApiServerState {
	/// The shared configuration
	pub config: SharedConfig,
	/// The database connection pool
	pub db_pool: Pool,
}
