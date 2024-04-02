use std::collections::HashSet;

use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use jsonwebtoken::errors::Result;

use super::utils::get_current_time;




#[derive(Serialize, Deserialize)]
pub struct SessionToken {
    exp : i64,
    iat: i64,
    id : String
}

impl SessionToken {
    pub fn encode(&self, secret : String) -> String {
        jsonwebtoken::encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(secret.to_owned().as_bytes()),
        )
        .unwrap()
    }

    pub fn decode(token: String, secret : String) ->  Option<SessionToken> {
        match jsonwebtoken::decode::<SessionToken>(
            &token,
            &DecodingKey::from_secret(secret.to_owned().as_bytes()),
            &Validation::default()
        ).ok() {
            Some(token_data) => Some(token_data.claims),
            None => None
        }
    }   
    pub fn new(id:String, exp:i64) -> Self {
        SessionToken {
            id,
            exp,
            iat : get_current_time().timestamp()
        }
    }
    
}