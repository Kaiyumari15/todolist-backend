use std::sync::LazyLock;

use chrono::Duration;
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header};
use rocket::{futures::future::Lazy, request::FromRequest};
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

pub async fn verify_token(token: &str) -> Result<Claims, VerifyJWTError> {
    // Decode the token using the public key
    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &*PUBLIC_KEY,
        &jsonwebtoken::Validation::new(Algorithm::HS512),
    )
    .map_err(|error| {
        VerifyJWTError::Other(error.to_string())
    })?;

    // Return the claims
    Ok(token_data.claims)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JWT {
    pub token: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = std::io::Error;
    
    async fn from_request(request: &'r rocket::request::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        // Check if the Authorization header is present
        match request.headers().get_one("Authorization") {
            Some(token) => {
                let token = token[7..].to_string(); 
                rocket::request::Outcome::Success(JWT { token })
            }
            _ => {
                rocket::request::Outcome::Error(
                    (rocket::http::Status::Unauthorized, std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Missing or invalid Authorization header")),
                )
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
/// The error type which will be returned when verifying a JWT token
pub enum VerifyJWTError {
    Malformed,
    Expired,
    Other(String),
}