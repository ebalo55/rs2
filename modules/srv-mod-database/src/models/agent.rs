use diesel::{AsChangeset, Insertable, Queryable, Selectable};

use rs2_communication_protocol::communication_structs::checkin::Checkin;

use crate::CUID2;
use crate::schema_extension::AgentFields;

#[derive(Debug, Queryable, Selectable, Clone, PartialEq)]
#[diesel(table_name = crate::schema::agents)]
pub struct Agent {
	/// The unique identifier for the agent (cuid2)
	pub id: String,
	/// The OS name
	pub operative_system: String,
	/// The victim hostname
	pub hostname: String,
	/// The domain of the victim
	pub domain: String,
	/// The username of whose runs the agent
	pub username: String,
	/// The internal IP of the victim
	pub ip: String,
	/// The process ID of the agent
	pub process_id: i64,
	/// The parent process ID of the agent
	pub parent_process_id: i64,
	/// The process name of the agent
	pub process_name: String,
	/// Whether the agent is running as elevated
	pub elevated: bool,
	/// The secret key of the server when communicating with the agent
	pub server_secret_key: String,
	/// The secret key of the agent
	pub secret_key: String,
	/// The agent's data signature, used to verify the agent's and avoid duplicates
	pub signature: String,
	/// The agent's creation date
	pub created_at: chrono::DateTime<chrono::Utc>,
	/// The agent's last update date
	pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Agent {
	pub fn get_field_value(&self, field: &AgentFields) -> Option<String> {
		match field {
			AgentFields::OperativeSystem => Some(self.operative_system.clone()),
			AgentFields::Hostname => Some(self.hostname.clone()),
			AgentFields::Domain => Some(self.domain.clone()),
			AgentFields::Username => Some(self.username.clone()),
			AgentFields::Ip => Some(self.ip.clone()),
			AgentFields::ProcessId => Some(self.process_id.to_string()),
			AgentFields::ParentProcessId => Some(self.parent_process_id.to_string()),
			AgentFields::ProcessName => Some(self.process_name.clone()),
			AgentFields::Elevated => Some(self.elevated.to_string()),
			AgentFields::ServerSecretKey => Some(self.server_secret_key.clone()),
			AgentFields::SecretKey => Some(self.secret_key.clone()),
			AgentFields::Signature => Some(self.signature.clone()),
		}
	}
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::agents)]
pub struct CreateAgent {
	/// The unique identifier for the agent (cuid2)
	pub id: String,
	/// The OS name
	pub operative_system: String,
	/// The victim hostname
	pub hostname: String,
	/// The domain of the victim
	pub domain: String,
	/// The username of whose runs the agent
	pub username: String,
	/// The internal IP of the victim
	pub ip: String,
	/// The process ID of the agent
	pub process_id: i64,
	/// The parent process ID of the agent
	pub parent_process_id: i64,
	/// The process name of the agent
	pub process_name: String,
	/// Whether the agent is running as elevated
	pub elevated: bool,
	/// The secret key of the server when communicating with the agent
	pub server_secret_key: String,
	/// The secret key of the agent
	pub secret_key: String,
	/// The agent's data signature, used to verify the agent's and avoid duplicates
	pub signature: String,
}

impl From<Checkin> for CreateAgent {
	fn from(checkin: Checkin) -> Self {
		Self {
			id: CUID2.create_id(),
			operative_system: checkin.operative_system,
			hostname: checkin.hostname,
			domain: checkin.domain,
			username: checkin.username,
			ip: checkin.ip,
			process_id: checkin.process_id,
			parent_process_id: checkin.parent_process_id,
			process_name: checkin.process_name,
			elevated: checkin.elevated,
			server_secret_key: "".to_string(),
			secret_key: "".to_string(),
			signature: "".to_string(),
		}
	}
}