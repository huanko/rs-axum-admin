use anyhow::{Ok, Result};
use chrono::{Duration, Utc};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::fmt;
use std::fmt::Display;

use crate::config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    i: i64,
    t: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cliams {
    pub exp: usize,
    pub iat: usize,
    pub id: i64,
}

impl Identity {
    pub fn new(id: i64,token: String) -> Self {
        Self {
            i: id,
            t: token,
        }
    }

    pub fn empty() -> Self {
        Self {
            i: 0,
            t: String::from(""),
        }
    }

    pub fn from_auth_token(token: String) -> Self {
        // decode 解码  Validation
        let secret = config::global().get_string("app.secret").unwrap();
        let token_data: jsonwebtoken::TokenData<Cliams> = 
            decode::<Cliams>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()).unwrap();
        let identity = Identity {
            i: token_data.claims.id,
            t: token,
        };
        identity
    }

    pub fn to_auth_token(&self) -> Result<String> {
        // encode 编码
        //let now = xtime::now(offset!(+8)).unix_timestamp();
        let now = Utc::now();
        let expire = Duration::hours(24);
        let exp = (now + expire).timestamp() as usize;
        let iat = now.timestamp() as usize;

        let claim = Cliams { iat, exp, id: self.i };

        let secret = config::global().get_string("app.secret").unwrap();
        let token = 
            encode(&Header::default(), 
                    &claim, 
                    &EncodingKey::from_secret(secret.as_ref()))?;
        Ok(token)
    }

    
    pub fn id(&self) -> i64 {
        self.i
    }

    pub fn match_token(&self, token: String) -> bool {
        self.t == token
    }
}


impl Display for Identity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.i == 0 {
            return write!(f, "<none>");
        }
       
        write!(f, "id:{}|token:{}", self.i, self.t)
    }
}