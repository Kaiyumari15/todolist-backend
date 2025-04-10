use std::sync::LazyLock;

use chrono::Duration;
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::request::FromRequest;
use serde::{Deserialize, Serialize};

/// Contains the public key used to verify the JWT token
static PUBLIC_KEY: LazyLock<DecodingKey> = LazyLock::new(|| {
    // Load the public key from the file
    let key = include_str!("../keys/jwt/public.pem");
    DecodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});

/// Contains the private key used to sign the JWT token
static PRIVATE_KEY: LazyLock<EncodingKey> = LazyLock::new(|| {
    // Load the private key from the file
    let key = include_str!("../keys/jwt/private.pem");
    EncodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});

#[derive(Debug, Serialize, Deserialize)]
/// The claims that will be included in the JWT token
/// 
/// # Fields
/// * `sub` - The subject of the token, usually the user ID
/// * `exp` - The expiration time of the token, in seconds since the epoch
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

/// Generate a JWT token for a user
/// 
/// # Arguments
/// * `user_id` - The ID of the user for whom the token is generated.
/// * `duration` - The duration for which the token is valid.
/// 
/// # Returns
/// * `String` - The generated JWT token.
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
    let token = encode(&Header::new(Algorithm::RS512), &claims, &*PRIVATE_KEY)
        .expect("Failed to encode token");

    token
}

/// Verify a JWT token and extract the claims
/// 
/// # Arguments
/// * `token` - The JWT token to be verified.
/// 
/// # Returns
/// * `Result<Claims, VerifyJWTError>` - The claims extracted from the token if verification is successful, or an error if verification fails.
pub async fn verify_token(token: &str) -> Result<Claims, VerifyJWTError> {
    
    // Set up the validation
    let mut validation = Validation::new(Algorithm::RS512);
    validation.validate_exp = true; // Validate expiration
    validation.leeway = 0; // No leeway for expiration // used for accurate testing

    // Decode the token using the public key
    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &*PUBLIC_KEY,
        &validation
    )
    .map_err(|error: jsonwebtoken::errors::Error| {
        match error.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken => VerifyJWTError::Malformed,
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => VerifyJWTError::Expired,
            _ => VerifyJWTError::Other(error.to_string()),
        }
    })?;

    // Return the claims
    Ok(token_data.claims)
}

#[derive(Serialize, Deserialize, Debug)]
/// The JWT struct which will be used to extract the token from the request
/// 
/// # Fields
/// * `token` - The JWT token extracted from the request
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
/// 
/// # Variants
/// * `Malformed` - The token is malformed and cannot be decoded
/// * `Expired` - The token has expired and is no longer valid
/// * `Other` - Any other error that may occur during verification
pub enum VerifyJWTError {
    Malformed,
    Expired,
    Other(String),
}