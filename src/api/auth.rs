use std::sync::LazyLock;

use chrono::Duration;
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header};
use rocket::futures::future::Lazy;
use serde::{Deserialize, Serialize};

static PUBLIC_KEY: LazyLock<DecodingKey> = LazyLock::new(|| {
    // Load the public key from the file
    let key = include_str!("../keys/jwt/public.pem");
    DecodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});
static PRIVATE_KEY: LazyLock<EncodingKey> = LazyLock::new(|| {
    // Load the private key from the file
    let key = include_str!("../keys/jwt/private.pem");
    EncodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub async fn generate_token(user_id: &str, duration: Duration) -> String {
    // Generate a JWT token with the claims
    let claims = Claims {
        sub: user_id.to_string(),
        exp: chrono::Utc::now()
            .checked_add_signed(duration)
            .expect("valid timestamp")
            .timestamp() as usize,
    };

    // Encode the token using the secret key
    let token = encode(&Header::new(Algorithm::HS512), &claims, &*PRIVATE_KEY)
        .expect("Failed to encode token");

    token
}

pub async fn verify_token(token: &str) -> String {
    // Decode the token using the public key
    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &*PUBLIC_KEY,
        &jsonwebtoken::Validation::new(Algorithm::HS512),
    )
    .expect("Failed to decode token"); // This will need to be changed to handle errors proper;y

    // Return the user ID from the claims
    token_data.claims.sub
}