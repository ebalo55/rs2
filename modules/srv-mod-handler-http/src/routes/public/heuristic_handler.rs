use axum::body::{Body, Bytes};
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Router;
use axum::routing::post;
use tracing::instrument;

use rs2_crypt::encoder::base32::Base32Encoder;
use rs2_crypt::encoder::base64::Base64Encoder;
use rs2_crypt::encoder::Encoder as CryptEncoder;
use rs2_crypt::encoder::hex::HexEncoder;
use srv_mod_config::handlers::{Encoder, EncryptionScheme};
use srv_mod_handler_base::state::{HandlerSharedState, HttpHandlerSharedState};

/// The handler for the agent checking operation
#[instrument(skip(state, body, headers))]
async fn handle_request(
	State(state): HttpHandlerSharedState,
	headers: HeaderMap,
	body: Bytes,
	id: String,
) -> Response<Body> {
	match state.config.security.encryption_scheme {
		// handle plaintext communication no decryption needed
		EncryptionScheme::Plain => {
			match state.config.security.encoder.as_ref() {
				// no encoding needed, this is definitely plaintext
				None => {
					process_body::process_body(state, body).await
				}
				// handle all the encodings
				Some(encoding) => {
					let decoded = decode_or_fail_response(encoding.clone(), body);

					if decoded.is_err() {
						return decoded.unwrap_err();
					}

					let decoded = decoded.unwrap();
					process_body::process_body(state, decoded).await
				}
			}
		}
		// handle symmetric encryption, decryption needed with the symmetric key of the agent if available or fallback
		// to plaintext
		EncryptionScheme::Symmetric => {
			match state.config.security.encoder.as_ref() {
				// no encoding needed, jump straight to decryption
				None => {
					todo!("Try to decrypt the payload with the symmetric key if any or fallback to plain text");
					todo!("Process request body");

					(StatusCode::OK, "").into_response()
				}
				// handle all the encodings
				Some(encoding) => {
					match encoding {
						// decode hex, then start processing
						Encoder::Hex => {
							todo!("Decode from hex");
							todo!("Try to decrypt the payload with the symmetric key if any or fallback to plain text");
							todo!("Process request body");

							(StatusCode::OK, "").into_response()
						}
						// decode base32, then start processing
						Encoder::Base32 => {
							todo!("Decode from base32");
							todo!("Try to decrypt the payload with the symmetric key if any or fallback to plain text");
							todo!("Process request body");

							(StatusCode::OK, "").into_response()
						}
						// decode base64, then start processing
						Encoder::Base64 => {
							todo!("Decode from base64");
							todo!("Try to decrypt the payload with the symmetric key if any or fallback to plain text");
							todo!("Process request body");

							(StatusCode::OK, "").into_response()
						}
					}
				}
			}
		}
		// handle asymmetric encryption, decryption needed with the private key of the agent if available or fallback
		// to plaintext
		EncryptionScheme::Asymmetric => {
			match state.config.security.encoder.as_ref() {
				// no encoding needed, jump straight to decryption
				None => {
					todo!("Try to decrypt the payload with the shared key if any or fallback to plain text");
					todo!("Process request body");

					(StatusCode::OK, "").into_response()
				}
				// handle all the encodings
				Some(encoding) => {
					match encoding {
						// decode hex, then start processing
						Encoder::Hex => {
							todo!("Decode from hex");
							todo!("Try to decrypt the payload with the shared key if any or fallback to plain text");
							todo!("Process request body");

							(StatusCode::OK, "").into_response()
						}
						// decode base32, then start processing
						Encoder::Base32 => {
							todo!("Decode from base32");
							todo!("Try to decrypt the payload with the shared key if any or fallback to plain text");
							todo!("Process request body");

							(StatusCode::OK, "").into_response()
						}
						// decode base64, then start processing
						Encoder::Base64 => {
							todo!("Decode from base64");
							todo!("Try to decrypt the payload with the shared key if any or fallback to plain text");
							todo!("Process request body");

							(StatusCode::OK, "").into_response()
						}
					}
				}
			}
		}
	}
}

/// This kind of route uses the first path parameter as the index of the id in the path
///
/// # Example 1
/// Path: `/3/this/is/a/dd1g8uw209me6bin2unm9u38mhmp23ic/sample/path` <br/>
/// `-----|N|0---|1-|2|3-------------------------------|4-----|5----`
///
/// ### Explanation
/// - N: id_position, defines the position of the id in the path, 3 in the example, can be any number
/// - 0 to 2: decoy strings, unused
/// - 3: the actual id of the request to parse
/// - 4+: other decoy strings, unused
///
/// # Example 2
/// Path: `/2,3,5/this/is/dd1g8uw/209me6bin2unm/a/9u38mhmp23ic/sample/path` <br/>
/// `-----|N,N,N|0---|1-|2------|3------------|4|------------|5-----|6---`
///
/// WHERE "," can be any of the following:
/// - ","
/// - ";"
/// - ":"
/// - "."
/// - "-"
/// - "_"
/// - " "
/// - "|"
/// - "$"
///
/// ### Explanation
/// - N: id_position, defines the position of the id in the path, 2, 3, 5 in the example, can be any number,
///      with any of the allowed separators
/// - 0 and 1: decoy strings, unused
/// - 2 and 3: fragments of the id, to be concatenated
/// - 4: decoy string, unused
/// - 5: last fragment of the id, to be concatenated
/// - 6+: other decoy strings, unused
pub async fn heuristic_handler_variant_1(
	Path((id_position, path)): Path<(String, String)>,
	State(state): State<HandlerSharedState>,
	headers: HeaderMap,
	body: Bytes,
) -> Response<Body> {
	// Split the id_position string by allowed separators
	let separators = [',', ';', ':', '.', '-', '_', ' ', '|', '$'];
	let id_positions: Vec<usize> = id_position
		.split(|c| separators.contains(&c))
		.map(|s| s.parse::<usize>())
		.collect::<Result<_, _>>()
		.map_err(|_| StatusCode::BAD_REQUEST)?;

	// Split the path by '/'
	let parts: Vec<&str> = path.split('/').collect();

	// Concatenate the fragments of the ID, appending an empty string for undefined positions
	let id = id_positions
		.iter()
		.map(|&pos| parts.get(pos).unwrap_or(&""))
		.collect::<Vec<&str>>()
		.join("");

	handle_request(state, headers, body, id).await
}

/// This kind of route automatically takes the first string matching the ID length (32) as the request ID
///
/// # Example
/// Path: `/this/is/a/dd1g8uw209me6bin2unm9u38mhmp23ic/sample/path` <br/>
/// `-----|0---|1-|2|3-------------------------------|4-----|5---`
///
/// ### Explanation
/// - 0 to 2: decoy strings, unused
/// - 3: the actual id of the request to parse
/// - 4+: other decoy strings, unused
pub async fn heuristic_handler_variant_2(
	Path(path): Path<String>,
	State(state): State<HandlerSharedState>,
	headers: HeaderMap,
	body: Bytes,
) -> Response<Body> {
	// Extract the ID by finding the first 32-character segment
	let id = path
		.split('/')
		.find(|&part| part.len() == 32)
		.ok_or(StatusCode::BAD_REQUEST)?
		.to_string();

	handle_request(state, headers, body, id).await
}

/// Creates the routes for the commands handlers
pub fn route(state: HandlerSharedState) -> Router<HandlerSharedState> {
	Router::new()
		.route("/:id_position/*path", post(heuristic_handler_variant_1))
		.route("/*path", post(heuristic_handler_variant_2))
		.with_state(state)
}

#[cfg(test)]
mod tests {
	use std::sync::Arc;

	use axum::http::Request;
	use bytes::{BufMut, BytesMut};
	use serial_test::serial;
	use tokio;
	use tower::ServiceExt;

	use rs2_communication_protocol::communication_structs::checkin::{Checkin, PartialCheckin};
	use rs2_communication_protocol::magic_numbers;
	use srv_mod_config::handlers;
	use srv_mod_config::handlers::{HandlerConfig, HandlerSecurityConfig, HandlerType};
	use srv_mod_database::{bb8, Pool};
	use srv_mod_database::diesel::{Connection, PgConnection};
	use srv_mod_database::diesel_async::AsyncPgConnection;
	use srv_mod_database::diesel_async::pooled_connection::AsyncDieselConnectionManager;
	use srv_mod_database::diesel_migrations::MigrationHarness;
	use srv_mod_database::migration::MIGRATIONS;
	use srv_mod_handler_base::state::HttpHandlerState;

	use super::*;

	fn make_config() -> HandlerConfig {
		let config = HandlerConfig {
			enabled: true,
			r#type: HandlerType::Http,
			protocols: vec![
				handlers::Protocol::Json
			],
			port: 8081,
			host: "127.0.0.1".to_string(),
			tls: None,
			security: HandlerSecurityConfig {
				encryption_scheme: EncryptionScheme::Plain,
				algorithm: None,
				encoder: None,
			},
		};

		config
	}

	async fn drop_database(url: String) {
		let mut connection = PgConnection::establish(url.as_str()).unwrap();

		connection.revert_all_migrations(MIGRATIONS).unwrap();
		connection.run_pending_migrations(MIGRATIONS).unwrap();
	}

	async fn make_pool(url: String) -> Pool {
		let connection_manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);
		Arc::new(
			bb8::Pool::builder()
				.max_size(1u32)
				.build(connection_manager)
				.await
				.unwrap(),
		)
	}

	#[tokio::test]
	#[serial]
	async fn test_post_handler_plain_no_encoding() {
		let shared_config = make_config();
		let connection_string = "postgresql://rs2:rs2@localhost/rs2".to_string();

		// Ensure the database is clean
		drop_database(connection_string.clone()).await;
		let pool = make_pool(connection_string.clone()).await;

		let route_state = Arc::new(HttpHandlerState {
			config: Arc::new(shared_config),
			db_pool: pool,
		});
		// init the app router
		let app = route(route_state.clone());

		let obj_checkin = Checkin::new(PartialCheckin {
			operative_system: "Windows".to_string(),
			hostname: "DESKTOP-PC".to_string(),
			domain: "WORKGROUP".to_string(),
			username: "user".to_string(),
			ip: "10.2.123.45".to_string(),
			process_id: 1234,
			parent_process_id: 5678,
			process_name: "agent.exe".to_string(),
			elevated: true,
		});
		let checkin = serde_json::to_string(&obj_checkin).unwrap();

		let mut bytes = BytesMut::with_capacity(checkin.len() + magic_numbers::JSON.len());
		for b in magic_numbers::JSON.iter() {
			bytes.put_u8(*b);
		}
		for b in checkin.as_bytes() {
			bytes.put_u8(*b);
		}
		let request = Request::post("/checkin")
			.header("Content-Type", "text/plain")
			.body(Body::from(axum::body::Bytes::from(bytes.to_vec())))
			.unwrap();

		// send the request
		let response = app
			.with_state(route_state.clone())
			.oneshot(request)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::OK);

		let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
		assert_eq!(body.is_empty(), false);
	}

	#[tokio::test]
	#[serial]
	async fn test_post_handler_plain_hex() {
		let mut shared_config = make_config();
		shared_config.security.encoder = Some(Encoder::Hex);
		let connection_string = "postgresql://rs2:rs2@localhost/rs2".to_string();

		// Ensure the database is clean
		drop_database(connection_string.clone()).await;
		let pool = make_pool(connection_string.clone()).await;

		let route_state = Arc::new(HttpHandlerState {
			config: Arc::new(shared_config),
			db_pool: pool,
		});
		// init the app router
		let app = route(route_state.clone());

		let obj_checkin = Checkin::new(PartialCheckin {
			operative_system: "Windows".to_string(),
			hostname: "DESKTOP-PC".to_string(),
			domain: "WORKGROUP".to_string(),
			username: "user".to_string(),
			ip: "10.2.123.45".to_string(),
			process_id: 1234,
			parent_process_id: 5678,
			process_name: "agent.exe".to_string(),
			elevated: true,
		});
		let checkin = serde_json::to_string(&obj_checkin).unwrap();

		let mut bytes = BytesMut::with_capacity(checkin.len() + magic_numbers::JSON.len());
		for b in magic_numbers::JSON.iter() {
			bytes.put_u8(*b);
		}
		for b in checkin.as_bytes() {
			bytes.put_u8(*b);
		}
		let body = HexEncoder::default().encode(bytes.freeze());

		let request = Request::post("/checkin")
			.header("Content-Type", "text/plain")
			.body(Body::from(body))
			.unwrap();

		// send the request
		let response = app
			.with_state(route_state.clone())
			.oneshot(request)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::OK);

		let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
		assert_eq!(body.is_empty(), false);
	}

	#[tokio::test]
	#[serial]
	async fn test_post_handler_plain_base32() {
		let mut shared_config = make_config();
		shared_config.security.encoder = Some(Encoder::Base32);
		let connection_string = "postgresql://rs2:rs2@localhost/rs2".to_string();

		// Ensure the database is clean
		drop_database(connection_string.clone()).await;
		let pool = make_pool(connection_string.clone()).await;

		let route_state = Arc::new(HttpHandlerState {
			config: Arc::new(shared_config),
			db_pool: pool,
		});
		// init the app router
		let app = route(route_state.clone());

		let obj_checkin = Checkin::new(PartialCheckin {
			operative_system: "Windows".to_string(),
			hostname: "DESKTOP-PC".to_string(),
			domain: "WORKGROUP".to_string(),
			username: "user".to_string(),
			ip: "10.2.123.45".to_string(),
			process_id: 1234,
			parent_process_id: 5678,
			process_name: "agent.exe".to_string(),
			elevated: true,
		});
		let checkin = serde_json::to_string(&obj_checkin).unwrap();

		let mut bytes = BytesMut::with_capacity(checkin.len() + magic_numbers::JSON.len());
		for b in magic_numbers::JSON.iter() {
			bytes.put_u8(*b);
		}
		for b in checkin.as_bytes() {
			bytes.put_u8(*b);
		}
		let body = Base32Encoder::default().encode(bytes.freeze());

		let request = Request::post("/checkin")
			.header("Content-Type", "text/plain")
			.body(Body::from(body))
			.unwrap();

		// send the request
		let response = app
			.with_state(route_state.clone())
			.oneshot(request)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::OK);

		let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
		assert_eq!(body.is_empty(), false);
	}

	#[tokio::test]
	#[serial]
	async fn test_post_handler_plain_base64() {
		let mut shared_config = make_config();
		shared_config.security.encoder = Some(Encoder::Base64);
		let connection_string = "postgresql://rs2:rs2@localhost/rs2".to_string();

		// Ensure the database is clean
		drop_database(connection_string.clone()).await;
		let pool = make_pool(connection_string.clone()).await;

		let route_state = Arc::new(HttpHandlerState {
			config: Arc::new(shared_config),
			db_pool: pool,
		});
		// init the app router
		let app = route(route_state.clone());

		let obj_checkin = Checkin::new(PartialCheckin {
			operative_system: "Windows".to_string(),
			hostname: "DESKTOP-PC".to_string(),
			domain: "WORKGROUP".to_string(),
			username: "user".to_string(),
			ip: "10.2.123.45".to_string(),
			process_id: 1234,
			parent_process_id: 5678,
			process_name: "agent.exe".to_string(),
			elevated: true,
		});
		let checkin = serde_json::to_string(&obj_checkin).unwrap();

		let mut bytes = BytesMut::with_capacity(checkin.len() + magic_numbers::JSON.len());
		for b in magic_numbers::JSON.iter() {
			bytes.put_u8(*b);
		}
		for b in checkin.as_bytes() {
			bytes.put_u8(*b);
		}
		let body = Base64Encoder::default().encode(bytes.freeze());

		let request = Request::post("/checkin")
			.header("Content-Type", "text/plain")
			.body(Body::from(body))
			.unwrap();

		// send the request
		let response = app
			.with_state(route_state.clone())
			.oneshot(request)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::OK);

		let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
		assert_eq!(body.is_empty(), false);
	}
}