use base64::Engine;
use base64::engine::general_purpose::STANDARD as b64_encode;
use log::info;
use rand::{Rng, thread_rng};

/// Generate a random secret for JWT signing
fn generate_jwt_secret() -> String {
	// Generate random bytes
	let mut rng = thread_rng();
	let mut secret_bytes = [0u8; 64];
	rng.fill(&mut secret_bytes);

	// Encode random bytes as base64 string
	b64_encode.encode(&secret_bytes)
}

pub fn generate_jwt() -> anyhow::Result<()> {
	let secret = generate_jwt_secret();
	info!("JWT secret successfully generated: {}", secret);
	Ok(())
}
